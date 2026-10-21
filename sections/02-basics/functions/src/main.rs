fn main() {
    greet("Rust");
    print_labeled(5, 'm');

    let y = {
        let x = 3;
        x + 1
    };
    println!("y = {y}");

    println!("five() = {}", five());
    println!("plus_one(5) = {}", plus_one(5));
}

fn greet(name: &str) {
    println!("Hello, {name}!");
}

fn print_labeled(value: i32, unit: char) {
    println!("mesure: {value}{unit}")
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
