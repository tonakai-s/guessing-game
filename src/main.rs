use std::io;
use std::cmp::Ordering;
use rand::Rng;
#[derive(Debug)]
pub struct Guess {
    value: i32
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess out of range, 1 and 100.");
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main(){
    let mut final_string = String::new();
    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);
    let mut counter_tries: i32 = 1;

    println!("Guess the number!");

    loop {
        println!("Please, input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let guess = Guess::new(guess);
        
        println!("You guessed: {:?}", guess);

        match guess.value.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }

        counter_tries = counter_tries + 1;
    }
    println!("Press enter to quit...");
    
    io::stdin()
        .read_line(&mut final_string)
        .expect("Failed to read line");
}