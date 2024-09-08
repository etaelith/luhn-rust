use std::io::stdin;

fn main() {
    let mut cc: i64;
    let mut digit_count = 0;
    let mut sum = 0;

    loop {
        println!("Enter card number: ");
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        cc = input.trim().parse().unwrap_or(-1);

        if cc > 0 {
            break;
        }
    }
    let amex = (cc as f64 / f64::powf(10.0, 13.0)) as i32;
    let master_long = (cc as f64 / f64::powf(10.0, 12.0)) as i32;
    let master_short = (cc as f64 / f64::powf(10.0, 14.0)) as i32;
    let visa = [
        (cc as f64 / f64::powf(10.0, 12.0)) as i32,
        (cc as f64 / f64::powf(10.0, 15.0)) as i32,
    ];

    while cc > 0 {
        let mut digit = (cc % 10) as i32;
        cc /= 10;
        digit_count += 1;

        if digit_count % 2 == 0 {
            digit *= 2;
            if digit > 9 {
                digit -= 9;
            }
        }

        sum += digit;
    }

    // Statements to check if cards are valid, American Express, Visa, Mastercard, or invalid
    if sum % 10 == 0 {
        if amex == 34 || amex == 37 {
            println!("AMEX");
        } else if (51..=55).contains(&master_short) || (2221..=2720).contains(&master_long) {
            println!("MASTERCARD");
        } else if visa.contains(&4) {
            println!("VISA");
        } else {
            println!("INVALID");
        }
    } else {
        println!("INVALID");
    }
}
