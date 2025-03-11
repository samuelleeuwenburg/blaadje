use super::{Channel, Message};
use screech::modules::{Mix, Oscillator, Vca};
use screech::{Module, Patchbay, Processor, Signal};
use screech_macro::modularize;
use std::sync::{Arc, Mutex};

#[modularize]
enum Modules {
    Oscillator(Oscillator),
    Vca(Vca),
    Mix(Mix),
}

pub struct Engine<const SAMPLE_RATE: usize, const NUM_MODULES: usize, const NUM_PATCHES: usize> {
    patchbay: Patchbay<NUM_PATCHES>,
    processor: Processor<SAMPLE_RATE, NUM_MODULES, Modules>,
    left: (Signal, usize),
    right: (Signal, usize),
}

impl<const SAMPLE_RATE: usize, const NUM_MODULES: usize, const NUM_PATCHES: usize>
    Engine<SAMPLE_RATE, NUM_MODULES, NUM_PATCHES>
{
    pub fn new() -> Self {
        let mut patchbay = Patchbay::new();
        let mut processor = Processor::empty();

        let mix_left = Mix::new(patchbay.point().unwrap());
        let mix_right = Mix::new(patchbay.point().unwrap());
        let point_left = mix_left.output();
        let point_right = mix_right.output();

        let left = processor.insert_module(Modules::Mix(mix_left)).unwrap();
        let right = processor.insert_module(Modules::Mix(mix_right)).unwrap();

        Self {
            patchbay,
            processor,
            left: (point_left, left),
            right: (point_right, right),
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
            match m {
                Message::AddOscillator => {
                    let osc = Oscillator::new(self.patchbay.point().unwrap());

                    let id = self
                        .processor
                        .insert_module(Modules::Oscillator(osc))
                        .unwrap();

                    let mut channel = channel.lock().unwrap();
                    channel.reply(Message::ModuleId(id))
                }
                Message::AddSignalToMainOut(channel, signal) => {
                    let (_, mix_l) = self.left;
                    let (_, mix_r) = self.left;

                    if let Some(Modules::Mix(m)) = self.processor.get_module_mut(mix_l) {
                        m.add_input(signal, channel);
                    }

                    if let Some(Modules::Mix(m)) = self.processor.get_module_mut(mix_r) {
                        m.add_input(signal, channel);
                    }
                }
                Message::GetSignal(id) => {
                    let message = match self.processor.get_module(id) {
                        Some(Modules::Oscillator(m)) => Message::Signal(m.output()),
                        Some(Modules::Vca(m)) => Message::Signal(m.output()),
                        Some(Modules::Mix(m)) => Message::Signal(m.output()),
                        None => Message::ModuleNotFound,
                    };

                    let mut channel = channel.lock().unwrap();
                    channel.reply(message)
                }
                message => println!("unhandled message type: {:?}", message),
            }
        }
    }

    pub fn next_samples(&mut self) -> (f32, f32) {
        self.processor.process_modules(&mut self.patchbay);

        let (l, _) = self.left;
        let (r, _) = self.right;

        (self.patchbay.get(l), self.patchbay.get(r))
    }
}
