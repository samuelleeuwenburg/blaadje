use super::modules::{Clock, Filter, Midi, Oscillator, Sample, Sequencer, Vca};
use crate::core::{args, args_min};
use crate::{Blad, Channel, Error, Screech};
use screech::{Module, Patchbay, Processor, Signal};
use screech_macro::modularize;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[modularize]
enum Modules {
    Clock(Clock),
    Filter(Filter),
    Midi(Midi),
    Oscillator(Oscillator),
    Sample(Sample),
    Sequencer(Sequencer),
    Vca(Vca),
}

impl Modules {
    fn reset(&mut self) {
        match self {
            Modules::Clock(m) => m.reset(),
            Modules::Filter(m) => m.reset(),
            Modules::Midi(m) => m.reset(),
            Modules::Oscillator(m) => m.reset(),
            Modules::Sample(m) => m.reset(),
            Modules::Sequencer(m) => m.reset(),
            Modules::Vca(m) => m.reset(),
        }
    }

    fn set(&mut self, list: &[Blad]) -> Result<Blad, Error> {
        match self {
            Modules::Clock(m) => m.set(list),
            Modules::Filter(m) => m.set(list),
            Modules::Midi(m) => m.set(list),
            Modules::Oscillator(m) => m.set(list),
            Modules::Sample(m) => m.set(list),
            Modules::Sequencer(m) => m.set(list),
            Modules::Vca(m) => m.set(list),
        }
    }

    fn get(&self, list: &[Blad]) -> Result<Blad, Error> {
        match self {
            Modules::Clock(m) => m.get(list),
            Modules::Filter(m) => m.get(list),
            Modules::Midi(m) => m.get(list),
            Modules::Oscillator(m) => m.get(list),
            Modules::Sample(m) => m.get(list),
            Modules::Sequencer(m) => m.get(list),
            Modules::Vca(m) => m.get(list),
        }
    }
}

pub struct Engine<const SAMPLE_RATE: usize, const NUM_MODULES: usize, const NUM_PATCHES: usize> {
    module_ids: HashMap<String, usize>,
    patchbay: Patchbay<NUM_PATCHES>,
    processor: Processor<SAMPLE_RATE, NUM_MODULES, Modules>,
    outputs_left: Vec<Signal>,
    outputs_right: Vec<Signal>,
    midi_buffer: Arc<Mutex<Vec<u32>>>,
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
            midi_buffer: Arc::new(Mutex::new(Vec::new())),
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
            ":midi" => {
                args(&list, 2)?;
                let message = &list[1].get_usize()?;

                let mut midi_buffer = self.midi_buffer.lock().unwrap();
                midi_buffer.push(*message as u32);

                Ok(Blad::Unit)
            }
            ":insert_module" => {
                args(&list, 3)?;
                let atom = &list[1].get_atom()?;
                let string_id = &list[2].get_string()?;

                let id = match self.module_ids.get(*string_id) {
                    Some(id) => {
                        let module = self.processor.get_module_mut(*id).unwrap();

                        module.reset();

                        *id
                    }
                    None => {
                        let module = self
                            .atom_to_module(atom)
                            .ok_or(Error::UnknownModule(atom.to_string()))?;

                        let id = self.processor.insert_module(module).unwrap();
                        self.module_ids.insert(string_id.to_string(), id);
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

                let id = &list[1].get_module()?;

                let module = self
                    .processor
                    .get_module_mut(*id)
                    .ok_or(Error::ModuleNotFound(*id))?;

                module.set(&list[2..list.len()])
            }

            ":get" => {
                args_min(&list, 3)?;

                let id = &list[1].get_module()?;

                let module = self
                    .processor
                    .get_module_mut(*id)
                    .ok_or(Error::ModuleNotFound(*id))?;

                module.get(&list[2..list.len()])
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
            ":midi" => {
                let voices = 8;
                let frequencies = (0..voices)
                    .map(|_| self.patchbay.point().unwrap())
                    .collect();
                let gates = (0..voices)
                    .map(|_| self.patchbay.point().unwrap())
                    .collect();

                Some(Modules::Midi(Midi::new(
                    frequencies,
                    gates,
                    self.patchbay.point().unwrap(),
                    self.midi_buffer.clone(),
                )))
            }
            ":sequencer" => Some(Modules::Sequencer(Sequencer::new(
                self.patchbay.point().unwrap(),
                self.patchbay.point().unwrap(),
                self.patchbay.point().unwrap(),
            ))),
            _ => None,
        }
    }

    fn module_to_atom(&self, id: usize) -> &str {
        match self.processor.get_module(id) {
            Some(Modules::Clock(_)) => ":clock",
            Some(Modules::Filter(_)) => ":filter",
            Some(Modules::Midi(_)) => ":midi",
            Some(Modules::Oscillator(_)) => ":oscillator",
            Some(Modules::Sample(_)) => ":sample",
            Some(Modules::Sequencer(_)) => ":sequencer",
            Some(Modules::Vca(_)) => ":vca",
            None => ":none",
        }
    }

    pub fn next_samples(&mut self) -> (f32, f32) {
        self.processor.process_modules(&mut self.patchbay);

        {
            self.midi_buffer.lock().unwrap().clear();
        }

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
