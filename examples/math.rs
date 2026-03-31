use vinculum_hs::functions::math::{add, factorial, multiply};

#[vinculum_hs::main(haskell_directory = "examples/haskell")]
fn main() {
    let a = 5;
    let b = 10;

    let result = add(a, b);
    println!("{a} + {b} = {result}");

    let result = multiply(a, b);
    println!("{a} * {b} = {result}");

    let result = factorial(a);
    println!("Factorial 5 = {result}");
}
