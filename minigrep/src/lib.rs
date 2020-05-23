use std::fs;
use std::error::Error;
use std::env;


pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    // args是一个命令行参数的迭代器
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("didn't get a filename string"),
        };

        let case_sensitive = env::var("CASE_SEVSITIVE").is_err();

        Ok(Config {
            query: query,
            filename,
            case_sensitive,
        })
    }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(&config.query, &content)
    } else {
        search_case_insensitive(&config.query, &content)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}


pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();

    // for line in content.lines() {
    //     if line.contains(query) {
    //         results.push(line)
    //     }
    // }

    // results

    // 使用迭代器
    content.lines().filter(|line| line.contains(query)).collect()
}


pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    // let mut results = Vec::new();

    // for line in content.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         results.push(line);
    //     }
    // }

    // results

    // 使用迭代器
    content.lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}
