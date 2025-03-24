use screech::{Module, PatchPoint, Patchbay, Signal};

/// Pulse generator
pub struct Clock {
    output: PatchPoint,
    frequency: Signal,
    value: f32,
}

impl Clock {
    pub fn new(output: PatchPoint) -> Self {
        Self {
            output,
            frequency: Signal::None,
            value: 0.0,
        }
    }

    pub fn set_frequency(&mut self, frequency: Signal) -> &mut Self {
        self.frequency = frequency;
        self
    }

    pub fn set_bpm(&mut self, bpm: Signal) -> &mut Self {
        self.frequency = bpm.scale(0.016666666);
        self
    }

    pub fn get_frequency(&self) -> Signal {
        self.frequency
    }

    pub fn output(&self) -> Signal {
        self.output.signal()
    }
}

impl<const SAMPLE_RATE: usize> Module<SAMPLE_RATE> for Clock {
    fn process<const P: usize>(&mut self, patchbay: &mut Patchbay<P>) {
        self.value += (1.0 / SAMPLE_RATE as f32) * patchbay.get(self.frequency);

        let output = if self.value >= 2.0 {
            self.value -= 2.0;
            1.0
        } else {
            0.0
        };

        patchbay.set(&mut self.output, output);
    }
}
