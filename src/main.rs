use std::env;

fn main() {
    let day: i8 = env::args()
        .nth(1)
        .expect("Not a valid day!")
        .parse()
        .expect("Not a valid day!");

    match day {
        _ => panic!("Either invalid or not implemented")
    }
}
