use lite_clojure_eval::{EvalRT};
use std::time::{SystemTime, UNIX_EPOCH};
fn clock_realtime() -> i64 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    return since_the_epoch.as_millis() as i64;
}

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
            let ts = clock_realtime();
            rt.eval_string(String::from("test"),&string);
            let dt = clock_realtime() - ts;
            println!("time={} ms", dt / 1000);
        }
    }
    
}
