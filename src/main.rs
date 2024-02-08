fn main() {
    // read message.txt into the program
    let message = std::fs::read_to_string("src/message.txt").unwrap();
    println!("{}", message);
}
