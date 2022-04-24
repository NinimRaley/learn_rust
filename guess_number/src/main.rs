use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Угадай число");
    let secret_number = rand::thread_rng().gen_range(1, 100);
    println!("Секретное число: {}", secret_number);
    loop {
        println!("Введи свою догадку");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Не удалось получить число");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
    
        println!("Вы загадали {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Слишком малое"),
            Ordering::Greater => println!("Слишком большое"),
            Ordering::Equal => {
                println!("Вы выиграли");
                break;
            }
        }
    }
}