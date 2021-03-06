fn main() {
    for i in 1..=100 {
        println!("{}", fizzbuzz(i));
    }
}

fn fizzbuzz(number: u32) -> String {
    match (number % 3, number % 5) {
        (0, 0) => "fizzbuzz".to_string(),
        (0, _) => "fizz".to_string(),
        (_, 0) => "buzz".to_string(),
        (_, _) => number.to_string(),
    }
}
