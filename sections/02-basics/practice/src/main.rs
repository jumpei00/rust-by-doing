fn main() {
    for i in 1..=15 {
        println!("{}", fizzbuzz(i));
    }

    for n in 0..=10 {
        println!("fib({n}) = {}", fib(n));
    }

    println!("{}C = {}F", 100.0, celsius_to_fahrenheit(100.0))
}

fn fizzbuzz(n: u32) -> String {
    match (n % 3, n % 5) {
        (0, 0) => String::from("FizzBuzz"),
        (0, _) => String::from("Fizz"),
        (_, 0) => String::from("Buzz"),
        _ => n.to_string(),
    }
}

fn fib(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }

    let mut prev: u64 = 0;
    let mut curr: u64 = 1;

    for _ in 1..n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }

    curr
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}
