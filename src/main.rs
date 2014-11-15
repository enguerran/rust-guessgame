use std::io;
use std::rand;

fn get_secret() -> uint {
	return (rand::random::<uint>() % 100u) + 1u;
}

fn cmp(a: uint, b: uint) -> Ordering {
    if a < b { Less }
    else if a > b { Greater }
    else { Equal }
}

fn main() {
    println!("\nHello fellow,");
    println!("Guess the number!");

    let secret_number = get_secret();
    println!("The secret number is: {}", secret_number);

    loop {
	    println!("Please input your guess.");

	    let input = io::stdin().read_line()
	                           .ok()
	                           .expect("Failed to read line");

	    let input_num: Option<uint> = from_str(input.as_slice().trim());

	    let num = match input_num {
	        Some(num) => num,
	        None      => {
	            println!("Please input a number!");
	            continue;
	        }
	    };

	    println!("You guessed: {}", num);

	    match cmp(num, secret_number) {
	        Less    => println!("Too small!"),
	        Greater => println!("Too big!"),
	        Equal   => {
	        	println!("You win!");
	        	return;
	        },
	    }   
    }
}