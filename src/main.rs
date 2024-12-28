use std::io; //io is a module
use rand::Rng; //rng is a trait
use std::cmp::Ordering;

fn main() {
    println!("guess!");
    let secret = rand::thread_rng().gen_range(1..=100);
    loop {
    let mut guess = String::new(); //new is a function of the String struct
    

    io::stdin() //returns Result Enum - this is what .expect looks at
        .read_line(&mut guess) //& means reference, not address like in C, immutable by default
        .expect("Failed"); //error handling
    //same as io::stdin().read_line(&mut guess).expect("Failed to read line");
    let guess: i32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue, //underscore is catch all

    }; 

    match guess.cmp(&secret){
        Ordering::Less => println!("too small"),
        Ordering::Equal => {
            println!("nice"); 
            break;
        },
        Ordering::Greater => println!("too big"),
    }

    println!("You guessed {guess}");
    let distance = secret - guess;
    println!("you were {distance} away!\n answer was {secret}");
    }

}

//SemVer automatically used in .toml 

/* example:
let x = 5;
let y = 10;

println!("x = {x} and y + 2 = {}", y + 2);
 */ 