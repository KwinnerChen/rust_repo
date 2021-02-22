use std::str;


fn give_me<T>(value: T) {
    let _ = value;
}


fn main() {
    let s = "hellow";
    let a = 5;

    give_me(s);
    give_me(&a);

    let num_from_str = str::parse::<u8>("34").unwrap();
    println!("{}", num_from_str);
}
