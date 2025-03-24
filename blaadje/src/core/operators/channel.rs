use super::super::{args, eval};
use crate::{Blad, Environment, Error};
use std::sync::{Arc, Mutex};

pub fn process_call(list: &[Blad], env: Arc<Mutex<Environment>>) -> Result<Blad, Error> {
    args(list, 1)?;

    let message = eval(&list[0], env.clone())?;
    let mut env = env.lock().unwrap();

    env.channel_call(message)
}

pub fn process_cast(list: &[Blad], env: Arc<Mutex<Environment>>) -> Result<Blad, Error> {
    args(list, 1)?;

    let message = eval(&list[0], env.clone())?;
    let mut env = env.lock().unwrap();

    env.channel_cast(message);

    Ok(Blad::Unit)
}
