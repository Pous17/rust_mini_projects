use std::io;

fn main() {
    println!("Please enter a size value: ");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let mut element = index;

    let mut c = element;

    let mut d = element; 

    let mut spaces: usize = 0; 

    let mut invertspaces: usize = element.try_into().unwrap();

    let base: usize = element.try_into().unwrap();
 

    println!("The length is: {element}");

    while element > 0 {
        println!("|");
        element = element - 1;
    }
    println!(r" {}", "-".repeat(base * 2)); 
    while c > 0 {
        println!("|{}|", " ".repeat(base * 2));
        c = c - 1;
    } 
    if c <= 0 {
        println!(r" {}", "-".repeat(base * 2));
    }
    while d > 0 {
        spaces += 2;
        invertspaces -= 1;
        println!(r" {} /{}\ ", " ".repeat(invertspaces), " ".repeat(spaces - 1));
        d = d - 1;
    }
    if d <= 0 {
        println!(r"   {}", "-".repeat((base * 2) - 1))
    }
    println!("   {}", "-".repeat((base * 2) - 1));

    while d > 0 {
    println!(r" {} /{}\ ", " ".repeat(invertspaces), " ".repeat(spaces - 1));
    d += 1; 
    }
}