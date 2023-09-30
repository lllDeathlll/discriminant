use discriminant_rust::{discriminant, errhandle};
use std::io;

fn main() {
    loop {
        println!("Discriminant Calculator");
        println!("Enter values according to the sample: ax^2+bx+c");

        // Gets variables checks them for errors
        println!("Enter variable a:");
        let mut a = String::new();
        io::stdin().read_line(&mut a).expect("Failed to read line");

        let a: Option<f64> = match a.trim().parse() {
            Ok(num) => Some(num),
            Err(err) => errhandle(err.to_string()),
        };

        println!("Enter variable b:");
        let mut b = String::new();
        io::stdin().read_line(&mut b).expect("Failed to read line");

        let b: Option<f64> = match b.trim().parse() {
            Ok(num) => Some(num),
            Err(err) => errhandle(err.to_string()),
        };

        println!("Enter variable c:");
        let mut c = String::new();
        io::stdin().read_line(&mut c).expect("Failed to read line");

        let c: Option<f64> = match c.trim().parse() {
            Ok(num) => Some(num),
            Err(err) => errhandle(err.to_string()),
        };

        // Matches cases of x being null and prints errors
        match discriminant(a, b, c) {
            Ok((Some(x1), Some(x2))) => println!("\nAnswer: x1={}, x2={}", x1, x2),
            Ok((Some(x), None)) => println!("\nAnswer: x={}\n", x),
            Ok((None, Some(x))) => println!("\nAnswer: x={}\n", x),
            Ok((None, None)) => println!("\n"),
            Err(error) => eprintln!("Error: {}\n", error),
        };
    }
}
