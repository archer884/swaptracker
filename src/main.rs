extern crate sys_info as sys;

use std::env;
use std::fs::OpenOptions;
use std::io::{Result, Write};
use std::thread;
use std::time::Duration;
use sys::MemInfo;

fn main() {
    let path = env::args().nth(1).expect("Output file path required.");
    monitor(&path);
}

fn monitor(path: &str) {
    let mut buf = Vec::new();
    loop {
        buf.push(sys::mem_info().expect("Unable to get memory state."));

        if buf.len() > 4 {
            write(path, &buf).expect("Unable to write results.");
            buf.clear();
        }

        thread::sleep(Duration::from_secs(60));
    }
}

fn write(path: &str, records: &[MemInfo]) -> Result<()> {
    let mut file = OpenOptions::new().create(true).append(true).open(&path)?;
    for record in records {
        writeln!(file, "{:?}", record)?;
    }
    Ok(())
}
