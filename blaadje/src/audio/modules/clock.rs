use screech::{Module, PatchPoint, Patchbay, Signal};

/// Pulse generator
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
        let step =
            ((patchbay.get(self.frequency) * (u32::MAX as f32)) / SAMPLE_RATE as f32 / 2.0) as u32;

        self.value = self.value.wrapping_add(step);

        let output = if self.value < step { 1.0 } else { 0.0 };

        patchbay.set(&mut self.output, output);
    }
}
