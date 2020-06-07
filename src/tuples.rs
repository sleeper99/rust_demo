// Tuples group together values of different types

pub fn run() {
    let person: (&str, &str, u8) = ("john", "wick", 30);

    println!("{} {} is {}", person.0, person.1, person.2);

}