use std::env;
use std::mem;
use std::process;

use initializer::Initializer;
use logger::Logger;

pub mod run;

fn main() {
    let filename = String::from("config.yml");
    let args: Vec<String> = env::args().collect();
    let main_arg = args[1].clone();
    mem::drop(args);

    match main_arg.as_str() {
        "init" => {
            let initializer = Initializer::new(filename.clone());
            match initializer.create_yml(None) {
                Ok(_) => Logger::init_success(&filename, None),
                Err(err) => Logger::init_failed(&err),
            };
        }
        "run" => {
            let runner = run::Runner::new(filename.clone());
            runner.run();
        }
        _ => {
            Logger::wrong_args();
            process::exit(1);
        }
    }
}
