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
        waveform * ((time * 2.0).sin() * 0.3 + 0.7) // 动态音量变化，减少振幅波动
    }
}

pub fn text_to_audio(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("未找到默认音频输出设备！");
    let config = device.default_output_config()?;

    let sample_rate = config.sample_rate().0 as f32;
    let mut time = 0.0;

    // 定义单词和字符的发音持续时间
    let char_duration = 0.1; // 单个字符音效持续时间（秒）
    let word_pause = 0.18; // 单词之间的间隔时间（秒）

    // 根据文本生成音色列表
    let voices: Vec<(Voice, bool)> = text
        .split_whitespace() // 按单词拆分
        .flat_map(|word| {
            let frenquence_flag = if word.contains(",") || word.contains(".") {
                -1.
            } else {
                1.
            };
            let pitch_offset = if word.contains("!") { 10. } else { 0. };
            word.chars()
                .map(move |c| {
                    (
                        Voice::new(
                            300.0 + frenquence_flag * ((c as u8) % 26) as f32,
                            15.0 + pitch_offset,
                            2.5,
                        ),
                        false,
                    )
                }) // 字符生成音色
                .chain(std::iter::once((Voice::new(0.0, 0.0, 0.0), true))) // 单词间隔静音
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
                            *sample = 0.0; // 静音处理单词间隔
                        } else {
                            let raw_sample = voice.generate_sample(time);
                            *sample = raw_sample * 0.9; // 限制最大振幅，减少失真
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
                        *sample = 0.0; // 静音，表示文本播放结束
                    }
                }
            },
            |err| eprintln!("音频流错误：{}", err),
            None,
        )?,
        _ => panic!("不支持的音频格式！"),
    };

    stream.play()?;
    println!("正在生成文本音频，按 Ctrl+C 退出...");
    std::thread::park();

    Ok(())
}
