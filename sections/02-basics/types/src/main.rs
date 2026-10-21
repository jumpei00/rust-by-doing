use std::io;

fn main() {
    let a: i32 = -42;
    let b: u8 = 255;
    let c = 98_222;
    let d = 0xff;
    println!("{a} {b} {c} {d}");

    let x = 2.0;
    let y: f32 = 3.0;
    println!("{x} {y}");
    println!("{}", 7 / 2);
    println!("{}", 7.0 / 2.0);
    println!("{}", 7 % 3);

    let t = true;
    let heart = '❤';
    let kanji = '字';
    println!("{t}, {heart}, {kanji}");

    let tup: (i32, f64, char) = (500, 6.4, 'z');
    let (p, q, r) = tup;
    println!("{p}, {q}, {r}");
    println!("{}, {}, {}", tup.0, tup.1, tup.2);

    let arr = [1, 2, 3, 4, 5];
    let zeros = [0; 3];
    println!("{}, {}", arr[0], arr[4]);
    println!("{:?} {:?}", arr, zeros);
    println!("len: {}", arr.len());

    println!("input -> (0~4)");

    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("error");

    let index: usize = index.trim().parse().expect("input number");

    let element = arr[index];
    println!("arr[{index}] = {element}")
}
