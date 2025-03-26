use crate::core::args_min;
use crate::{Blad, Error, Literal, Screech};
use screech::{Module, PatchPoint, Patchbay, Signal};

pub struct Clock {
    output: PatchPoint,
    frequency: Signal,
    value: u32,
}

impl Clock {
    pub fn new(output: PatchPoint) -> Self {
        Self {
            output,
            frequency: Signal::None,
            value: 0,
        }
    }

    pub fn set_bpm(&mut self, bpm: Signal) -> &mut Self {
        self.frequency = bpm.scale(0.016666666);
        self
    }

    pub fn reset(&mut self) {
        self.frequency = Signal::None;
    }

    pub fn set(&mut self, list: &[Blad]) -> Result<Blad, Error> {
        args_min(list, 1)?;

        for b in list.iter() {
            let pair = b.get_list()?;
            let property = pair[0].get_atom()?;
            let value = &pair[1];

            match (property, value) {
                (":frequency", Blad::Screech(Screech::Signal(signal))) => {
                    self.frequency = *signal;
                    Ok(Blad::Unit)
                }
                (":frequency", Blad::Literal(Literal::F32(frequency))) => {
                    self.frequency = Signal::Fixed(*frequency);
                    Ok(Blad::Unit)
                }
                (":bpm", Blad::Screech(Screech::Signal(signal))) => {
                    self.set_bpm(*signal);
                    Ok(Blad::Unit)
                }
                (":bpm", Blad::Literal(Literal::F32(bpm))) => {
                    self.set_bpm(Signal::Fixed(*bpm));
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

impl<const SAMPLE_RATE: usize> Module<SAMPLE_RATE> for Clock {
    fn process<const P: usize>(&mut self, patchbay: &mut Patchbay<P>) {
        let step =
            ((patchbay.get(self.frequency) * (u32::MAX as f32)) / SAMPLE_RATE as f32 / 2.0) as u32;

        self.value = self.value.wrapping_add(step);

        let output = if self.value < step { 1.0 } else { 0.0 };

        patchbay.set(&mut self.output, output);
    }
}
