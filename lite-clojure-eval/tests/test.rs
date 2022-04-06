use lite_clojure_eval::{EvalRT, Variable};
#[test]
fn test_loop() {
    let mut rt = EvalRT::new();
    rt.init();
    rt.eval_file("tests/loop.clj");  
}

#[test]
fn test_map() {
    let mut rt = EvalRT::new();
    rt.init();
    if let Some(ret) = rt.eval_file("tests/map.clj") {
        println!("{:?}",ret);
    }
}
#[test]
fn test_invoke() {
    
    let mut rt = EvalRT::new();
    rt.init();
    rt.eval_file("tests/invoke.clj");
    let ret = rt.invoke_func("foo", vec![Variable::Int(2)]).ok().and_then(|v| v.cast_int());
    assert!(ret == Some(114516));
}