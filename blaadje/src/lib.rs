use std::sync::{Arc, Mutex};

mod audio;
mod core;
mod prelude;

pub use audio::Engine;
pub use core::{eval, parse, Blad, Channel, Environment, Error, Literal, Screech};
pub use prelude::set_prelude;

pub fn run(code: &str) -> Result<Blad, Error> {
    let (env, _) = Environment::new();
    let env = Arc::new(Mutex::new(env));

    let program = parse(code)?;

    set_prelude(env.clone())?;

    eval(&program, env.clone())
}

pub fn run_with_env(code: &str, env: Arc<Mutex<Environment>>) -> Result<Blad, Error> {
    let program = parse(code)?;
    eval(&program, env)
}
