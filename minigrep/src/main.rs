use std::env;
use std::process;
use minigrep::Config;
use minigrep::run;
use minigrep::test;


fn main() {
    let args:Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("searching for {}", config.query);
    println!("in file {}", config.filename);

    if let Err(e) = run(config) {
        println!("application error: {}", e);
        process::exit(1);
    };

    test::testlib::test1();
}

