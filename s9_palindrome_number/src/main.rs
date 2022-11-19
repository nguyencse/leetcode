fn main() {
    let x = -121;

    let result = x.to_string() == x.to_string().chars().collect::<String>();
    println!("result: {}", result);
}
