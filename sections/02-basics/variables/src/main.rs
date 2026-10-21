const MAX_POINTS: u32 = 100_000;

fn main() {
    let x = 5;
    println!("x = {x}");

    let mut y = 10;
    println!("y = {y}");
    y = 20;
    println!("y = {y}");

    let x = x + 1;
    {
        let x = x * 2;
        println!("inside x = {x}")
    }
    println!("outside x = {x}");

    let spaces = "  ";
    let spaces = spaces.len();
    println!("spaces = {spaces}");

    println!("MAX_POINTS = {MAX_POINTS}")
}
