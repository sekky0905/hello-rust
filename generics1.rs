fn main() {
    println!("result of print_result(1)");
    print_result(1);
    println!("result of print_result(7)");
    print_result(7);
}

fn test(t: i64) -> Result<i64, String> {
    if t > 5 {
        Ok(t)
    } else {
        Err("Error".to_string())
    }
}

fn print_result(t: i64) {
    match test(t) {
        Ok(t) => println!("Ok. argument = {}", t),
        Err(e) => println!("Err: {}", e),
    }
}