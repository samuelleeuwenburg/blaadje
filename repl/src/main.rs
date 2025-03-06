use blaadje::{prelude_environment, run_with_env};
use std::error::Error;
use std::io::{stdin, stdout, Write};

fn main() -> Result<(), Box<dyn Error>> {
    let env = prelude_environment().expect("Unable to create prelude environment");

    // Loop
    loop {
        print!("blaadje> ");
        stdout().flush()?;

        // Read
        let mut input = String::new();
        stdin().read_line(&mut input)?;

        // Evaluate
        let output = run_with_env(&input, env.clone());

        // Print
        match output {
            Ok(v) => println!("\x1b[96m{:?}\x1b[0m", v),
            Err(v) => println!("\x1b[91mError: {:?}\x1b[0m", v),
        }
    }

    Ok(())
}
