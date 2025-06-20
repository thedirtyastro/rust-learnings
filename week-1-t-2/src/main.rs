fn main() {
    println!("--- Exercise 1: Tuple Declaration and Access (with indexing) ---");
    let person = ("Alice", 30, 5.6); // Type inference
    println!("Name: {}", person.0);
    println!("Age: {}", person.1);
    println!("Height: {}\n", person.2);

    println!("--- Exercise 2: Destructuring Tuples (with pattern matching) ---");
    let point = (10, 20);
    match point {
        (x, y) => println!("x = {}, y = {}\n", x, y),
    }

    println!("--- Exercise 3: Tuple from Function Return (with destructuring) ---");
    let (name, id) = get_user();
    println!("User: {}, ID: {}\n", name, id);

    println!("--- Exercise 4: Manual Iteration via Macro ---");
    let scores = (100, 90, 80);
    // Manually access using macro or tuple indexing
    print_scores(scores);

    println!("--- Exercise 5: Tuple of Mixed Types (using match destructuring) ---");
    let mixed = ("Rust", true, 3.14);
    match mixed {
        (lang, is_awesome, version) => {
            println!("Language: {}", lang);
            println!("IsAwesome: {}", is_awesome);
            println!("Version: {}\n", version);
        }
    }
}

fn get_user() -> (&'static str, i32) {
    ("Ramesh", 999)
}

fn print_scores(scores: (i32, i32, i32)) {
    let (a, b, c) = scores;
    for (i, val) in [a, b, c].iter().enumerate() {
        println!("Score {}: {}", i + 1, val);
    }
    println!();
}
