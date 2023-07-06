use elevenlabs_rs::{
    tts::{TtsApi, TtsBody, VoiceSettings},
    *,
};

fn main() {
    let auth = Auth::from_env().unwrap();
    let elevenlabs = Elevenlabs::new(auth, "https://api.elevenlabs.io/v1/");

    let tts_body = TtsBody {
        model_id: None,
        text: "Hello world".to_string(),
        voice_settings: VoiceSettings {
            stability: 0.5,
            similarity_boost: 0.5,
        },
    };

    let tts_result = elevenlabs.tts(&tts_body, "yoZ06aMxZJJ28mfd3POQ");
    let bytes = tts_result.unwrap();

    std::fs::write("tts.mp3", bytes).unwrap();
}
