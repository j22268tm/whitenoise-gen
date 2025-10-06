use rodio::{OutputStream, Sink, Source};
use std::{time::Duration, u64};
use rand::Rng;

struct WhiteNoise {
    sample_rate: u32,
}

impl Iterator for WhiteNoise {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let sample = rand::thread_rng().gen_range(-1.0..1.0);
        Some(sample)
    }
}

impl Source for WhiteNoise {
    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn total_duration(&self) -> Option<Duration> {
        None    
    }
}

fn main() {
    println!("Input sample rate (default 44100): ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let sample_rate: u32 = input.trim().parse().unwrap_or(44100);
    println!("Playing white noise... Quit with Ctrl+C");

    let(_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let source = WhiteNoise { sample_rate };
    sink.append(source);

    std::thread::sleep(Duration::from_secs(u64::MAX));
}