trait Inter {
    fn exposed(&self, a: &str) -> bool;
}

struct Implementation {
    value: i32,
    has: bool,
}

impl Inter for Implementation {
    fn exposed(&self, a: &str) -> bool {
        if self.value == 0 {
            true
        } else {
            false
        }
    }
}

fn main() {
    let i = Implementation {
        value: 1,
        has: false,
    };
    println!("{:?}", i.exposed("1"));
}