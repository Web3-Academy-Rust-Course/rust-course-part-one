trait Displayable {
    fn display(&self) -> String;
    fn default_display(&self) {
        println!("{:?}", String::from("default display"))
    }
}

impl Displayable for String {
    fn display(&self) -> String {
        self.to_string()
    }
}

impl Displayable for i32 {
    fn display(&self) -> String {
        self.to_string()
    }
}

struct Random {
    name: String,
}

impl Displayable for Random {
    fn display(&self) -> String {
        self.name.clone()
    }
}

fn print_value<T: Displayable>(value: &T) {
    println!("{}", value.display());
}

fn main() {
    // You can initialize String like "".to_string()
    let name: String = "Alice".to_string();
    let age: i32 = 30;
    // You can inititalize String like String::from("")
    let random = Random {
        name: String::from("Random Name"),
    };

    // Everything that implements Displayable trait will by default have display_default method
    // becuase it has default implmentation and it does not need aditional defining like dispaly
    // method
    random.default_display();
    name.default_display();
    print_value(&random);
    print_value(&name); // Prints "Alice"
    print_value(&age); // Prints "30"
}
