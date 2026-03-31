use vinculum::functions::math::add;

#[vinculum::main(haskell_file = "examples/add")]
fn main() {
    let result = add(2, 3);
    println!("Result: {}", result);
}
