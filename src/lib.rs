struct Qua {
    audio_file: String,
    song_preview_time: i32,
    background_file: String,
    banner_file: String,
    map_id: i32,     // = -1
    map_set_id: i32, // = -1
    game_mode: GameMode,
    title: String,
    artist: String,
    source: String,
    tags: String,
    creator: String,
    difficulty_name: String,
    description: String,
    genre: String,
    bpm_does_not_affect_scroll_velocity: bool,
    initial_scroll_velocity: f32,
    has_scratch_key: bool,
    editor_layers: Vec<EditorLayerInfo>,
    custom_audio_samples: Vec<CustomAudioSampleInfo>,
    sound_effects: Vec<SoundEffectInfo>,
    timing_points: Vec<TimingPointInfo>,
    slider_velocities: Vec<SliderVelocityInfo>,
    hit_objects: Vec<HitObjectInfo>,
}

enum GameMode {
    Keys4 = 1,
    Keys7 = 2,
}

struct EditorLayerInfo {}
struct CustomAudioSampleInfo {}
struct SoundEffectInfo {}
struct TimingPointInfo {}
struct SliderVelocityInfo {}
struct HitObjectInfo {}
