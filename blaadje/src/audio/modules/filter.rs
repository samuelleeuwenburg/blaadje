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

    pub fn output(&self) -> Signal {
        self.output.signal()
    }

    pub fn set_input(&mut self, signal: Signal) -> &mut Self {
        self.input = signal;
        self
    }

    pub fn set_frequency(&mut self, frequency: Signal) -> &mut Self {
        self.frequency = frequency;
        self
    }

    pub fn set_resonance(&mut self, q: Signal) -> &mut Self {
        self.resonance = q;
        self
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
