////////// DO NOT CHANGE BELOW HERE /////////
fn print_result(num: i32) {
    println!("The result is {num}");
}
////////// DO NOT CHANGE ABOVE HERE /////////

// TODO: create `math!()` macro.
macro_rules! math {
    (square $a:expr) => {
        $a * $a
    };
    ($a:expr, plus, $b:expr) => {
        $a + $b
    };
}
////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    let var = 5;
    print_result(math!((2 * 3), plus, var));
    print_result(math!(square var));
}
