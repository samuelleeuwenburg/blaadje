use crate::core::args_min;
use crate::{Blad, Error, Literal, Screech};
use screech::{Module, PatchPoint, Patchbay, Signal};

const PI: f32 = 3.141;

pub struct Filter {
    input: Signal,
    frequency: Signal,
    resonance: Signal,
    output: PatchPoint,
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
}

impl Filter {
    pub fn new(output: PatchPoint) -> Self {
        Self {
            input: Signal::None,
            frequency: Signal::None,
            resonance: Signal::Fixed(1.8),
            output,
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
        }
    }

    pub fn reset(&mut self) {
        self.input = Signal::None;
        self.frequency = Signal::None;
        self.resonance = Signal::Fixed(1.8);
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
                (":frequency", Blad::Screech(Screech::Signal(signal))) => {
                    self.frequency = *signal;
                    Ok(Blad::Unit)
                }
                (":frequency", Blad::Literal(Literal::F32(frequency))) => {
                    self.frequency = Signal::Fixed(*frequency);
                    Ok(Blad::Unit)
                }
                (":resonance", Blad::Screech(Screech::Signal(signal))) => {
                    self.resonance = *signal;
                    Ok(Blad::Unit)
                }
                (":resonance", Blad::Literal(Literal::F32(q))) => {
                    self.resonance = Signal::Fixed(*q);
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

impl<const SAMPLE_RATE: usize> Module<SAMPLE_RATE> for Filter {
    fn is_ready<const POINTS: usize>(&self, patchbay: &Patchbay<POINTS>) -> bool {
        patchbay.check(self.input)
    }

    fn process<const P: usize>(&mut self, patchbay: &mut Patchbay<P>) {
        let input = patchbay.get(self.input);

        let omega = 2.0 * PI * patchbay.get(self.frequency) / SAMPLE_RATE as f32;
        let alpha = f32::sin(omega) / (2.0 * patchbay.get(self.resonance));
        let cos_omega = f32::cos(omega);

        // Biquad formula
        let norm = 1.0 / (1.0 + alpha);
        let a0 = (1.0 - cos_omega) / 2.0 * norm;
        let a1 = (1.0 - cos_omega) * norm;
        let a2 = a0;
        let b1 = -2.0 * cos_omega * norm;
        let b2 = (1.0 - alpha) * norm;

        let output = a0 * input + a1 * self.x1 + a2 * self.x2 - b1 * self.y1 - b2 * self.y2;

        patchbay.set(&mut self.output, output);

        self.x2 = self.x1;
        self.x1 = input;
        self.y2 = self.y1;
        self.y1 = output;
    }
}
