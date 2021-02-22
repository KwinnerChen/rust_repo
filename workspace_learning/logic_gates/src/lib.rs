//! 这是一个逻辑门模拟器程序，用于演示编写单元测试和集成测试


/// 实现一个逻辑门and，将两个位作为输入，并返回一个位作为输出
pub fn and(a: u8, b: u8) -> u8 {
    match (a, b) {
        (1, 1) => 1,
        _ => 0
    }
}

/// 实现一个逻辑门xor，将两个位作为输入，并返回一个位作为输出
pub fn xor(a: u8, b: u8) -> u8 {
    match (a, b) {
        (1, 0) | (0, 1) => 1,
        _ => 0,
    }
}

/// 使用原始的逻辑门实现了一个半加法器·
pub fn half_adder(a: u8, b: u8) -> (u8, u8) {
    (xor(a, b), and(a, b))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(1, and(1, 1));
        assert_eq!(0, and(0, 1));
        assert_eq!(0, and(1, 0));
        assert_eq!(0, and(0, 0));
    }

    #[test]
    fn test_xor() {
        assert_eq!(1, xor(1, 0));
        assert_eq!(0, xor(0, 0));
        assert_eq!(0, xor(1, 1));
        assert_eq!(1, xor(0, 1));
    }

    #[test]
    fn one_bit_adder() {
        const TEST_COLLECTION: [((u8, u8), (u8, u8)); 4] = [
            ((0, 0), (0, 0)),
            ((1, 0), (1, 0)),
            ((0, 1), (1, 0)),
            ((1, 1), (0, 1)),
        ];

        for (inn, out) in TEST_COLLECTION.to_vec() {
            let (a, b) = inn;
            println!("Testing: {}, {} -> {:?}", a, b, out);
            assert_eq!(half_adder(a, b), out);
        }
    }
}