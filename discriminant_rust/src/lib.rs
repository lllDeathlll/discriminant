use std::{error::Error, io::ErrorKind};

pub struct Discriminant {
    a: Option<f64>,
    b: Option<f64>,
    c: Option<f64>,
}

pub fn discriminant(
    a: Option<f64>,
    b: Option<f64>,
    c: Option<f64>,
) -> Result<(Option<f64>, Option<f64>), Box<dyn Error>> {
    let disc = Discriminant { a, b, c };

    let a = disc.a.unwrap_or(0.0);
    let b = disc.b.unwrap_or(0.0);
    let c = disc.c.unwrap_or(0.0);

    if (a == 0.0 || b == 0.0) && c == 0.0 || (c == 0.0 || b == 0.0) && a == 0.0 {
        // If two or more variables are null returns error
        Err(Box::new(std::io::Error::new(
            ErrorKind::Other,
            "You must enter 2 or more variables",
        )))
    } else if a == 0.0 {
        let x1 = disc.b.unwrap() / disc.c.unwrap() * -1.0;

        Ok((Some(x1), None))
    } else if b == 0.0 {
        let x1 = (disc.a.unwrap() / (disc.c.unwrap() * -1.0)).sqrt();
        let x2 = x1 * -1.0;

        Ok((Some(x1), Some(x2)))
    } else if c == 0.0 {
        let x = disc.b.unwrap() / disc.a.unwrap() * -1.0;

        Ok((Some(x), Some(0.0)))
    } else {
        // Calculates discriminant
        let discriminant =
            f64::powf(disc.b.unwrap(), 2.0) - (4.0 * disc.a.unwrap() * disc.c.unwrap());
        if discriminant < 0.0 {
            Err(Box::new(std::io::Error::new(
                ErrorKind::Other,
                "Discriminant is negative",
            )))
        } else {
            let x1 = (disc.b.unwrap() * -1.0 + f64::sqrt(discriminant)) / 2.0 * disc.a.unwrap();
            let x2 = (disc.b.unwrap() * -1.0 - f64::sqrt(discriminant)) / 2.0 * disc.a.unwrap();

            Ok((Some(x1), Some(x2)))
        }
    }
}
pub fn errhandle(err: String) -> Option<f64> {
    // If string is empty sets value to none
    if err == "cannot parse float from empty string" {
        None
    }
    // If string is not a number sets value to none
    else if err == "invalid float literal" {
        eprintln!("Value must be a number");
        None
    }
    // Panics with error message on other error
    else {
        panic!("Error: {}", err);
    }
}
