use crate::core::args;
use crate::{eval, Blad, BladError, Environment, Literal};
use screech::Signal;
use std::cell::RefCell;
use std::rc::Rc;

mod channel;
mod engine;

pub use channel::{Channel, Message};
pub use engine::Engine;

#[derive(Debug, Clone, PartialEq)]
pub enum Screech {
    Oscillator(usize),
    Signal(Signal),
}

pub fn process_screech(
    operator: &str,
    list: &[Blad],
    env: Rc<RefCell<Environment>>,
) -> Result<Blad, BladError> {
    let rest = &list[0..list.len()];

    match operator {
        ":signal" => process_signal(rest, env.clone()),
        ":output" => process_output(rest, env.clone()),
        ":oscillator" => process_oscillator(rest, env.clone()),
        _ => Err(BladError::UndefinedOperator(operator.into())),
    }
}

fn process_signal(list: &[Blad], env: Rc<RefCell<Environment>>) -> Result<Blad, BladError> {
    args(list, 1)?;

    let result = eval(&list[0], env.clone())?;

    match &result {
        Blad::Screech(Screech::Oscillator(id)) => {
            match env.borrow_mut().channel_call(Message::GetSignal(*id)) {
                Message::Signal(signal) => Ok(Blad::Screech(Screech::Signal(signal))),
                msg => Err(BladError::UnexpectedMessage(msg)),
            }
        }
        _ => Err(BladError::ExpectedScreechModule(result)),
    }
}

fn process_output(list: &[Blad], env: Rc<RefCell<Environment>>) -> Result<Blad, BladError> {
    args(list, 2)?;

    let channel = eval(&list[0], env.clone())?;
    let signal = eval(&list[1], env.clone())?;

    match (&channel, &signal) {
        (Blad::Literal(Literal::Usize(channel)), Blad::Screech(Screech::Signal(signal))) => {
            env.borrow_mut()
                .channel_cast(Message::AddSignalToMainOut(*channel, *signal));
            Ok(Blad::Unit)
        }
        _ => Err(BladError::IncorrectArguments(vec![
            BladError::ExpectedUsize(channel),
            BladError::ExpectedScreechSignal(signal),
        ])),
    }
}

fn process_oscillator(list: &[Blad], env: Rc<RefCell<Environment>>) -> Result<Blad, BladError> {
    args(list, 1)?;

    let operator = eval(&list[0], env.clone())?;

    match &operator {
        Blad::Atom(o) if o == ":new" => {
            match env.borrow_mut().channel_call(Message::AddOscillator) {
                Message::ModuleId(id) => Ok(Blad::Screech(Screech::Oscillator(id))),
                msg => Err(BladError::UnexpectedMessage(msg)),
            }
        }
        Blad::Atom(o) if o == ":foo" => Ok(Blad::Unit),
        Blad::Atom(o) => Err(BladError::UndefinedOperator(o.into())),
        _ => Err(BladError::ExpectedAtom(operator)),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::run;

    #[test]
    fn parse_screech() {}
}
