// Make a macro that takes 2 numbers and multiplies them, and then prints the outcome
macro_rules! multiply_print {
    ($input1:expr, $input2:expr) => {
        println!(
            "{:?} +  {:?}? is exual to {:?}",
            $input1,
            $input2,
            $input1 + $input2
        )
    };
}

fn main() {
    multiply_print!(1, 5)
}
