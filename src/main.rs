use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Usage: cargo run [arg1] [arg2] ...");
    } else {
        for arg in &args[1..] {
            println!("{}", arg);
        }
    }
}
