fn main() {
    loop {
        // Display the game state here

        // Get input from the user
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");

        // Process the input
        match input.trim() {
            "QUIT" => {
                println!("Thanks for playing ETT!");
                break;
            },
            _ => {
                println!("You entered: {}", input);
                // Add more game logic here later
            }
        }
    }
}

