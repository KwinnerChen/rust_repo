#[macro_export]
macro_rules! map {
    ($($k:expr => $v:expr), *) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($k, $v);
            )*
            map
        }
    };

    // 用于匹配左后条目添加逗号的情况
    ($($k:expr => $v:expr), *,) => {
        map! {$($k => $v),*}
    };
}


#[cfg(test)]
mod tests {
    #[test]
    fn do_work() {
        let d = map! {
            "1" => 1,
            "2" => 2,
            "3" => 3,
        };

        assert_eq!(d["1"], 1);
        assert_eq!(d["2"], 2);
    }
}