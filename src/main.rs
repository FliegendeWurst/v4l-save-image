use chrono::{Datelike, Local, Timelike};
use image::{ImageBuffer, Rgb};
use v4l::{Buffer, CaptureDevice, CaptureFormat, FourCC, MappedBufferStream};

fn main() {
    let path = "/dev/video0".to_owned();
    println!("Using device: {}", path);

    let mut dev = CaptureDevice::with_path(path).expect("Failed to open device");
    println!("{:?}", dev.enumerate_formats());
    println!("{:?}", dev.set_format(&CaptureFormat::new(640, 480, FourCC::new(b"RGB3"))));
    let params = dev.get_params().expect("Failed to get parameters");
    println!("Active format:\n{}", dev.get_format().unwrap());
    println!("Active parameters:\n{}", params);

    let mut stream = MappedBufferStream::with_buffers(&mut dev, 1)
        .expect("Failed to create buffer stream");

    let buf = stream.next().expect("Failed to capture buffer");
    println!("Buffer");
    println!("  sequence  : {}", buf.seq());
    println!("  timestamp : {}", buf.timestamp());
    println!("  flags     : {}", buf.flags());
    println!("  length    : {}", buf.len());
    let now = Local::now();
    let img: ImageBuffer<Rgb<u8>, _> = ImageBuffer::from_vec(640, 480, buf.data().to_vec()).unwrap();
    img.save(format!("image_{}-{:02}-{:02}_{:02}-{:02}-{:02}.jpg", now.year(), now.month(), now.day(), now.hour(), now.minute(), now.second())).unwrap();
}
