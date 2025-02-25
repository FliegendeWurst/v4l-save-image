use std::fs;

use chrono::{Datelike, Local, Timelike};
use v4l::{buffer::Type, io::traits::CaptureStream, prelude::MmapStream, Device};

fn main() {
    println!("Using device: 0");

    let mut dev = Device::new(0).expect("Failed to open device");

    let warmup = 20;

    let mut stream = MmapStream::with_buffers(&mut dev, Type::VideoCapture, warmup + 1)
        .expect("Failed to create buffer stream");

    (0..warmup).for_each(|_| { stream.next().unwrap(); });

    let (data, buf) = stream.next().expect("Failed to capture buffer");
    println!("Buffer");
    println!("  sequence  : {}", buf.sequence);
    println!("  timestamp : {}", buf.timestamp);
    println!("  flags     : {}", buf.flags);
    println!("  length    : {}", buf.bytesused);
    let now = Local::now();
    fs::write(format!("image_{}-{:02}-{:02}_{:02}-{:02}-{:02}.jpg", now.year(), now.month(), now.day(), now.hour(), now.minute(), now.second()), data).unwrap();
}
