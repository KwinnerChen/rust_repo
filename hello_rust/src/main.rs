fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {

    #[test]
    fn test_params() {
        let a = 1;
        fn chage_a(mut x: i32) -> i32 {
            x += 1;
            x
        }

        let b = chage_a(a);

        println!("{}, {}", &a, &b);
        assert_ne!(a, chage_a(a));
    }
}