fn strip_line(line: &mut String) {
    if line.ends_with("\n") {
        line.pop();
        if line.ends_with("\r") {
            line.pop();
        }
    }
}

fn get_line() -> String {
    let mut line = String::new();
    loop {
        println!("Input your number:");
        match std::io::stdin().read_line(&mut line) {
            Ok(_) => {
                strip_line(&mut line);
                return line;
            }
            Err(e) => println!("Error: {}", e),
        };
    }
}

fn get_integer() -> i32 {
    loop {
        let line = get_line();
        match line.parse::<i32>() {
            Ok(n) => return n,
            Err(e) => println!("Error: {}", e),
        }
    }
}

fn get_secret_number(nmin: i32, nmax: i32) -> i32 {
    use rand::Rng;
    rand::thread_rng().gen_range(nmin, nmax + 1)
}

fn main() {
    let secret_number = get_secret_number(1, 100);
    loop {
        let guess = get_integer();
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("The secret number is greater."),
            std::cmp::Ordering::Greater => println!("The secret number is lower."),
            std::cmp::Ordering::Equal => {
                println!("Congratulations! The secret number {} is found.", guess);
                break;
            }
        };
    }
}
