use std::io;

fn read_opt() -> i32 {
    let opts = ["F", "C"];
    let mut buf = String::new();
    println!("Do you want to convert from Celsius to Farenheit (F) or Farenheit to Celsius (C)?");

    loop {
        buf.clear();
        io::stdin().read_line(&mut buf).expect("Did stdin close?");
        buf.make_ascii_uppercase();
        let opt = buf.trim();

        for i in 0..(opts.len()) {
            if opts[i].eq(opt) {
                return i as i32;
            }
        }

        println!("Invalid value. Please type F or C:")
    }
}

fn read_f64() -> f64 {
    return loop {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).expect("Did stdin close?");
        match buf.trim().parse::<f64>() {
            Ok(value) => {
                break value;
            }
            Err(_) => {
                println!("Not a valid number!")
            }
        }
    };
}

fn main() {
    let opt = read_opt();
    
    if opt == 0 {
        // Celsius to Farenheit
        println!("Enter a value in Celsius: ");
        let num = read_f64();
        let converted = num * 9.0 / 5.0 + 32.0;
        println!("{num}°C equals {converted}°F");
    } else if opt == 1 {
        // Farenheit to Celsius
        println!("Enter a value in Farenheit: ");
        let num = read_f64();
        let converted = (num - 32.0) * 5.0 / 9.0;
        println!("{num}°F equals {converted}°C");
    } else {
        println!("Error 💀");
    }
}
