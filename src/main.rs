use std::error::Error;

// mod ex1;
// mod ex2;
mod ex3;

fn main() -> Result<(), Box<dyn Error>> {
    // println!("{:#?}", ex1::main(&vec![1234, 45, 32, 3, 25, 73, 76, 4, 46, 19]));
    // println!("{}", ex2::main("Mary had a little lamb"));
    ex3::main()?;

    Ok(())
}
