fn main() {
    for i in 1..10 {
        println!("fibonacci({}) = {}", i, fibonacci(i));
    }
}

fn fibonacci(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
