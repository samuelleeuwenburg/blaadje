use blaadje::{Blad, Channel, Literal, System};
use cpal::{
    available_hosts, default_host, host_from_id,
    traits::{DeviceTrait, HostTrait},
    Stream,
};
use midir::{Ignore, MidiInput, MidiInputPort, MidiOutput};
use ringbuf::{
    storage::Heap,
    traits::{Consumer, Observer, Producer, Split},
    wrap::caching::Caching,
    HeapRb, SharedRb,
};
use std::error::Error;
use std::sync::{Arc, Mutex};

pub struct Sys {
    stream: Option<Stream>,
    buffer: Option<Caching<Arc<SharedRb<Heap<f32>>>, true, false>>,
    midi_in_port: Option<MidiInputPort>,
    midi_channel: Arc<Mutex<Channel>>,
}

impl Sys {
    pub fn new() -> Self {
        // let midi_in = midi(midi_channel.clone())?;

        Self {
            stream: None,
            buffer: None,
            midi_in_port: None,
            midi_channel: Arc::new(Mutex::new(Channel::new())),
        }
    }
}

impl System for Sys {
    fn get_hosts(&self) -> Vec<String> {
        available_hosts()
            .into_iter()
            .map(|h| h.name().to_string())
            .collect()
    }

    fn get_default_host(&self) -> String {
        default_host().id().name().to_string()
    }

    fn get_devices(&self, host_id: &str) -> Vec<String> {
        let host_id = available_hosts()
            .into_iter()
            .find(|h| h.name() == host_id)
            .unwrap();

        let host = host_from_id(host_id).unwrap();
        host.devices()
            .unwrap()
            .into_iter()
            .map(|d| d.name().unwrap().to_string())
            .collect()
    }

    fn get_default_device(&self, host: &str) -> String {
        let host_id = available_hosts()
            .into_iter()
            .find(|h| h.name() == host)
            .unwrap();

        let host = host_from_id(host_id).unwrap();

        host.default_output_device()
            .unwrap()
            .name()
            .unwrap()
            .to_string()
    }

    fn start_audio(
        &mut self,
        host_id: &str,
        device_id: &str,
        buffer_size: usize,
        _sample_rate: usize,
        _bit_depth: usize,
    ) -> Result<(), Box<dyn Error>> {
        let host_id = available_hosts()
            .into_iter()
            .find(|h| h.name() == host_id)
            .unwrap();

        let host = host_from_id(host_id)?;
        let device = host
            .devices()?
            .into_iter()
            .find(|d| d.name().unwrap() == device_id)
            .unwrap();

        let config = device.default_output_config().unwrap();
        let mut config: cpal::StreamConfig = config.into();
        config.buffer_size = cpal::BufferSize::Fixed(buffer_size as u32);

        // The buffer to share samples
        let ring = HeapRb::<f32>::new(buffer_size * 2);
        let (producer, mut consumer) = ring.split();

        let output_data_fn = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for sample in data {
                *sample = match consumer.try_pop() {
                    Some(s) => s,
                    None => 0.0,
                };
            }
        };

        let stream = device.build_output_stream(&config, output_data_fn, err_fn, None)?;
        self.stream = Some(stream);
        self.buffer = Some(producer);

        Ok(())
    }

    fn stop_audio(&mut self) {
        self.stream = None;
        self.buffer = None;
    }

    fn buffer_full(&self) -> bool {
        if let Some(buffer) = &self.buffer {
            buffer.is_full()
        } else {
            false
        }
    }

    fn push_sample(&mut self, sample: f32) {
        if let Some(buffer) = &mut self.buffer {
            buffer.try_push(sample).unwrap();
        }
    }

    fn get_midi_inputs(&self) -> Vec<String> {
        let midi_in = MidiInput::new("midir scan input").unwrap();
        midi_in
            .ports()
            .iter()
            .map(|p| midi_in.port_name(p).unwrap().to_string())
            .collect()
    }
    fn get_midi_outputs(&self) -> Vec<String> {
        let midi_out = MidiOutput::new("midir scan output").unwrap();
        midi_out
            .ports()
            .iter()
            .map(|p| midi_out.port_name(p).unwrap().to_string())
            .collect()
    }
    fn connect_midi_input(&mut self, id: &str) {
        let mut midi_in = MidiInput::new("midir input").unwrap();
        midi_in.ignore(Ignore::None);
        let port = midi_in
            .ports()
            .into_iter()
            .find(|p| midi_in.port_name(p).unwrap() == id)
            .unwrap();

        let channel = self.midi_channel.clone();

        midi_in
            .connect(
                &port,
                "midir_read_input",
                move |_, message, _| {
                    let mut midi_message = 0_usize;

                    for (i, byte) in message.iter().enumerate() {
                        midi_message += (*byte as usize) << i * 8;
                    }

                    let mut channel = channel.lock().unwrap();

                    channel.send(Blad::List(vec![
                        Blad::Atom(":midi".to_string()),
                        Blad::Literal(Literal::Usize(midi_message)),
                    ]));
                },
                (),
            )
            .unwrap();

        self.midi_in_port = Some(port);
    }

    fn disconnect_midi_input(&mut self) {
        self.midi_in_port = None;
    }

    fn get_midi_channel(&self) -> Arc<Mutex<Channel>> {
        self.midi_channel.clone()
    }
}

fn err_fn(err: cpal::StreamError) {
    eprintln!("an error occurred on stream: {}", err);
}
