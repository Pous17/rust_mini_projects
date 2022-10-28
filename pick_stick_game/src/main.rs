use std::{thread, time, io};
use rand::Rng;

fn player_input() -> String {
    let mut play = String::new();
    println!("Your turn:");

    io::stdin()
        .read_line(&mut play)
        .expect("Unable to read input");
    play
}

fn valid_input() -> i8 {
    player_input()
        .trim()
        .parse()
        .unwrap_or(0)
}
fn main() { 

    // credit https://stackoverflow.com/a/65583826/20337705
    std::process::Command::new("clear").status().unwrap();

    println!("\nWelcome to my stick game.");
    println!("\nEach turn you and your adversary will pick between 1 and 3 sticks.");
    println!("\nThe player who picks the last stick loses.\n");
    println!("\nLoading...");

    thread::sleep(time::Duration::from_millis(5000));
    let mut stick: i8 = 16;

    loop {
        std::process::Command::new("clear").status().unwrap();
        println!("{}\n", "|".repeat(stick as usize));
        
        let mut validated: bool = true;
        let a: i8 = valid_input();

        if a >= 1 && a <= 3 {
            stick -= a;
            thread::sleep(time::Duration::from_millis(200));
        } else {
            println!("Enter a valid number");
            thread::sleep(time::Duration::from_millis(700));
            validated = false;
        }

        if stick <= 0 {
            println!("You have lost...");
            break;
        }
        
        //avoid that the adversary plays when you entered a invalid input.
        if validated == true {
            let adversary_play = rand::thread_rng().gen_range(1..=3);
            stick -= adversary_play;

            println!("Your adversary took {} sticks", adversary_play); 
        }

        if stick <= 0 {
            println!("\nYou win!");
            break;
        }

        thread::sleep(time::Duration::from_millis(800));

    }
}