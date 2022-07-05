use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    let mut final_string = String::new();
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    let mut counter_tries: u32 = 1;
    let mut prev_number: u32 = 0;

    println!("Guess the number!");

    loop {
        

        if counter_tries == 5 {
            println!("Is this hard?? You achieved five tries. =]");
        }
        println!("Please, input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        if guess == prev_number{
            println!("Why are you trying this number again? Are you trolling? =[")
        } else {
            println!("You guessed: {guess}");

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                },
            }

            prev_number = guess;
            counter_tries = counter_tries + 1;
        }
    }
    println!("Press enter to quit...");
    
    io::stdin()
        .read_line(&mut final_string)
        .expect("Failed to read line");

}