use screech::{Module, PatchPoint, Patchbay, Signal};

pub struct Sequencer {
    clock: Signal,
    frequency_output: PatchPoint,
    steps: Vec<f32>,
    active_step: usize,
}

impl Sequencer {
    pub fn new(frequency_output: PatchPoint) -> Self {
        Self {
            clock: Signal::None,
            frequency_output,
            steps: Vec::new(),
            active_step: 0,
        }
    }

    pub fn set_clock(&mut self, clock: Signal) -> &mut Self {
        self.clock = clock;
        self
    }

    pub fn set_steps(&mut self, steps: Vec<f32>) -> &mut Self {
        self.steps = steps;

        if self.active_step > self.steps.len() {
            self.active_step = 0;
        }

        self
    }

    pub fn frequency_output(&self) -> Signal {
        self.frequency_output.signal()
    }
}

impl<const SAMPLE_RATE: usize> Module<SAMPLE_RATE> for Sequencer {
    fn is_ready<const POINTS: usize>(&self, patchbay: &Patchbay<POINTS>) -> bool {
        patchbay.check(self.clock)
    }

    fn process<const P: usize>(&mut self, patchbay: &mut Patchbay<P>) {
        if patchbay.get(self.clock) > 0.0 {
            self.active_step += 1;
        }

        if self.active_step >= self.steps.len() {
            self.active_step = 0;
        }

        let output = if self.steps.len() == 0 {
            0.0
        } else {
            *self.steps.get(self.active_step).unwrap()
        };

        patchbay.set(&mut self.frequency_output, output);
    }
}
