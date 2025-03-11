mod audio;
mod core;
mod prelude;

use std::cell::RefCell;
use std::rc::Rc;

pub use audio::{Channel, Engine, Message};
pub use core::{eval, parse, Environment};
pub use core::{Blad, BladError, Literal};
pub use prelude::set_prelude;

pub fn run(code: &str) -> Result<Blad, BladError> {
    let (env, _) = Environment::new();
    let env = Rc::new(RefCell::new(env));

    let program = parse(code)?;

    set_prelude(env.clone())?;

    eval(&program, env.clone())
}

pub fn run_with_env(code: &str, env: Rc<RefCell<Environment>>) -> Result<Blad, BladError> {
    let program = parse(code)?;
    eval(&program, env)
}
