fn main() {
    for i in 1..10 {
        println!("Hello,{} world!", to_original_string(i));
    }
}

fn main() {
    let s = "hello".to_string();
    let t = s;
    println!("{}", s);
    println!("{}", t);
}

fn to_original_string(n: usize) -> String {
    let s = match n % 10 {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    };

    format!("{}{}", n, s)
}
