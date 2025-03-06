mod core;
mod prelude;

use std::cell::RefCell;
use std::rc::Rc;

pub use core::{eval, parse, Environment};
pub use core::{Blad, BladError, Literal};
pub use prelude::prelude_environment;

pub fn run(code: &str) -> Result<Blad, BladError> {
    let program = parse(code)?;
    let env = prelude_environment()?;
    eval(&program, env)
}

pub fn run_with_env(code: &str, env: Rc<RefCell<Environment>>) -> Result<Blad, BladError> {
    let program = parse(code)?;
    eval(&program, env)
}
