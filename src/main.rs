use voice::text_to_audio;

pub mod voice;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = "Ha Ha Ha Ha Ha Ha Ha, Nice to meet you!";
    text_to_audio(text)?;
    Ok(())
}

// use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
// use std::f32::consts::PI;

// struct Voice {
//     base_freq: f32,
//     pitch_variation: f32,
//     rate: f32,
// }

// impl Voice {
//     fn new(base_freq: f32, pitch_variation: f32, rate: f32) -> Self {
//         Self {
//             base_freq,
//             pitch_variation,
//             rate,
//         }
//     }

//     fn generate_sample(&self, time: f32) -> f32 {
//         let pitch = self.base_freq + (self.pitch_variation * (time * self.rate).sin());
//         let waveform =
//             ((2.0 * PI * pitch * time).sin() + (4.0 * PI * pitch * time).sin() * 0.5) * 0.5; // 结合多个波形
//         waveform * ((time * 5.0).sin() * 0.5 + 0.5) // 加入动态音量变化
//     }
// }

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let host = cpal::default_host();
//     let device = host
//         .default_output_device()
//         .expect("未找到默认音频输出设备！");
//     let config = device.default_output_config()?;

//     let sample_rate = config.sample_rate().0 as f32;

//     let mut time = 0.0;

//     // 定义不同角色的音色
//     let voices = vec![
//         Voice::new(200.0, 20.0, 3.0), // 角色1
//         Voice::new(300.0, 30.0, 2.0), // 角色2
//     ];
//     let mut voice_index = 0;

//     let stream = match config.sample_format() {
//         cpal::SampleFormat::F32 => device.build_output_stream(
//             &config.into(),
//             move |data: &mut [f32], _| {
//                 for sample in data.iter_mut() {
//                     let voice = &voices[voice_index];
//                     *sample = voice.generate_sample(time);

//                     time += 1.0 / sample_rate;

//                     // 每秒切换角色
//                     if time % 1.0 < 1.0 / sample_rate {
//                         voice_index = (voice_index + 1) % voices.len();
//                     }
//                 }
//             },
//             |err| eprintln!("音频流错误：{}", err),
//             None,
//         )?,
//         _ => panic!("不支持的音频格式！"),
//     };

//     stream.play()?;
//     println!("正在生成音效，按 Ctrl+C 退出...");
//     std::thread::park();

//     Ok(())
// }

// use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
// use std::f32::consts::PI;

// struct Voice {
//     base_freq: f32,
//     pitch_variation: f32,
//     rate: f32,
// }

// impl Voice {
//     fn new(base_freq: f32, pitch_variation: f32, rate: f32) -> Self {
//         Self {
//             base_freq,
//             pitch_variation,
//             rate,
//         }
//     }

//     fn generate_sample(&self, time: f32) -> f32 {
//         let pitch = self.base_freq + (self.pitch_variation * (time * self.rate).sin());
//         let waveform =
//             ((2.0 * PI * pitch * time).sin() + (4.0 * PI * pitch * time).sin() * 0.5) * 0.5;
//         waveform * ((time * 5.0).sin() * 0.5 + 0.5)
//     }
// }

// // 音量包络：模拟自然的渐入和渐出
// fn envelope(time: f32, duration: f32) -> f32 {
//     let attack = 0.1; // 渐入时间
//     let release = 0.1; // 渐出时间
//     if time < attack {
//         time / attack // 渐入
//     } else if time > duration - release {
//         (duration - time) / release // 渐出
//     } else {
//         1.0 // 持续
//     }
// }

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let host = cpal::default_host();
//     let device = host
//         .default_output_device()
//         .expect("未找到默认音频输出设备！");
//     let config = device.default_output_config()?;

//     let sample_rate = config.sample_rate().0 as f32;
//     let mut time = 0.0;

//     // 定义不同角色的音色
//     let voices = vec![
//         Voice::new(200.0, 20.0, 3.0), // 角色1
//         Voice::new(300.0, 30.0, 2.0), // 角色2
//     ];
//     let mut voice_index = 0;

//     let duration = 0.5; // 每段语音持续时间
//     let pause_duration = 0.2; // 停顿时间
//     let mut segment_time = 0.0;

//     let stream = match config.sample_format() {
//         cpal::SampleFormat::F32 => device.build_output_stream(
//             &config.into(),
//             move |data: &mut [f32], _| {
//                 for sample in data.iter_mut() {
//                     let voice = &voices[voice_index];

//                     // 在语音段内生成音频
//                     if segment_time < duration {
//                         let env = envelope(segment_time, duration);
//                         *sample = env * voice.generate_sample(time);
//                     } else {
//                         *sample = 0.0; // 停顿期间输出静音
//                     }

//                     time += 1.0 / sample_rate;
//                     segment_time += 1.0 / sample_rate;

//                     // 切换到下一段语音
//                     if segment_time > duration + pause_duration {
//                         segment_time = 0.0;
//                         voice_index = (voice_index + 1) % voices.len();
//                     }
//                 }
//             },
//             |err| eprintln!("音频流错误：{}", err),
//             None,
//         )?,
//         _ => panic!("不支持的音频格式！"),
//     };

//     stream.play()?;
//     println!("正在生成音效，按 Ctrl+C 退出...");
//     std::thread::park();

//     Ok(())
// }
