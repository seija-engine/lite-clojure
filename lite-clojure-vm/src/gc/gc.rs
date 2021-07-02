use std::any::Any;
use std::marker::PhantomData;
use std::mem;
use std::ptr;
use std::ptr::NonNull;
use std::{
    cell::Cell,
    ops::{Deref, DerefMut},
};

use crate::Error;

pub unsafe trait Trace {
    unsafe fn root(&mut self) {}
    unsafe fn unroot(&mut self) {}

    fn trace(&self, gc: &mut GC) {
        let _ = gc;
    }
}

pub struct Move<T>(pub T);

unsafe impl<T> DataDef for Move<T>  {
    type Value = T;
    fn size(&self) -> usize {
        mem::size_of::<T>()
    }
    fn initialize(self, result: WriteOnly<T>) -> &mut T {
        result.write(self.0)
    }
}

pub trait CloneUnrooted {
    type Value;
    unsafe fn clone_unrooted(&self) -> Self::Value;
}

impl<T: ?Sized + CloneUnrooted> CloneUnrooted for &'_ T {
    type Value = T::Value;
    #[inline]
    unsafe fn clone_unrooted(&self) -> Self::Value {
        (**self).clone_unrooted()
    }
}

pub unsafe trait CopyUnrooted: CloneUnrooted<Value = Self> + Sized {
    unsafe fn copy_unrooted(&self) -> Self {
        ptr::read(self)
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Generation(i32);

impl Generation {
    pub fn next(self) -> Generation {
        assert!(self.0 < i32::max_value(),"To many generations has been created");
        Generation(self.0 + 1)
    }
}

#[derive(Debug)]
pub struct GcPtr<T: ?Sized>(NonNull<T>);

impl<T: ?Sized> GcPtr<T> {
    pub unsafe fn from_raw(ptr: *const T) -> GcPtr<T> {
        GcPtr(NonNull::new_unchecked(ptr as *mut _))
    }

    pub fn as_lifetime(&self) -> &() {
        &()
    }
}

impl<T: ?Sized + Eq> Eq for GcPtr<T> {}
impl<T: ?Sized + PartialEq> PartialEq for GcPtr<T> {
    fn eq(&self, other: &GcPtr<T>) -> bool {
        **self == **other
    }
}

unsafe impl<T: ?Sized> CopyUnrooted for GcPtr<T> {}
impl<T: ?Sized> CloneUnrooted for GcPtr<T> {
    type Value = Self;
    #[inline]
    unsafe fn clone_unrooted(&self) -> Self {
        GcPtr(self.0)
    }
}

impl<T: ?Sized> ::std::borrow::Borrow<T> for GcPtr<T> {
    fn borrow(&self) -> &T {
        &**self
    }
}

impl<T: ?Sized> Deref for GcPtr<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { self.0.as_ref() }
    }
}
pub struct OwnedPtr<T: ?Sized>(NonNull<T>);

impl<T: ?Sized> Deref for OwnedPtr<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { self.0.as_ref() }
    }
}
#[derive(Debug, Eq, PartialEq)]
pub struct Borrow<'a, T>(T, PhantomData<&'a T>);

impl<T> CloneUnrooted for Borrow<'_, T> where T: CloneUnrooted {
    type Value = T::Value;
    #[inline]
    unsafe fn clone_unrooted(&self) -> Self::Value {
        self.0.clone_unrooted()
    }
}


pub type GcRef<'a, T> = Borrow<'a, GcPtr<T>>;
pub type OwnedGcRef<'a, T> = Borrow<'a, OwnedPtr<T>>;

impl<'gc, T: ?Sized> From<OwnedGcRef<'gc, T>> for GcRef<'gc, T> {
    fn from(ptr: OwnedGcRef<'gc, T>) -> Self {
        Borrow(ptr.0.into(), PhantomData)
    }
}

impl<T: ?Sized> From<OwnedPtr<T>> for GcPtr<T> {
    fn from(ptr: OwnedPtr<T>) -> GcPtr<T> {
        GcPtr(ptr.0)
    }
}

impl<T> Deref for Borrow<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for Borrow<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<'gc, T> Borrow<'gc, T> {
    pub fn new(value: &'gc T) -> Borrow<'gc, T::Value> where T: CloneUnrooted {
        unsafe { Borrow::with_root(value.clone_unrooted(), value) }
    }
    #[inline]
    pub(crate) unsafe fn with_root<U: ?Sized>(value: T, _root: &'gc U) -> Self {
        Borrow(value, PhantomData)
    }

    pub unsafe fn unrooted(self) -> T {
        self.0
    }

    pub fn as_lifetime(&self) -> &'gc () {
        &()
    }
}

//这个WriteOnly用来确保不会对未初始化的内存做读取操作
pub struct WriteOnly<'s, T: ?Sized + 's>(*mut T, PhantomData<&'s mut T>);

impl<'s, T: ?Sized> WriteOnly<'s, T> {
    pub unsafe fn new(t: *mut T) -> WriteOnly<'s, T> {
        WriteOnly(t, PhantomData)
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.0
    }
}

impl<'s, T> WriteOnly<'s, T> {
    /// Writes a value to the pointer and returns a pointer to the now initialized value.
    pub fn write(self, t: T) -> &'s mut T {
        unsafe {
            ptr::write(self.0, t);
            &mut *self.0
        }
    }
}

pub unsafe trait DataDef {
    type Value: ?Sized + for<'a> FromPtr<&'a Self>;

    fn size(&self) -> usize;

    fn initialize<'w>(self, ptr: WriteOnly<'w, Self::Value>) -> &'w mut Self::Value;
}
#[derive(Debug)]
struct GcHeader {
    next: Option<AllocPtr>,
    marked: Cell<bool>,
    value_size: usize,
}

impl GcHeader {
    fn value_offset() -> usize {
        let hs = mem::size_of::<GcHeader>();
        let max_align = mem::align_of::<f64>();
        hs + ((max_align - (hs % max_align)) % max_align)
    }

    fn value(&mut self) -> *mut () {
        unsafe {
            let ptr: *mut GcHeader = self;
            (ptr as *mut u8).offset(GcHeader::value_offset() as isize) as *mut ()
        }
    }
}

pub unsafe trait FromPtr<D> {
    unsafe fn make_ptr(data: D, ptr: *mut ()) -> *mut Self;
}

unsafe impl<D, T> FromPtr<D> for T {
    unsafe fn make_ptr(_: D, ptr: *mut ()) -> *mut Self {
        ptr as *mut Self
    }
}

#[derive(Debug)]
struct AllocPtr {
    ptr: *mut GcHeader,
}

impl Deref for AllocPtr {
    type Target = GcHeader;
    fn deref(&self) -> &GcHeader {
        unsafe { &*self.ptr }
    }
}

impl DerefMut for AllocPtr {
    fn deref_mut(&mut self) -> &mut GcHeader {
        unsafe { &mut *self.ptr }
    }
}

impl AllocPtr {
    fn new<T>(value_size: usize) -> AllocPtr {
        let alloc_size = GcHeader::value_offset() + value_size;
        unsafe {
            let ptr = allocate(alloc_size) as *mut GcHeader;
            ptr::write(
                ptr,
                GcHeader {
                    next: None,
                    value_size: value_size,
                    marked: Cell::new(false),
                },
            );
            AllocPtr { ptr }
        }
    }

    fn size(&self) -> usize {
        GcHeader::value_offset() + self.value_size
    }
}

pub struct GC {
    values: Option<AllocPtr>,
    memory_limit: usize,
    generation: Generation,
    allocated_memory: usize,
}

impl GC {
    pub fn new(generation: Generation, memory_limit: usize) -> GC {
        GC {
            values: None,
            generation,
            memory_limit,
            allocated_memory: 0,
        }
    }

    pub fn alloc<D>(&mut self, def: D) -> GcRef<D::Value>
    where
        D: DataDef,
        D::Value: Sized + Any,
    {
        GcRef::from(self.alloc_owned(def))
    }

    pub fn alloc_owned<D>(&mut self, def: D) -> OwnedGcRef<D::Value>
    where
        D: DataDef,
        D::Value: Sized,
    {
        let size = def.size();
        let needed = self.allocated_memory.saturating_add(size);
        if needed >= self.memory_limit {
            panic!("out of memory");
        }
        self.alloc_ignore_limit_(size, def)
    }

    fn alloc_ignore_limit_<D>(&mut self, size: usize, def: D) -> OwnedGcRef<D::Value> where D: DataDef,D::Value: Sized {
        let mut ptr = AllocPtr::new::<D::Value>(size);
        ptr.next = self.values.take();
        self.allocated_memory += ptr.size();
        unsafe {
            let p: *mut D::Value = D::Value::make_ptr(&def, ptr.value());
            let ret: *const D::Value = &*def.initialize(WriteOnly::new(p));
            assert!(ret == p);
            self.values = Some(ptr);
            let mut ptr = OwnedPtr(NonNull::new_unchecked(p));
            //TODO Trace::unroot
            OwnedGcRef::with_root(ptr, self)
        }
    }

    pub fn new_child_gc(&self) -> GC {
        GC::new(self.generation.next(), self.memory_limit)
    }

    pub unsafe fn alloc_and_collect<R, D>(&mut self,roots: R,def: D) -> Result<OwnedGcRef<D::Value>,Error>
    where
        D: DataDef,
        D::Value: Sized + Any {

        //TODO 
        Ok(self.alloc_owned(def))
    }
    
}

#[inline]
unsafe fn allocate(size: usize) -> *mut u8 {
    let cap = size / mem::size_of::<f64>()
        + (if size % mem::size_of::<f64>() != 0 {
            1
        } else {
            0
        });
    ptr_from_vec(Vec::<f64>::with_capacity(cap))
}

#[inline]
fn ptr_from_vec(mut buf: Vec<f64>) -> *mut u8 {
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    ptr as *mut u8
}

#[macro_export]
macro_rules! construct_gc {
    (impl $typ: ident [$($acc: tt)*] [$($ptr: ident)*] @ $field: ident : $expr: expr, $($rest: tt)*) => { {
        let $field = $expr;
        $crate::construct_gc!(impl $typ
                      [$($acc)* $field: unsafe { $crate::gc::gc::CloneUnrooted::clone_unrooted(&$field) },]
                      [$($ptr)* $field]
                      $($rest)*
        )
    } };

    (impl $typ: ident [$($acc: tt)*] [$($ptr: ident)*] @ $field: ident, $($rest: tt)*) => {
        $crate::construct_gc!(impl $typ
                      [$($acc)* $field: unsafe { $crate::gc::CloneUnrooted::clone_unrooted(&$field) },]
                      [$($ptr)* $field]
                      $($rest)*
        )
    };

    (impl $typ: ident [$($acc: tt)*] [$($ptr: ident)*] $field: ident $(: $expr: expr)?, $($rest: tt)*) => {
        $crate::construct_gc!(impl $typ
                      [$($acc)* $field $(: $expr)?,]
                      [$($ptr)*]
                      $($rest)*
        )
    };

    (impl $typ: ident [$($acc: tt)*] [$($ptr: ident)*] ) => { {
        let root = [$( $ptr.as_lifetime() )*].first().map_or(&(), |v| *v);
        #[allow(unused_unsafe)]
        let v = $typ { $($acc)* };
        #[allow(unused_unsafe)]
        unsafe { $crate::gc::gc::Borrow::with_root(v, root) }
    } };

    ($typ: ident { $( $tt: tt )* } ) => {
        $crate::construct_gc!{impl $typ [] [] $( $tt )* }
    };

    ($typ: ident $(:: $variant: ident)? ( $( $tt: tt )* ) ) => {
        $crate::construct_enum_gc!{impl $typ $(:: $variant)? [] [] $( $tt )* }
    };
}

#[macro_export]
macro_rules! construct_enum_gc {
    (impl $typ: ident $(:: $variant: ident)? [$($acc: tt)*] [$($ptr: ident)*] @ $expr: expr, $($rest: tt)*) => { {
        let ref ptr = $expr;
        $crate::construct_enum_gc!(impl $typ $(:: $variant)?
                      [$($acc)* unsafe { $crate::gc::CloneUnrooted::clone_unrooted(ptr) },]
                      [$($ptr)* ptr]
                      $($rest)*
        )
    } };

    (impl $typ: ident $(:: $variant: ident)? [$($acc: tt)*] [$($ptr: ident)*] $expr: expr, $($rest: tt)*) => {
        $crate::construct_enum_gc!(impl $typ $(:: $variant)?
                      [$($acc)* $expr,]
                      [$($ptr)*]
                      $($rest)*
        )
    };

    (impl $typ: ident $(:: $variant: ident)? [$($acc: tt)*] [$($ptr: ident)*] @ $expr: expr) => { {
        let ref ptr = $expr;
        $crate::construct_enum_gc!(impl $typ $(:: $variant)?
                      [$($acc)* unsafe { $crate::gc::CloneUnrooted::clone_unrooted(ptr) },]
                      [$($ptr)* ptr]
        )
    } };

    (impl $typ: ident $(:: $variant: ident)? [$($acc: tt)*] [$($ptr: ident)*] $expr: expr) => {
        $crate::construct_enum_gc!(impl $typ $(:: $variant)?
                      [$($acc)* $expr,]
                      [$($ptr)*]
        )
    };

    (impl $typ: ident $(:: $variant: ident)? [$($acc: tt)*] [$($ptr: ident)*] ) => { {
        let root = [$( $ptr.as_lifetime() )*].first().map_or(&(), |v| *v);
        #[allow(unused_unsafe)]
        let v = $typ $(:: $variant)? ( $($acc)* );
        // Make sure that we constructed something and didn't call a function which could leak the
        // pointers
        match &v {
            $typ $(:: $variant)? (..) if true => (),
            _ => unreachable!(),
        }
        #[allow(unused_unsafe)]
        unsafe { $crate::gc::Borrow::with_root(v, root) }
    } };
}

unsafe impl<'a, T> DataDef for Borrow<'a, T>
where
    T: DataDef,
    T::Value: Sized,
{
    type Value = T::Value;
    fn size(&self) -> usize {
        (**self).size()
    }
    fn initialize(self, result: WriteOnly<Self::Value>) -> &mut Self::Value {
        self.0.initialize(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::gc::gc::{GC,Generation,GcHeader};
    #[test]
    fn test_gc() {
        let mut gc = GC::new(Generation::default(), usize::MAX);
        dbg!(GcHeader::value_offset());
    }

}
