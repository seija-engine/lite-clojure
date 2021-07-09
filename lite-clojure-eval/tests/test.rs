use lite_clojure_eval::EvalRT;
#[test]
fn test_eval() {
    let mut rt = EvalRT::new();
    rt.init();

    rt.eval_file("tests/loop.clj");
    
}