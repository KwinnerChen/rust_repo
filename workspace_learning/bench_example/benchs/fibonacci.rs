#![allow(unused_imports)]

#[macro_use]  // 该注释用于将criterion中的宏引入作用域，而不需要使用use单独引用
extern crate criterion;
extern crate bench_example;

use criterion::Criterion;
use bench_example::{fast_fibonacci, slow_fibonacci};

// 主测试函数，默认接受一个Criterion实例为参数
fn fast_fibonacci_benchmark(c: &mut Criterion) {
    // bench_function接受一个字符串和一个以&mut Bencher为参数的闭包，无返回值
    c.bench_function("fibonacci 8", |b| b.iter(|| fast_fibonacci(8)));
}

fn slow_fibonacci_benchmark(c: &mut Criterion) {
    c.bench_function("fibonacci 8", |b| b.iter(|| slow_fibonacci(8)));
}

// 将测试函数编组，函数个数不限，编组实际是将测试函数写入一个函数，函数名称为宏的ident类型
criterion_group!(fib_bench, slow_fibonacci_benchmark, fast_fibonacci_benchmark);
// 该宏将编组函数置于main函数中用于编译运行
criterion_main!(fib_bench);