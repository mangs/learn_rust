fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 2) + fibonacci(n - 1),
    }
}

fn main() {
    let mut user_choice = String::new();
    println!("Calculate the nth Fibonacci number:");

    std::io::stdin()
        .read_line(&mut user_choice)
        .expect("error reading your choice");
    
    let user_choice: u32 = user_choice
        .trim()
        .parse()
        .expect("couldn't convert input to number");

    println!("Outcome: {}", fibonacci(user_choice));
}
