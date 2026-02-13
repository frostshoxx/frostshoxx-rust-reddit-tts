use tts::Tts;
use std::thread;
use std::time::Duration;

pub struct TtsService;

impl TtsService {
    pub fn new() -> Self {
        Self
    }

    pub fn speak_text(&self, text: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut tts = Tts::default()?;
        tts.speak(text, true)?;
        
        // Wait until the engine finishes speaking
        while tts.is_speaking()? {
            thread::sleep(Duration::from_millis(100));
        }
        
        tts.stop()?;
        Ok(())
    }

    pub fn pause_between(&self, duration_ms: u64) {
        thread::sleep(Duration::from_millis(duration_ms));
    }
}
