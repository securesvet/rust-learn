// Just trying out comments in Rust

fn main() {
    format_print();
    hello();
}

fn hello() {
    println!("Hello, world!");
}

fn format_print() {
    println!("{} days", 31);

    // named arguments
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );
}
