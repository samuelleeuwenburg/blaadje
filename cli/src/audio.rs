use cpal::{
    traits::{DeviceTrait, HostTrait},
    Stream,
};
use ringbuf::{
    storage::Heap,
    traits::{Consumer, Split},
    wrap::caching::Caching,
    HeapRb, SharedRb,
};
use std::error::Error;
use std::sync::Arc;

pub fn init_stream(
) -> Result<(Stream, Caching<Arc<SharedRb<Heap<f32>>>, true, false>), Box<dyn Error>> {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .expect("failed to find output device");

    let config = device.default_output_config().unwrap();
    let supported_buffer_size = config.buffer_size();
    // let sample_format = config.sample_format();
    let mut config: cpal::StreamConfig = config.into();

    // let sample_rate = config.sample_rate.0 as f32;
    // let channels = config.channels as usize;
    let buffer_size = 1028;
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

    Ok((stream, producer))
}

fn err_fn(err: cpal::StreamError) {
    eprintln!("an error occurred on stream: {}", err);
}
