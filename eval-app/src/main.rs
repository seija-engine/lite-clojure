use lite_clojure_eval::{EvalRT};

fn main() {
    let mut args = std::env::args();
    args.next();
    let file_path = args.next().unwrap_or(String::from("./index.clj"));
    
    let mut rt = EvalRT::new();
    rt.init();
    dbg!(&file_path);
    let code_string = std::fs::read_to_string(file_path);
    match code_string {
        Err(err) => {dbg!(err);},
        Ok(string) => {
            rt.eval_string(String::from("test"),&string);
        }
    }
    
}
