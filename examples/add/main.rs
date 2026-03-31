use vinculum::functions::add;

#[vinculum::main(haskell_file = "examples/add/Script.hs")]
fn main() {
    let result = add(2, 3);
    println!("Result: {}", result);
}
