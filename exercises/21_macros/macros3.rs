// TODO: Fix the compiler error without taking the macro definition out of this
// module.
mod macros {
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}
// The macro is available at the crate root, so no need to use it from the module.
fn main() {
    my_macro!();
}
