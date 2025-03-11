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
    let sample_format = config.sample_format();
    let config: cpal::StreamConfig = config.into();

    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    // Create a delay in case the input and output devices aren't synced.
    let latency_frames = (150.0 / 1_000.0) * config.sample_rate.0 as f32;
    let latency_samples = latency_frames as usize * config.channels as usize;

    // The buffer to share samples
    let ring = HeapRb::<f32>::new(latency_samples * 2);
    let (producer, mut consumer) = ring.split();

    println!("Default output config: {:?}", config);

    let output_data_fn = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        let mut input_fell_behind = false;
        for sample in data {
            *sample = match consumer.try_pop() {
                Some(s) => s,
                None => {
                    input_fell_behind = true;
                    0.0
                }
            };
        }
        if input_fell_behind {
            // eprintln!("input stream fell behind: try increasing latency");
        }
    };

    let stream = device.build_output_stream(&config, output_data_fn, err_fn, None)?;

    Ok((stream, producer))
}

fn err_fn(err: cpal::StreamError) {
    eprintln!("an error occurred on stream: {}", err);
}
