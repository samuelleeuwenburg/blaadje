use crate::core::args_min;
use crate::{Blad, Error, Screech};
use screech::{Module, PatchPoint, Patchbay, Signal};

enum Mode {
    OneShot,
    Loop,
}

pub struct Sample {
    trigger: Signal,
    output: PatchPoint,
    samples: Vec<f32>,
    position: usize,
    mode: Mode,
    active: bool,
}

impl Sample {
    pub fn new(output: PatchPoint) -> Self {
        Self {
            trigger: Signal::None,
            output,
            position: 0,
            samples: Vec::new(),
            mode: Mode::OneShot,
            active: false,
        }
    }

    pub fn reset(&mut self) {
        self.trigger = Signal::None;
        self.mode = Mode::OneShot;
    }

    pub fn set(&mut self, list: &[Blad]) -> Result<Blad, Error> {
        args_min(list, 1)?;

        for b in list.iter() {
            let pair = b.get_list()?;
            let property = pair[0].get_atom()?;
            let value = &pair[1];

            match (property, value) {
                (":trigger", Blad::Screech(Screech::Signal(signal))) => {
                    self.trigger = *signal;
                    Ok(Blad::Unit)
                }
                (":samples", Blad::List(s)) => {
                    let mut samples = vec![];

                    for sample in s {
                        samples.push(sample.get_f32()?);
                    }

                    self.samples = samples;

                    Ok(Blad::Unit)
                }
                (":mode", Blad::Atom(string)) => {
                    match string.as_ref() {
                        ":oneshot" => self.mode = Mode::OneShot,
                        ":loop" => self.mode = Mode::Loop,
                        _ => self.mode = Mode::OneShot,
                    };
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

impl<const SAMPLE_RATE: usize> Module<SAMPLE_RATE> for Sample {
    fn is_ready<const POINTS: usize>(&self, patchbay: &Patchbay<POINTS>) -> bool {
        patchbay.check(self.trigger)
    }

    fn process<const P: usize>(&mut self, patchbay: &mut Patchbay<P>) {
        if patchbay.get(self.trigger) > 0.0 {
            self.position = 0;
            self.active = true;
        }

        if self.position >= self.samples.len() {
            self.position = 0;

            self.active = match self.mode {
                Mode::Loop => true,
                Mode::OneShot => false,
            }
        };

        let output = match (self.samples.len(), self.active) {
            (0, _) => 0.0,
            (_, true) => {
                let sample = self.samples[self.position];
                self.position += 1;
                sample
            }
            _ => 0.0,
        };

        patchbay.set(&mut self.output, output);
    }
}
