use crate::core::args_min;
use crate::{Blad, Error, Screech};
use screech::{Module, PatchPoint, Patchbay, Signal};

/// VCA module that takes two inputs (signal and modulator) and has a single output.
pub struct Vca {
    modulator: Signal,
    input: Signal,
    output: PatchPoint,
}

impl Vca {
    pub fn new(output: PatchPoint) -> Self {
        Vca {
            modulator: Signal::None,
            input: Signal::None,
            output,
        }
    }

    pub fn reset(&mut self) {
        self.modulator = Signal::None;
        self.input = Signal::None;
    }

    pub fn set(&mut self, list: &[Blad]) -> Result<Blad, Error> {
        args_min(list, 1)?;

        for b in list.iter() {
            let pair = b.get_list()?;
            let property = pair[0].get_atom()?;
            let value = &pair[1];

            match (property, value) {
                (":input", Blad::Screech(Screech::Signal(signal))) => {
                    self.input = *signal;
                    Ok(Blad::Unit)
                }
                (":modulator", Blad::Screech(Screech::Signal(signal))) => {
                    self.modulator = *signal;
                    Ok(Blad::Unit)
                }
                (a, b) => Err(Error::IncorrectPropertyPair(a.to_string(), b.clone())),
            }?;
        }

        Ok(Blad::Unit)
    }

    pub fn get(&self, list: &[Blad]) -> Result<Blad, Error> {
        args_min(list, 1)?;
        let property = list[0].get_atom()?;

        match property {
            ":output" => Ok(Blad::Screech(Screech::Signal(self.output.signal()))),
            _ => Err(Error::InvalidProperty(property.into())),
        }
    }
}

impl<const SAMPLE_RATE: usize> Module<SAMPLE_RATE> for Vca {
    fn is_ready<const POINTS: usize>(&self, patchbay: &Patchbay<POINTS>) -> bool {
        patchbay.check(self.input) && patchbay.check(self.modulator)
    }

    fn process<const P: usize>(&mut self, patchbay: &mut Patchbay<P>) {
        // Take the input signal and multiply it by the modulator input.
        patchbay.set(
            &mut self.output,
            patchbay.get(self.input) * patchbay.get(self.modulator),
        );
    }
}
