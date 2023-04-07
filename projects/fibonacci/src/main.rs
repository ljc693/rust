use std::io;

fn main() {
    // 生成 n 阶斐波那契数列。
    println!("输入 n :");
    let mut inp: String = String::new();
    io::stdin().read_line(&mut inp).expect("input error!");
    let inp: u32 = inp.trim().parse().unwrap();

    println!("{}阶斐波那契数列: {}", inp, fibonacci(inp));
}
fn fibonacci(n: u32) -> u32 {
    if n == 1 {
        1
    } else if n == 2 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
