use crate::core::args_min;
use crate::{Blad, Error, Literal, Screech};
use screech::{Module, PatchPoint, Patchbay, Signal};

const PI: f32 = 3.141;

enum Waveform {
    Sine,
    Saw,
    Triangle,
    Pulse(f32),
}

pub struct Oscillator {
    wave_shape: Waveform,
    frequency: Signal,
    amplitude: Signal,
    output: PatchPoint,
    value: f32,
}

impl Oscillator {
    pub fn new(output: PatchPoint) -> Self {
        Oscillator {
            wave_shape: Waveform::Sine,
            frequency: Signal::Fixed(220.0),
            amplitude: Signal::Fixed(0.1),
            output,
            value: 0.0,
        }
    }

    pub fn reset(&mut self) {
        self.wave_shape = Waveform::Sine;
        self.frequency = Signal::Fixed(220.0);
        self.amplitude = Signal::Fixed(0.1);
    }

    pub fn set(&mut self, list: &[Blad]) -> Result<Blad, Error> {
        args_min(list, 1)?;

        for b in list.iter() {
            let pair = b.get_list()?;
            let property = pair[0].get_atom()?;
            let value = &pair[1];

            match (property, value) {
                (":frequency", Blad::Literal(Literal::F32(f))) => {
                    self.frequency = Signal::Fixed(*f);
                    Ok(Blad::Unit)
                }
                (":frequency", Blad::Screech(Screech::Signal(signal))) => {
                    self.frequency = *signal;
                    Ok(Blad::Unit)
                }
                (":amplitude", Blad::Literal(Literal::F32(f))) => {
                    self.amplitude = Signal::Fixed(*f);
                    Ok(Blad::Unit)
                }
                (":amplitude", Blad::Screech(Screech::Signal(signal))) => {
                    self.amplitude = *signal;
                    Ok(Blad::Unit)
                }
                (":waveshape", Blad::Atom(string)) => {
                    match string.as_ref() {
                        ":pulse" => self.wave_shape = Waveform::Pulse(0.5),
                        ":sine" => self.wave_shape = Waveform::Sine,
                        ":triangle" => self.wave_shape = Waveform::Triangle,
                        ":saw" => self.wave_shape = Waveform::Saw,
                        _ => self.wave_shape = Waveform::Sine,
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

impl<const SAMPLE_RATE: usize> Module<SAMPLE_RATE> for Oscillator {
    fn is_ready<const POINTS: usize>(&self, patchbay: &Patchbay<POINTS>) -> bool {
        patchbay.check(self.frequency) && patchbay.check(self.amplitude)
    }

    fn process<const P: usize>(&mut self, patchbay: &mut Patchbay<P>) {
        // Ramp up from -1.0 to 1.0 based on the set `frequency`
        // then use this value to convert to the specific waveforms
        self.value += (1.0 / SAMPLE_RATE as f32) * patchbay.get(self.frequency);
        // Wrap around
        if self.value >= 1.0 {
            self.value -= 2.0;
        }

        // Create the desired waveform
        let wave = match self.wave_shape {
            Waveform::Saw => self.value,
            Waveform::Sine => sine(self.value),
            Waveform::Triangle => triangle(self.value),
            Waveform::Pulse(duty_cycle) => pulse(self.value, duty_cycle),
        };

        // Set the amplitude
        let output = wave * patchbay.get(self.amplitude);

        // Update the output value in the patchbay.
        patchbay.set(&mut self.output, output);
    }
}

// Bashkara approximation of a sine
fn sine(input: f32) -> f32 {
    // Calculate with positive values only
    let x = if input < 0.0 { -input * PI } else { input * PI };

    let numerator = 16.0 * x * (PI - x);
    let denominator = 5.0 * PI * PI - 4.0 * x * (PI - x);
    let sine = numerator / denominator;

    // Normalize back positive to negative if needed
    if input < 0.0 {
        -sine
    } else {
        sine
    }
}
fn triangle(input: f32) -> f32 {
    if input < 0.0 {
        (input + 1.0) * 2.0 - 1.0
    } else {
        (input * 2.0) * -1.0 + 1.0
    }
}

fn pulse(input: f32, duty_cycle: f32) -> f32 {
    // Normalize around the centerpoint
    let threshold = (duty_cycle * 2.0) - 1.0;

    if input >= threshold {
        1.0
    } else {
        -1.0
    }
}
