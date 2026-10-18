use std::io;

fn main() {
    let secret: u32 = rand::random_range(1..=100);

    println!("1から100の数を当ててください。");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("行の読み込みに失敗しました");

    let guess: u32 = guess.trim().parse().expect("数値を入力してください。");

    println!("あなたの入力: {guess}");
    println!("答え: {secret}")
}
