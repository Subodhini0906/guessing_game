use std::{cmp::Ordering, io};
use rand::Rng;
fn main() {  
    println!("guessing game");

    let secret_num=rand::thread_rng().gen_range(1, 150);
    println!("secret:{}",secret_num);

    loop {
        println!("pls input your guess");

        let mut guess =String::new();

        io::stdin()
           .read_line(&mut guess)
           .expect("Failed to read line");
        let guess:u32 =guess.trim().parse().expect("fail");
        println!("guess:{}",guess);
        
        match guess.cmp(&secret_num){
            Ordering::Less=>println!("Too low !"),
            Ordering::Greater=>println!("Too high!"),
            Ordering::Equal=>{
                println!("win win!");
                break;
            },
        }
    }
}
