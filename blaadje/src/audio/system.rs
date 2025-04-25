use crate::Channel;
use std::error::Error;
use std::sync::{Arc, Mutex};

pub trait System {
    fn get_hosts(&self) -> Vec<String>;
    fn get_default_host(&self) -> String;
    fn get_devices(&self, host: &str) -> Vec<String>;
    fn get_default_device(&self, host: &str) -> String;
    fn start_audio(
        &mut self,
        host_id: &str,
        device_id: &str,
        buffer_size: usize,
        sample_rate: usize,
        bit_depth: usize,
    ) -> Result<(), Box<dyn Error>>;
    fn stop_audio(&mut self);
    fn buffer_full(&self) -> bool;
    fn push_sample(&mut self, sample: f32);
    fn get_midi_channel(&self) -> Arc<Mutex<Channel>>;
    fn get_midi_inputs(&self) -> Vec<String>;
    fn get_midi_outputs(&self) -> Vec<String>;
    fn connect_midi_input(&mut self, id: &str);
    fn disconnect_midi_input(&mut self);
}
