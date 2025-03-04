mod core;
mod prelude;

use core::{eval, parse, Environment};
pub use core::{Blad, BladError, Literal};
use prelude::prelude_environment;

pub fn run(code: &str) -> Result<Blad, BladError> {
    let program = parse(code)?;
    let env = prelude_environment()?;
    eval(&program, env)
}
