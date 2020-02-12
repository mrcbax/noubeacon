extern crate anyhow;
extern crate cpal;

use std::thread;
use std::time::Duration;

use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};

fn audio_thread(frequency: f32) {
    thread::spawn(move || {
        let host = cpal::default_host();
        let device = host.default_output_device().expect("failed to find a default output device");
        let format = device.default_output_format().expect("failed to obtain a proper output format");
        let event_loop = host.event_loop();
        let stream_id = event_loop.build_output_stream(&device, &format).expect("failed to build output stream");
        event_loop.play_stream(stream_id.clone()).expect("failed to play stream");

        let sample_rate = format.sample_rate.0 as f32;
        let mut sample_clock = 0f32;

        // Produce a sinusoid of maximum amplitude.
        let mut next_value = || {
            sample_clock = (sample_clock + 1.0) % sample_rate;
            (sample_clock * frequency * 2.0 * 3.141592 / sample_rate).sin()
        };

        event_loop.run(move |id, result| {
            let data = match result {
                Ok(data) => data,
                Err(err) => {
                    eprintln!("an error occurred on stream {:?}: {}", id, err);
                    return;
                }
            };

            match data {
                cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::U16(mut buffer) } => {
                    for sample in buffer.chunks_mut(format.channels as usize) {
                        let value = ((next_value() * 0.5 + 0.5) * std::u16::MAX as f32) as u16;
                        for out in sample.iter_mut() {
                            *out = value;
                        }
                    }
                },
                cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::I16(mut buffer) } => {
                    for sample in buffer.chunks_mut(format.channels as usize) {
                        let value = (next_value() * std::i16::MAX as f32) as i16;
                        for out in sample.iter_mut() {
                            *out = value;
                        }
                    }
                },
                cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer) } => {
                    for sample in buffer.chunks_mut(format.channels as usize) {
                        let value = next_value();
                        for out in sample.iter_mut() {
                            *out = value;
                        }
                    }
                },
                _ => (),
            }
        });
    });
}

fn main() {
    audio_thread(18000.0);
    audio_thread(18250.0);
    audio_thread(18500.0);
    audio_thread(18750.0);
    audio_thread(19000.0);
    audio_thread(19250.0);
    audio_thread(19500.0);
    audio_thread(19750.0);
    audio_thread(20000.0);
    audio_thread(20250.0);
    audio_thread(20500.0);
    audio_thread(20750.0);
    audio_thread(21000.0);
    audio_thread(21250.0);
    audio_thread(21500.0);
    audio_thread(21750.0);
    audio_thread(22000.0);
    loop {
        thread::sleep(Duration::from_millis(1000));
    }
}
