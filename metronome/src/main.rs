use colored::*;
use std::{thread, time, io};

fn bpm_selector() -> u16 {

    std::process::Command::new("clear").status().unwrap();
    let mut input = String::new();
    let mut bpm = 0;

    println!("Enter bpm:");

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    let clean_input = input.trim();
    match clean_input.parse::<u16>() {
        Ok(i) => bpm = i,
        Err(..) => println!("not an integer"),
    };
    bpm
}

fn bpm_calculator(bpm: u16) -> u16 {
    
    let calculated_bpm = 1000 / (bpm / 60);

    calculated_bpm
}

fn metronome(calculated_bpm: u16) {
    let count: [&str; 5] = 
        ["0",
        "1 |",
        "2 ||",
        "3 |||",
        "4 ||||"];
    let mut n: usize = 0;
    loop {
        std::process::Command::new("clear").status().unwrap();
        n += 1;
        println!("{}", count[n]);
        thread::sleep(time::Duration::from_millis(calculated_bpm.into()));
        if n == 4 {
            n = 0;
        }
    }
}

fn main(){
    metronome(bpm_calculator(bpm_selector()));
}

