use vinculum_hs::functions::math::{add, factorial, multiply, testfn};

#[vinculum_hs::main]
fn main() {
    let a = 5;
    let b = 10;

    let result = add(a, b);
    println!("{a} + {b} = {result}");

    let result = multiply(a, b);
    println!("{a} * {b} = {result}");

    let result = factorial(a);
    println!("Factorial 5 = {result}");

    println!("{}", testfn(4));
}
