use minigrep;

#[test]
fn one_result() {
    let query = "duct";
    let contents = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";

    assert_eq!(vec!["safe, fast, productive."], minigrep::search(query, contents));
}


#[test]
fn case_insensitive() {
    let query = "rUsT";
    let contents = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";

    assert_eq!(vec!["Rust:", "Trust me."], minigrep::search_case_insensitive(&query, &contents));
}