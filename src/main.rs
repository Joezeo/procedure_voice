use voice::text_to_audio;

pub mod voice;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = "Ha Ha Ha Ha Ha Ha Ha, Nice to meet you!";
    text_to_audio(text)?;
    Ok(())
}
