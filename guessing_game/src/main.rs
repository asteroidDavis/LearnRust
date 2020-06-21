use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please guess your number: ");
        // creates a mutable variable because by default they are immutable.
        // On the other hand String by default is growable.
        // :: is an type associated function(think static) instead
        // of . for instance associated functions
        let mut guess = String::new();
        // notice that you need to specify this is a reference to a mutable instead
        // of just a reference
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
        // shardow the previous guess value with a new value
        // And handle possible errors by evaluating the result with a match block
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
