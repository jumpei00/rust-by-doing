fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("can divide at 4");
    } else if number % 3 == 0 {
        println!("can divide at 3");
    } else {
        println!("cannot divide");
    }

    let condition = true;
    let value = if condition { 5 } else { 6 };
    println!("value = {value}");

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result = {result}");

    let mut n = 3;
    while n != 0 {
        println!("{n}!");
        n -= 1;
    }
    println!("go!!");

    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("element: {element}");
    }
    for i in (1..4).rev() {
        println!("{i}...");
    }
}
