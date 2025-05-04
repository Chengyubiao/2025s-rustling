// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
// hint.



macro_rules! my_macro {
    ($msg:expr) => {
        println!("{}", $msg);
    };
}

fn main() {
    my_macro!("Check out my macro!");
    my_macro!("This is a custom message!");
}
