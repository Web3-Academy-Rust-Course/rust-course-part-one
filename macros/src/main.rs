macro_rules! check {
    ($input1:ident, $input2:expr) => {
        println!(
            "Is {:?} equal to {:?}? {:?}",
            $input1,
            $input2,
            $input1 == $input2
        );
    };
}

macro_rules! make_a_function {
    ($name:ident, $($input:tt),*) => { // First you give it one name for the function, then it checks everything else
        fn $name(what: String) {
            let output = stringify!($($input),*); // It makes everything else into a string
            println!("{} {:?}", output, what);
        }
    };
}

fn main() {
    let x = 6;
    println!("Hello, world!");
    check!(x, 2);
    make_a_function!(print_all, 4);
    print_all("I print it".to_string());
    print_all(String::from("What can I print"))
}
