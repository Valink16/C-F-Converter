use std::io;

fn main() {
    println!("[*] Welcome to this extremely precise and cleverly engineered temperature converter !");
    loop {
        let method = ask_bin("Please choose between the two conversion methods:\n[1] Celcius to Fahrenheit\n[2] Fahrenheit to Celcius\n".to_string(), ('1', '2'));
        
        let temp: f64 = loop { // Trying to read a temperature from user input
            println!("Please enter a valid temperature: ");
            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input)
                .expect("Unable to read user input");

            let user_input: f64 = match user_input.trim().parse() {
                Ok(t) => t,
                Err(_) => continue
            };
            break user_input;
        };
        
        if method { // Method is 1 (C to F)
            println!("{}C = {}F", temp, temp * 9.0 / 5.0 + 32.0);
        } else { // Method is 2 (F to C)
            println!("{}F = {}C", temp, (temp - 32.0) * 5.0 / 9.0);
        }
        
        
        if !ask_bin("Keep going ?".to_string(), ('y', 'n')) {
            break;
        }
    }
}

fn ask_bin(msg: String, answ: (char, char)) -> bool {
    // Used to ask questions which only have 2 possible answers, answers must be char type
    // Returns true if user input equals first answer, false if user input equals second answer
    loop {
        println!("{}\nPlease enter '{}' or '{}': ", msg, answ.0, answ.1);
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read input from user");
        
        let res: char = match input.trim().parse() {
            Ok(c) => c,
            Err(_) => continue
        };

        if res == answ.0 {
            break true;
        } else if res == answ.1 {
            break false;
        }
    }
}
