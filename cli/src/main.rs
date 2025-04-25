mod system;

use blaadje::{run_with_env, set_prelude, Engine, Environment};
use clap::{Arg, Command};
use notify::{
    event::{AccessKind, AccessMode},
    recommended_watcher, Event, EventKind, RecursiveMode, Result as NotifyResult, Watcher,
};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use system::Sys;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("blaadje-cli")
        .about("CLI tool for the blaadje livecoding environment")
        .subcommand_required(true)
        .subcommand(
            Command::new("run")
                .about("Run blaadje code")
                .arg(Arg::new("file").required(true).num_args(1)),
        )
        .subcommand(
            Command::new("live")
                .about("Interactive file mode")
                .arg(Arg::new("file").required(true).num_args(1)),
        )
        .subcommand(
            Command::new("repl")
                .about("Open interactive REPL environment")
                .arg(
                    Arg::new("file")
                        .required(false)
                        .short('f')
                        .long("file")
                        .num_args(1),
                ),
        )
        .get_matches();

    let (env, channel) = Environment::new();
    let env = Arc::new(Mutex::new(env));
    set_prelude(env.clone()).expect("Unable to set prelude");

    thread::spawn(|| {
        let sys = Box::new(Sys::new());
        let channels = vec![channel];
        let mut engine = Engine::<44_100, 128, 128>::new(sys, channels);
        engine.process();
    });

    match matches.subcommand() {
        Some(("run", matches)) => {
            let file = matches.get_one::<String>("file").unwrap();
            run_file(env.clone(), file)
        }

        Some(("repl", matches)) => {
            if let Some(file) = matches.get_one::<String>("file") {
                run_file(env.clone(), file)?;
            }

            repl(env)
        }

        Some(("live", matches)) => {
            let file = matches.get_one::<String>("file").unwrap();

            {
                env.lock().unwrap().live_mode();
            }

            run_file(env.clone(), file)?;

            let (tx, rx) = mpsc::channel::<NotifyResult<Event>>();
            let mut watcher = recommended_watcher(tx)?;

            watcher.watch(Path::new(file), RecursiveMode::Recursive)?;

            for res in rx {
                match res {
                    Ok(event) => match event.kind {
                        EventKind::Access(AccessKind::Close(AccessMode::Write)) => {
                            // Cleanup
                            run("(output_disconnect_all)", env.clone());

                            // Rerun file
                            run_file(env.clone(), file)?;
                        }
                        _ => (),
                    },
                    Err(e) => println!("watch error: {:?}", e),
                }
            }

            Ok(())
        }
        _ => unreachable!(),
    }
}

fn run_file(env: Arc<Mutex<Environment>>, path: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut code = String::new();
    file.read_to_string(&mut code)?;

    run(&code, env.clone());

    Ok(())
}

fn repl(env: Arc<Mutex<Environment>>) -> Result<(), Box<dyn Error>> {
    // Loop
    loop {
        print!("blaadje> ");
        stdout().flush()?;

        // Read
        let mut input = String::new();
        stdin().read_line(&mut input)?;

        run(&input, env.clone());
    }
}

fn run(code: &str, env: Arc<Mutex<Environment>>) {
    // Evaluate
    let output = run_with_env(&code, env);

    // Print
    match output {
        Ok(v) => println!("\x1b[96m{}\x1b[0m", v),
        Err(v) => println!("\x1b[91mError: {:?}\x1b[0m", v),
    }
}
