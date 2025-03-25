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

    pub fn set_samples(&mut self, samples: Vec<f32>) -> &mut Self {
        self.samples = samples;
        self.position = 0;
        self
    }

    pub fn set_trigger(&mut self, trigger: Signal) -> &mut Self {
        self.trigger = trigger;
        self
    }

    pub fn set_oneshot(&mut self) -> &mut Self {
        self.mode = Mode::OneShot;
        self
    }

    pub fn set_loop(&mut self) -> &mut Self {
        self.mode = Mode::Loop;
        self
    }

    pub fn output(&self) -> Signal {
        self.output.signal()
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
