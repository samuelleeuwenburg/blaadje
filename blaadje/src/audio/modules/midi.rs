use crate::core::args_min;
use crate::core::notes::midi_to_pitch;
use crate::{Blad, Error, Literal, Screech};
use screech::{Module, PatchPoint, Patchbay};
use std::convert::From;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub enum MidiMessage {
    NoteOn(u8, u8, u8),
    NoteOff(u8, u8, u8),
    TimingClock,
    Unknown,
}

impl From<u32> for MidiMessage {
    fn from(message: u32) -> Self {
        let channel = (message & 0xf) as u8;
        let message_type = (message >> 4) & 0xf;
        let lower = ((message >> 8) & 0xff) as u8;
        let upper = ((message >> 16) & 0xff) as u8;

        match (message_type, channel) {
            (0x8, _) => MidiMessage::NoteOff(channel, lower, upper),
            (0x9, _) => MidiMessage::NoteOn(channel, lower, upper),
            (0xf, 0x8) => MidiMessage::TimingClock,
            _ => MidiMessage::Unknown,
        }
    }
}

pub struct Voice {
    frequency: PatchPoint,
    gate: PatchPoint,
    active_note: Option<u8>,
}

impl Voice {
    pub fn new(frequency: PatchPoint, gate: PatchPoint) -> Self {
        Self {
            frequency,
            gate,
            active_note: None,
        }
    }
}

pub struct Midi {
    voices: Vec<Voice>,
    clock: PatchPoint,
    buffer: Arc<Mutex<Vec<u32>>>,
    channel: u8,
}

impl Midi {
    pub fn new(
        frequency_outs: Vec<PatchPoint>,
        gate_outs: Vec<PatchPoint>,
        clock: PatchPoint,
        buffer: Arc<Mutex<Vec<u32>>>,
    ) -> Self {
        let voices = frequency_outs
            .into_iter()
            .zip(gate_outs.into_iter())
            .map(|(a, b)| Voice::new(a, b))
            .collect();

        Self {
            voices,
            clock,
            buffer,
            channel: 0,
        }
    }

    pub fn set(&mut self, list: &[Blad]) -> Result<Blad, Error> {
        args_min(list, 1)?;

        for b in list.iter() {
            let pair = b.get_list()?;
            let property = pair[0].get_atom()?;
            let value = &pair[1];

            match (property, value) {
                (":channel", Blad::Literal(Literal::Usize(channel))) => {
                    self.channel = *channel as u8;
                    Ok(Blad::Unit)
                }
                (a, b) => Err(Error::IncorrectPropertyPair(a.to_string(), b.clone())),
            }?;
        }

        Ok(Blad::Unit)
    }

    pub fn reset(&mut self) {}

    pub fn get(&self, list: &[Blad]) -> Result<Blad, Error> {
        args_min(list, 1)?;
        let property = list[0].get_atom()?;

        match property {
            ":clock" => Ok(Blad::Screech(Screech::Signal(self.clock.signal()))),
            ":voices" => {
                let signals = self
                    .voices
                    .iter()
                    .map(|v| {
                        Blad::List(vec![
                            Blad::Screech(Screech::Signal(v.frequency.signal())),
                            Blad::Screech(Screech::Signal(v.gate.signal())),
                        ])
                    })
                    .collect();

                Ok(Blad::List(signals))
            }
            _ => Err(Error::InvalidProperty(property.into())),
        }
    }
}

impl<const SAMPLE_RATE: usize> Module<SAMPLE_RATE> for Midi {
    fn process<const P: usize>(&mut self, patchbay: &mut Patchbay<P>) {
        let messages = self.buffer.lock().unwrap();

        patchbay.set(&mut self.clock, 0.0);

        for v in self.voices.iter_mut() {
            let f = v.frequency.signal();
            let g = v.gate.signal();
            patchbay.set(&mut v.frequency, patchbay.get(f));
            patchbay.set(&mut v.gate, patchbay.get(g));
        }

        for message in messages.iter() {
            match MidiMessage::from(*message) {
                MidiMessage::TimingClock => patchbay.set(&mut self.clock, 1.0),
                MidiMessage::NoteOn(channel, note, _velocity) if channel == self.channel => {
                    for v in self.voices.iter_mut() {
                        if let None = v.active_note {
                            patchbay.set(&mut v.frequency, midi_to_pitch(note).unwrap_or(0.0));
                            patchbay.set(&mut v.gate, 1.0);
                            v.active_note = Some(note);
                            break;
                        }
                    }
                }
                MidiMessage::NoteOff(channel, note, _velocity) if channel == self.channel => {
                    for v in self.voices.iter_mut() {
                        if v.active_note == Some(note) {
                            patchbay.set(&mut v.frequency, midi_to_pitch(note).unwrap_or(0.0));
                            patchbay.set(&mut v.gate, 0.0);
                            v.active_note = None;
                            break;
                        }
                    }
                }
                _ => (),
            }
        }
    }
}
