use std::{cell::{Cell}, ops::{Deref,DerefMut}};
use std::mem;
use std::ptr;
#[derive(Clone, Copy, Default, Debug)]
pub struct Generation(i32);

pub unsafe trait DataDef {
    type Value:?Sized  + for<'a> FromPtr<&'a Self>;
    fn size(&self) -> usize;
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
    fn new<T>( value_size: usize) -> AllocPtr {
        let alloc_size = GcHeader::value_offset() + value_size;
        unsafe  {
            let ptr = allocate(alloc_size) as *mut GcHeader;
            ptr::write(
                ptr,
                GcHeader {
                    next: None,
                    value_size: value_size,
                    marked: Cell::new(false),
                },
            );
            AllocPtr {ptr}
        }
    }

    fn size(&self) -> usize {
        GcHeader::value_offset() + self.value_size
    }
}

pub struct GC {
    values: Option<AllocPtr>,
    memory_limit: usize,
    generation:Generation,
    allocated_memory:usize
}

impl GC {
    pub fn new(generation: Generation, memory_limit: usize) -> GC {
        GC {
            values:None,
            generation,
            memory_limit,
            allocated_memory:0
        }
    }

    pub fn alloc<D>(&mut self,def:D) where D:DataDef,D::Value: Sized {
        let size = def.size();
        let needed = self.allocated_memory.saturating_add(size);
        if needed >= self.memory_limit {
            panic!("out of memory");
        }
        self.alloc_ignore_limit_(size, def);
    }

    fn alloc_ignore_limit_<D>(&mut self,size:usize,def:D) where D:DataDef,D::Value: Sized {
        let mut ptr = AllocPtr::new::<D::Value>(size);
        ptr.next = self.values.take();
        self.allocated_memory += ptr.size();
        unsafe  {
            
        }
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

#[test]
fn test_gc() {
    let mut gc = GC::new(Generation::default(), usize::MAX);
    dbg!(GcHeader::value_offset());
}