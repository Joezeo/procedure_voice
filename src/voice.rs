use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::f32::consts::PI;

struct Voice {
    base_freq: f32,
    pitch_variation: f32,
    rate: f32,
}

impl Voice {
    fn new(base_freq: f32, pitch_variation: f32, rate: f32) -> Self {
        Self {
            base_freq,
            pitch_variation,
            rate,
        }
    }

    fn generate_sample(&self, time: f32) -> f32 {
        let pitch = self.base_freq + (self.pitch_variation * (time * self.rate).sin());
        let waveform =
            ((2.0 * PI * pitch * time).sin() + (4.0 * PI * pitch * time).sin() * 0.3) * 0.4;
        waveform * ((time * 2.0).sin() * 0.3 + 0.7) // Dynamic volume changes to reduce amplitude fluctuations
    }
}

pub fn text_to_audio(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("Default audio output device not found.");
    let config = device.default_output_config()?;

    let sample_rate = config.sample_rate().0 as f32;
    let mut time = 0.0;

    // Define durations for characters and words
    let char_duration = 0.1; // Duration for each character sound (in seconds)
    let word_pause = 0.18; // Pause duration between words (in seconds)

    // Generate a list of voices based on the text
    let voices: Vec<(Voice, bool)> = text
        .split_whitespace() // Split text into words
        .flat_map(|word| {
            let frequency_flag = if word.contains(",") || word.contains(".") {
                -1.
            } else {
                1.
            };
            let pitch_offset = if word.contains("!") { 10. } else { 0. };
            word.chars()
                .map(move |c| {
                    (
                        Voice::new(
                            200.0 + frequency_flag * ((c as u8) % 26) as f32,
                            10.0 + pitch_offset,
                            1.5,
                        ),
                        false,
                    )
                }) // Generate a voice for each character
                .chain(std::iter::once((Voice::new(0.0, 0.0, 0.0), true))) // Add silence between words
        })
        .collect();

    let mut index = 0;
    let mut char_time = 0.0;

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => device.build_output_stream(
            &config.into(),
            move |data: &mut [f32], _| {
                for sample in data.iter_mut() {
                    if index < voices.len() {
                        let (voice, is_pause) = &voices[index];
                        if *is_pause {
                            *sample = 0.0; // Silence for word pauses
                        } else {
                            // Apply fade-in and fade-out effects
                            let fade_duration = 0.01; // Duration for fade-in and fade-out (in seconds)
                            let amplitude = if char_time < fade_duration {
                                char_time / fade_duration
                            } else if char_time > char_duration - fade_duration {
                                (char_duration - char_time) / fade_duration
                            } else {
                                1.0
                            };
                            let raw_sample = voice.generate_sample(time);
                            *sample = raw_sample * amplitude * 0.9; // Limit maximum amplitude
                        }

                        time += 1.0 / sample_rate;
                        char_time += 1.0 / sample_rate;

                        if *is_pause && char_time >= word_pause
                            || !*is_pause && char_time >= char_duration
                        {
                            index += 1;
                            char_time = 0.0;
                        }
                    } else {
                        *sample = 0.0; // Silence indicates the end of the text
                    }
                }
            },
            |err| eprintln!("Audio stream error: {}", err),
            None,
        )?,
        _ => panic!("Unsupported audio format."),
    };

    stream.play()?;
    println!("Generating audio from text, press Ctrl+C to exit...");
    std::thread::park();

    Ok(())
}
