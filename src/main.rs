extern crate chrono;
extern crate sys_info as sys;

mod error;
mod timestamp;

use error::*;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::thread;
use std::time::Duration;
use sys::MemInfo;
use timestamp::Timestamp;

fn main() {
    let path = env::args().nth(1).expect("Output file path required.");

    if let Err(e) = monitor(&path) {
        eprintln!("{}", e);
    }
}

fn monitor(path: &str) -> Result<()> {
    let mut buf = Vec::new();
    loop {
        buf.push(Timestamp::new(sys::mem_info().map_err(Error::sys)?));

        if buf.len() > 4 {
            write(path, &buf)?;
            buf.clear();
        }

        thread::sleep(Duration::from_secs(60));
    }
}

fn write(path: &str, records: &[Timestamp<MemInfo>]) -> Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .map_err(|e| Error::io(e, "Unable to open output"))?;

    for record in records {
        serialize(&mut file, &record)?;
    }
    Ok(())
}

fn serialize<T: Write>(w: &mut T, item: &Timestamp<MemInfo>) -> Result<()> {
    let info = item.as_ref();
    writeln!(
        w,
        "{},{},{},{},{},{},{},{}",
        item.time,
        info.total,
        info.free,
        info.avail,
        info.buffers,
        info.cached,
        info.swap_total,
        info.swap_free
    ).map_err(|e| Error::io(e, "Unable to write to output"))
}
