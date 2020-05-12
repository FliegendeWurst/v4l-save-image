use chrono::{Datelike, Local, Timelike};
use image::{ImageBuffer, Rgb};
use v4l::{Buffer, CaptureDevice, CaptureFormat, FourCC, MappedBufferStream};

use std::time::Instant;

fn main() {
    // Determine which device to use
    let path: String = "/dev/video0".to_owned();
    println!("Using device: {}\n", path);

    let count = 1;

    // Allocate 4 buffers by default
    let buffers = 4;

    let mut dev = CaptureDevice::with_path(path).expect("Failed to open device");
    println!("{:?}", dev.enumerate_formats());
    println!("{:?}", dev.set_format(&CaptureFormat::new(640, 480, FourCC::new(b"RGB3"))));
    let params = dev.get_params().expect("Failed to get parameters");
    println!("Active format:\n{}", dev.get_format().unwrap());
    println!("Active parameters:\n{}", params);

    // Setup a buffer stream and grab a frame, then print its data
    let mut stream = MappedBufferStream::with_buffers(&mut dev, buffers)
        .expect("Failed to create buffer stream");

    // warmup
    stream.next().expect("Failed to capture buffer");

    let start = Instant::now();
    let mut megabytes_ps: f64 = 0.0;
    for i in 0..count {
        let t0 = Instant::now();
        let buf = stream.next().expect("Failed to capture buffer");
        let duration_us = t0.elapsed().as_micros();

        let cur = buf.len() as f64 / 1_048_576.0 * 1_000_000.0 / duration_us as f64;
        if i == 0 {
            megabytes_ps = cur;
        } else {
            // ignore the first measurement
            let prev = megabytes_ps * (i as f64 / (i + 1) as f64);
            let now = cur * (1.0 / (i + 1) as f64);
            megabytes_ps = prev + now;
        }

        println!("Buffer");
        println!("  sequence  : {}", buf.seq());
        println!("  timestamp : {}", buf.timestamp());
        println!("  flags     : {}", buf.flags());
        println!("  length    : {}", buf.len());
        let now = Local::now();
        let img: ImageBuffer<Rgb<u8>, _> = ImageBuffer::from_vec(640, 480, buf.data().to_vec()).unwrap();
        img.save(format!("image_{}-{:02}-{:02}_{:02}-{:02}-{:02}.jpg", now.year(), now.month(), now.day(), now.hour(), now.minute(), now.second())).unwrap();
    }

    println!();
    println!("FPS: {}", count as f64 / start.elapsed().as_secs_f64());
    println!("MB/s: {}", megabytes_ps);
}
