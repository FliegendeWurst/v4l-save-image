use chrono::{Datelike, Local, Timelike};
use image::{ImageBuffer, Rgb};
use v4l::{buffer::Type, io::traits::CaptureStream, prelude::MmapStream, Device};

fn main() {
    println!("Using device: 0");

    let mut dev = Device::new(0).expect("Failed to open device");

    let mut stream = MmapStream::with_buffers(&mut dev, Type::VideoCapture, 1)
        .expect("Failed to create buffer stream");

    let (data, buf) = stream.next().expect("Failed to capture buffer");
    println!("Buffer");
    println!("  sequence  : {}", buf.sequence);
    println!("  timestamp : {}", buf.timestamp);
    println!("  flags     : {}", buf.flags);
    println!("  length    : {}", buf.bytesused);
    let now = Local::now();
    let img: ImageBuffer<Rgb<u8>, _> = ImageBuffer::from_vec(640, 480, data.to_vec()).unwrap();
    img.save(format!("image_{}-{:02}-{:02}_{:02}-{:02}-{:02}.jpg", now.year(), now.month(), now.day(), now.hour(), now.minute(), now.second())).unwrap();
}
