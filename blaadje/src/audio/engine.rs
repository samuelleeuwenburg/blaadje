use super::modules::{Clock, Filter, Oscillator, Sample, Sequencer, Vca};
use crate::core::{args, args_min};
use crate::{Blad, Channel, Error, Literal, Screech};
use screech::{Module, Patchbay, Processor, Signal};
use screech_macro::modularize;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[modularize]
enum Modules {
    Oscillator(Oscillator),
    Vca(Vca),
    Clock(Clock),
    Sequencer(Sequencer),
    Filter(Filter),
    Sample(Sample),
}

pub struct Engine<const SAMPLE_RATE: usize, const NUM_MODULES: usize, const NUM_PATCHES: usize> {
    module_ids: HashMap<String, usize>,
    patchbay: Patchbay<NUM_PATCHES>,
    processor: Processor<SAMPLE_RATE, NUM_MODULES, Modules>,
    outputs_left: Vec<Signal>,
    outputs_right: Vec<Signal>,
}

impl<const SAMPLE_RATE: usize, const NUM_MODULES: usize, const NUM_PATCHES: usize>
    Engine<SAMPLE_RATE, NUM_MODULES, NUM_PATCHES>
{
    pub fn new() -> Self {
        Self {
            module_ids: HashMap::new(),
            patchbay: Patchbay::new(),
            processor: Processor::empty(),
            outputs_left: Vec::new(),
            outputs_right: Vec::new(),
        }
    }

    pub fn process_channel(&mut self, channel: Arc<Mutex<Channel>>) {
        let messages = {
            let mut channel = channel.lock().unwrap();
            let messages = channel.messages().to_owned();
            channel.clear();
            messages
        };

        for m in messages {
            let reply = self.process_message(m);
            let mut channel = channel.lock().unwrap();
            channel.reply(reply);
        }
    }

    fn process_message(&mut self, message: Blad) -> Result<Blad, Error> {
        let list = message.get_list()?;
        args_min(list, 1)?;
        let operator = &list[0].get_atom()?;

        match operator.as_ref() {
            ":insert_module" => {
                args(&list, 3)?;
                let atom = &list[1].get_atom()?;
                let atom_id = &list[2].get_atom()?;

                let module = self
                    .atom_to_module(atom)
                    .ok_or(Error::UnknownModule(atom.to_string()))?;

                let id = match self.module_ids.get(*atom_id) {
                    Some(id) => *id,
                    None => {
                        let id = self.processor.insert_module(module).unwrap();
                        self.module_ids.insert(atom_id.to_string(), id);
                        id
                    }
                };

                Ok(Blad::Screech(Screech::Module(id)))
            }

            ":scale" => {
                args(&list, 3)?;
                let signal = &list[1].get_signal()?;
                let scale = &list[2].get_f32()?;
                let signal = signal.scale(*scale);

                Ok(Blad::Screech(Screech::Signal(signal)))
            }

            ":offset" => {
                args(&list, 3)?;
                let signal = &list[1].get_signal()?;
                let offset = &list[2].get_f32()?;
                let signal = signal.offset(*offset);

                Ok(Blad::Screech(Screech::Signal(signal)))
            }

            ":module" => {
                args(&list, 2)?;
                let id = &list[1].get_module()?;
                Ok(Blad::Atom(self.module_to_atom(*id).to_string()))
            }

            ":set" => {
                args_min(&list, 3)?;

                let id = match &list[1] {
                    Blad::Screech(Screech::Module(id)) => Ok(id),
                    Blad::Atom(atom_id) => self
                        .module_ids
                        .get(atom_id)
                        .ok_or(Error::ModuleAtomNotFound(atom_id.to_string())),
                    _ => Err(Error::ExpectedScreechModule(list[1].clone())),
                }?;

                let module = self
                    .processor
                    .get_module_mut(*id)
                    .ok_or(Error::ModuleNotFound(*id))?;

                let reply = match module {
                    Modules::Oscillator(m) => set_oscillator(m, &list[2..list.len()])?,
                    Modules::Vca(m) => set_vca(m, &list[2..list.len()])?,
                    Modules::Clock(m) => set_clock(m, &list[2..list.len()])?,
                    Modules::Sequencer(m) => set_sequencer(m, &list[2..list.len()])?,
                    Modules::Filter(m) => set_filter(m, &list[2..list.len()])?,
                    Modules::Sample(m) => set_sample(m, &list[2..list.len()])?,
                };

                Ok(reply)
            }

            ":get" => {
                args_min(&list, 3)?;

                let id = match &list[1] {
                    Blad::Screech(Screech::Module(id)) => Ok(id),
                    Blad::Atom(atom_id) => self
                        .module_ids
                        .get(atom_id)
                        .ok_or(Error::ModuleAtomNotFound(atom_id.to_string())),
                    _ => Err(Error::ExpectedScreechModule(list[1].clone())),
                }?;

                let module = self
                    .processor
                    .get_module_mut(*id)
                    .ok_or(Error::ModuleNotFound(*id))?;

                let reply = match module {
                    Modules::Oscillator(m) => get_oscillator(m, &list[2..list.len()])?,
                    Modules::Vca(m) => get_vca(m, &list[2..list.len()])?,
                    Modules::Clock(m) => get_clock(m, &list[2..list.len()])?,
                    Modules::Sequencer(m) => get_sequencer(m, &list[2..list.len()])?,
                    Modules::Filter(m) => get_filter(m, &list[2..list.len()])?,
                    Modules::Sample(m) => get_sample(m, &list[2..list.len()])?,
                };

                Ok(reply)
            }

            ":output_left" => {
                args(&list, 2)?;
                let signal = &list[1].get_signal()?;

                if !self.outputs_left.contains(signal) {
                    self.outputs_left.push(*signal);
                }

                Ok(Blad::Unit)
            }

            ":output_right" => {
                args(&list, 2)?;
                let signal = &list[1].get_signal()?;

                if !self.outputs_right.contains(signal) {
                    self.outputs_right.push(*signal);
                }

                Ok(Blad::Unit)
            }

            ":output_disconnect_all" => {
                self.outputs_left.clear();
                self.outputs_right.clear();

                Ok(Blad::Unit)
            }

            _ => Err(Error::UndefinedOperator(operator.to_string())),
        }
    }

    fn atom_to_module(&mut self, atom: &str) -> Option<Modules> {
        match atom {
            ":oscillator" => Some(Modules::Oscillator(Oscillator::new(
                self.patchbay.point().unwrap(),
            ))),
            ":filter" => Some(Modules::Filter(Filter::new(self.patchbay.point().unwrap()))),
            ":vca" => Some(Modules::Vca(Vca::new(self.patchbay.point().unwrap()))),
            ":sample" => Some(Modules::Sample(Sample::new(self.patchbay.point().unwrap()))),
            ":clock" => Some(Modules::Clock(Clock::new(self.patchbay.point().unwrap()))),
            ":sequencer" => Some(Modules::Sequencer(Sequencer::new(
                self.patchbay.point().unwrap(),
            ))),
            _ => None,
        }
    }

    fn module_to_atom(&self, id: usize) -> &str {
        match self.processor.get_module(id) {
            Some(Modules::Clock(_)) => ":clock",
            Some(Modules::Filter(_)) => ":filter",
            Some(Modules::Oscillator(_)) => ":oscillator",
            Some(Modules::Sample(_)) => ":sample",
            Some(Modules::Sequencer(_)) => ":sequencer",
            Some(Modules::Vca(_)) => ":vca",
            None => ":none",
        }
    }

    pub fn next_samples(&mut self) -> (f32, f32) {
        self.processor.process_modules(&mut self.patchbay);

        let mut left = 0.0;
        let mut right = 0.0;

        for signal in self.outputs_left.iter() {
            left += self.patchbay.get(*signal);
        }

        for signal in self.outputs_right.iter() {
            right += self.patchbay.get(*signal);
        }

        (left, right)
    }
}

fn set_oscillator(osc: &mut Oscillator, list: &[Blad]) -> Result<Blad, Error> {
    args_min(list, 1)?;

    for b in list.iter() {
        let pair = b.get_list()?;
        let property = pair[0].get_atom()?;
        let value = &pair[1];

        match (property, value) {
            (":frequency", Blad::Literal(Literal::F32(f))) => {
                osc.set_frequency(Signal::Fixed(*f));
                Ok(Blad::Unit)
            }
            (":frequency", Blad::Screech(Screech::Signal(signal))) => {
                osc.set_frequency(*signal);
                Ok(Blad::Unit)
            }
            (":amplitude", Blad::Literal(Literal::F32(f))) => {
                osc.set_amplitude(Signal::Fixed(*f));
                Ok(Blad::Unit)
            }
            (":amplitude", Blad::Screech(Screech::Signal(signal))) => {
                osc.set_amplitude(*signal);
                Ok(Blad::Unit)
            }
            (":waveshape", Blad::Atom(string)) => {
                match string.as_ref() {
                    ":pulse" => osc.output_pulse(0.5),
                    ":sine" => osc.output_sine(),
                    ":triangle" => osc.output_triangle(),
                    ":saw" => osc.output_saw(),
                    _ => osc.output_sine(),
                };
                Ok(Blad::Unit)
            }
            (a, b) => Err(Error::IncorrectPropertyPair(a.to_string(), b.clone())),
        }?;
    }

    Ok(Blad::Unit)
}

fn get_oscillator(osc: &mut Oscillator, list: &[Blad]) -> Result<Blad, Error> {
    args_min(list, 1)?;
    let property = list[0].get_atom()?;

    match property {
        ":frequency" => Ok(Blad::Screech(Screech::Signal(osc.get_frequency()))),
        ":amplitude" => Ok(Blad::Screech(Screech::Signal(osc.get_amplitude()))),
        ":output" => Ok(Blad::Screech(Screech::Signal(osc.output()))),
        _ => Err(Error::InvalidProperty(property.into())),
    }
}

fn set_vca(m: &mut Vca, list: &[Blad]) -> Result<Blad, Error> {
    args_min(list, 1)?;

    for b in list.iter() {
        let pair = b.get_list()?;
        let property = pair[0].get_atom()?;
        let value = &pair[1];

        match (property, value) {
            (":input", Blad::Screech(Screech::Signal(signal))) => {
                m.set_input(*signal);
                Ok(Blad::Unit)
            }
            (":modulator", Blad::Screech(Screech::Signal(signal))) => {
                m.set_modulator(*signal);
                Ok(Blad::Unit)
            }
            (a, b) => Err(Error::IncorrectPropertyPair(a.to_string(), b.clone())),
        }?;
    }

    Ok(Blad::Unit)
}

fn get_vca(vca: &mut Vca, list: &[Blad]) -> Result<Blad, Error> {
    args_min(list, 1)?;
    let property = list[0].get_atom()?;

    match property {
        ":output" => Ok(Blad::Screech(Screech::Signal(vca.output()))),
        _ => Err(Error::InvalidProperty(property.into())),
    }
}

fn set_clock(m: &mut Clock, list: &[Blad]) -> Result<Blad, Error> {
    args_min(list, 1)?;

    for b in list.iter() {
        let pair = b.get_list()?;
        let property = pair[0].get_atom()?;
        let value = &pair[1];

        match (property, value) {
            (":frequency", Blad::Screech(Screech::Signal(signal))) => {
                m.set_frequency(*signal);
                Ok(Blad::Unit)
            }
            (":frequency", Blad::Literal(Literal::F32(frequency))) => {
                m.set_frequency(Signal::Fixed(*frequency));
                Ok(Blad::Unit)
            }
            (":bpm", Blad::Screech(Screech::Signal(signal))) => {
                m.set_bpm(*signal);
                Ok(Blad::Unit)
            }
            (":bpm", Blad::Literal(Literal::F32(bpm))) => {
                m.set_bpm(Signal::Fixed(*bpm));
                Ok(Blad::Unit)
            }
            (a, b) => Err(Error::IncorrectPropertyPair(a.to_string(), b.clone())),
        }?;
    }

    Ok(Blad::Unit)
}

fn get_clock(clock: &mut Clock, list: &[Blad]) -> Result<Blad, Error> {
    args_min(list, 1)?;
    let property = list[0].get_atom()?;

    match property {
        ":output" => Ok(Blad::Screech(Screech::Signal(clock.output()))),
        ":frequency" => Ok(Blad::Screech(Screech::Signal(clock.get_frequency()))),
        _ => Err(Error::InvalidProperty(property.into())),
    }
}

fn set_sequencer(m: &mut Sequencer, list: &[Blad]) -> Result<Blad, Error> {
    args_min(list, 1)?;

    for b in list.iter() {
        let pair = b.get_list()?;
        let property = pair[0].get_atom()?;
        let value = &pair[1];

        match (property, value) {
            (":clock", Blad::Screech(Screech::Signal(signal))) => {
                m.set_clock(*signal);
                Ok(Blad::Unit)
            }
            (":steps", Blad::List(atoms)) => {
                let mut steps = vec![];

                for atom in atoms {
                    steps.push(atom.to_pitch()?);
                }

                m.set_steps(steps);

                Ok(Blad::Unit)
            }
            (a, b) => Err(Error::IncorrectPropertyPair(a.to_string(), b.clone())),
        }?;
    }

    Ok(Blad::Unit)
}

fn get_sequencer(sequencer: &mut Sequencer, list: &[Blad]) -> Result<Blad, Error> {
    args_min(list, 1)?;
    let property = list[0].get_atom()?;

    match property {
        ":frequency_output" => Ok(Blad::Screech(Screech::Signal(sequencer.frequency_output()))),
        _ => Err(Error::InvalidProperty(property.into())),
    }
}

fn set_filter(m: &mut Filter, list: &[Blad]) -> Result<Blad, Error> {
    args_min(list, 1)?;

    for b in list.iter() {
        let pair = b.get_list()?;
        let property = pair[0].get_atom()?;
        let value = &pair[1];

        match (property, value) {
            (":input", Blad::Screech(Screech::Signal(signal))) => {
                m.set_input(*signal);
                Ok(Blad::Unit)
            }
            (":frequency", Blad::Screech(Screech::Signal(signal))) => {
                m.set_frequency(*signal);
                Ok(Blad::Unit)
            }
            (":frequency", Blad::Literal(Literal::F32(frequency))) => {
                m.set_frequency(Signal::Fixed(*frequency));
                Ok(Blad::Unit)
            }
            (":resonance", Blad::Screech(Screech::Signal(signal))) => {
                m.set_resonance(*signal);
                Ok(Blad::Unit)
            }
            (":resonance", Blad::Literal(Literal::F32(q))) => {
                m.set_resonance(Signal::Fixed(*q));
                Ok(Blad::Unit)
            }
            (a, b) => Err(Error::IncorrectPropertyPair(a.to_string(), b.clone())),
        }?;
    }

    Ok(Blad::Unit)
}

fn get_filter(m: &mut Filter, list: &[Blad]) -> Result<Blad, Error> {
    args_min(list, 1)?;
    let property = list[0].get_atom()?;

    match property {
        ":output" => Ok(Blad::Screech(Screech::Signal(m.output()))),
        _ => Err(Error::InvalidProperty(property.into())),
    }
}

fn set_sample(m: &mut Sample, list: &[Blad]) -> Result<Blad, Error> {
    args_min(list, 1)?;

    for b in list.iter() {
        let pair = b.get_list()?;
        let property = pair[0].get_atom()?;
        let value = &pair[1];

        match (property, value) {
            (":trigger", Blad::Screech(Screech::Signal(signal))) => {
                m.set_trigger(*signal);
                Ok(Blad::Unit)
            }
            (":samples", Blad::List(s)) => {
                let mut samples = vec![];

                for sample in s {
                    samples.push(sample.get_f32()?);
                }

                m.set_samples(samples);

                Ok(Blad::Unit)
            }
            (":mode", Blad::Atom(string)) => {
                match string.as_ref() {
                    ":oneshot" => m.set_oneshot(),
                    ":loop" => m.set_loop(),
                    _ => m.set_oneshot(),
                };
                Ok(Blad::Unit)
            }
            (a, b) => Err(Error::IncorrectPropertyPair(a.to_string(), b.clone())),
        }?;
    }

    Ok(Blad::Unit)
}

fn get_sample(m: &mut Sample, list: &[Blad]) -> Result<Blad, Error> {
    args_min(list, 1)?;
    let property = list[0].get_atom()?;

    match property {
        ":output" => Ok(Blad::Screech(Screech::Signal(m.output()))),
        _ => Err(Error::InvalidProperty(property.into())),
    }
}
