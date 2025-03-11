#![allow(unreachable_code)]
mod audio;

use audio::init_stream;
use blaadje::{run_with_env, set_prelude, Channel, Engine, Environment};
use ringbuf::traits::Producer;
use ringbuf::{storage::Heap, wrap::caching::Caching, SharedRb};
use std::cell::RefCell;
use std::error::Error;
use std::io::{stdin, stdout, Write};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

const SAMPLE_RATE: usize = 48_000;

fn main() -> Result<(), Box<dyn Error>> {
    let (env, channel) = Environment::new();

    let env = Rc::new(RefCell::new(env));
    set_prelude(env.clone()).expect("Unable to set prelude");

    let (_stream, producer) = init_stream()?;

    thread::spawn(|| {
        let _ = audio(producer, channel);
    });

    let _ = repl(env);

    Ok(())
}

fn repl(env: Rc<RefCell<Environment>>) -> Result<(), Box<dyn Error>> {
    // Loop
    loop {
        print!("blaadje> ");
        stdout().flush()?;

        // Read
        let mut input = String::new();
        stdin().read_line(&mut input)?;

        // Evaluate
        let output = run_with_env(&input, env.clone());

        // Print
        match output {
            Ok(v) => println!("\x1b[96m{:?}\x1b[0m", v),
            Err(v) => println!("\x1b[91mError: {:?}\x1b[0m", v),
        }
    }
}

fn audio(
    mut producer: Caching<Arc<SharedRb<Heap<f32>>>, true, false>,
    channel: Arc<Mutex<Channel>>,
) -> Result<(), Box<dyn Error>> {
    let mut engine = Engine::<48_000, 128, 128>::new();
    let mut count = 0;
    let mut sample = 0.0;

    loop {
        // Process message loop
        engine.process_channel(channel.clone());

        // Alternate between channels
        if let Ok(_) = producer.try_push(sample) {
            let (l, r) = engine.next_samples();
            sample = if count == 0 { l } else { r };
            count += 1;
            if count > 1 {
                count = 0;
            }
        }
    }

    Ok(())
}
