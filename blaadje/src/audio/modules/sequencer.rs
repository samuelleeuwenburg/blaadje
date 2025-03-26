use crate::core::args_min;
use crate::{Blad, Error, Screech};
use screech::{Module, PatchPoint, Patchbay, Signal};

struct Step {
    pub frequency: f32,
    pub amplitude: f32,
    pub active: bool,
}

impl Step {
    fn new() -> Self {
        Step {
            frequency: 110.0,
            amplitude: 1.0,
            active: true,
        }
    }
}

pub struct Sequencer {
    trigger: Signal,
    frequency_output: PatchPoint,
    amplitude_output: PatchPoint,
    trigger_output: PatchPoint,
    steps: Vec<Step>,
    active_step: usize,
}

impl Sequencer {
    pub fn new(
        frequency_output: PatchPoint,
        amplitude_output: PatchPoint,
        trigger_output: PatchPoint,
    ) -> Self {
        Self {
            trigger: Signal::None,
            frequency_output,
            amplitude_output,
            trigger_output,
            steps: Vec::new(),
            active_step: 0,
        }
    }

    pub fn reset(&mut self) {
        self.trigger = Signal::None;
        self.steps = Vec::new();
    }

    pub fn set_frequencies(&mut self, values: Vec<f32>) -> &mut Self {
        for (i, v) in values.iter().enumerate() {
            match self.steps.get_mut(i) {
                Some(step) => step.frequency = *v,
                None => {
                    let mut step = Step::new();
                    step.frequency = *v;
                    self.steps.insert(i, step);
                }
            }
        }
        self
    }

    pub fn set_amplitudes(&mut self, values: Vec<f32>) -> &mut Self {
        for (i, v) in values.iter().enumerate() {
            match self.steps.get_mut(i) {
                Some(step) => step.amplitude = *v,
                None => {
                    let mut step = Step::new();
                    step.amplitude = *v;
                    self.steps.insert(i, step);
                }
            }
        }
        self
    }

    pub fn set_triggers(&mut self, values: Vec<bool>) -> &mut Self {
        for (i, v) in values.iter().enumerate() {
            match self.steps.get_mut(i) {
                Some(step) => step.active = *v,
                None => {
                    let mut step = Step::new();
                    step.active = *v;
                    self.steps.insert(i, step);
                }
            }
        }
        self
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
                (":frequencies", Blad::List(vs)) => {
                    let mut values = vec![];

                    for v in vs {
                        values.push(v.to_pitch()?);
                    }

                    self.set_frequencies(values);

                    Ok(Blad::Unit)
                }
                (":amplitudes", Blad::List(vs)) => {
                    let mut values = vec![];

                    for v in vs {
                        values.push(v.get_f32()?);
                    }

                    self.set_amplitudes(values);

                    Ok(Blad::Unit)
                }
                (":triggers", Blad::List(vs)) => {
                    let mut values = vec![];

                    for v in vs {
                        let value = if v.get_usize()? >= 1 { true } else { false };
                        values.push(value);
                    }

                    self.set_triggers(values);

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
            ":frequency_output" => Ok(Blad::Screech(Screech::Signal(
                self.frequency_output.signal(),
            ))),
            ":amplitude_output" => Ok(Blad::Screech(Screech::Signal(
                self.amplitude_output.signal(),
            ))),
            ":trigger_output" => Ok(Blad::Screech(Screech::Signal(self.trigger_output.signal()))),
            _ => Err(Error::InvalidProperty(property.into())),
        }
    }
}

impl<const SAMPLE_RATE: usize> Module<SAMPLE_RATE> for Sequencer {
    fn is_ready<const POINTS: usize>(&self, patchbay: &Patchbay<POINTS>) -> bool {
        patchbay.check(self.trigger)
    }

    fn process<const P: usize>(&mut self, patchbay: &mut Patchbay<P>) {
        let trigger_input = patchbay.get(self.trigger);

        if trigger_input > 0.0 {
            self.active_step += 1;
        }

        if self.active_step >= self.steps.len() {
            self.active_step = 0;
        }

        let (frequency, amplitude, active) = if self.steps.len() == 0 {
            (0.0, 0.0, false)
        } else {
            let step = self.steps.get(self.active_step).unwrap();
            (step.frequency, step.amplitude, step.active)
        };

        let trigger = if active { trigger_input } else { 0.0 };

        patchbay.set(&mut self.trigger_output, trigger);

        if active {
            patchbay.set(&mut self.frequency_output, frequency);
            patchbay.set(&mut self.amplitude_output, amplitude);
        }
    }
}
