use std::io;
use rand::Rng;
fn main() {  
    println!("guessing game");

    let secret_num=rand::thread_rng().gen_range(1, 150);

    println!("pls input your guess");
    println!("secret:{}",secret_num);

    let mut guess =String::new();

    io::stdin()
       .read_line(&mut guess)
       .expect("Failed to read line");
    println!("guess:{}",guess);
}
