use lite_clojure_eval::EvalRT;
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
    rt.eval_file("tests/map.clj");
}

