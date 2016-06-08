fn main() {
    println!("{}", is_alpha_numeric(100));
}

fn is_alpha_numeric(cc: u8) -> bool {
    match cc {
        97...122 | 65...90 | 48...57 => {
            true
        }
        _ => {
            false
        }
    }
}

