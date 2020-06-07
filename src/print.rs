pub fn run() {
    // Print to consol
    println!("Hello world!");

    // Basic Formatting
    println!("{} is {}", "he", "fat");

    // Positional Arguments
    println!(
        "{0} is {1}, and {0} likes {2}", "Brad", "fat", "hot wings"
    );

    // Named Arguments
    println!(
        "{name} likes to play {game}", name="John", game="IRL shotting game"
    );

    // Placeholder traits
    println!(
        "Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10
    );

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}
