// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.

mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }

    pub fn call_macro() {
        my_macro!(); // Call the macro inside the module
    }
}

fn main() {
    macros::call_macro(); // Call the function that uses the macro
}
