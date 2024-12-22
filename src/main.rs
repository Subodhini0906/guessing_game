use std::{cmp::Ordering, io};
use rand::Rng;
use colored::*;
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
        let guess:u32 =match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue,
        };
        println!("guess:{}",guess);
        
        match guess.cmp(&secret_num){
            Ordering::Less=>println!("{}","Too low !".red()),
            Ordering::Greater=>println!("{}","Too high!".purple()),
            Ordering::Equal=>{
                println!("{}","win win!".green());
                break;
            },
        }
    }
}
