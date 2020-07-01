pub fn run() {
    greeting("hello", "aaron");

    let sum = add(5, 5);
    println!("{}", sum);
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    return n1 + n2;
}