pub struct Qua {
    /// The name of the audio file
    pub audio_file: String,
    /// Time in milliseconds of the song where the preview starts
    pub song_preview_time: i32,
    /// The name of the background file
    pub background_file: String,
    /// The name of the mapset banner
    pub banner_file: String,
    /// The unique Map Identifier (-1 if not submitted)
    pub map_id: i32, // = -1
    /// The unique Map Set identifier (-1 if not submitted)
    pub map_set_id: i32, // = -1
    /// The game mode for this map
    pub game_mode: GameMode,
    /// The title of the song
    pub title: String,
    /// The artist of the song
    pub artist: String,
    /// The source of the song (album, mixtape, etc.)
    pub source: String,
    /// Any tags that could be used to help find the song.
    pub tags: String,
    /// The creator of the map
    pub creator: String,
    /// The difficulty name of the map.
    pub difficulty_name: String,
    /// A description about this map.
    pub description: String,
    /// The genre of the song
    pub genre: String,
    /// Indicates if the BPM changes in affect scroll velocity.
    ///
    /// If this is set to false, SliderVelocities are in the denormalized format (BPM affects SV),
    /// and if this is set to true, SliderVelocities are in the normalized format (BPM does not affect SV).
    pub bpm_does_not_affect_scroll_velocity: bool,
    /// The initial scroll velocity before the first SV change.
    ///
    /// Only matters if BPMDoesNotAffectScrollVelocity is true.
    pub initial_scroll_velocity: f32,
    /// If true, the map will have a +1 scratch key, allowing for 5/8 key play
    pub has_scratch_key: bool,
    /// EditorLayer .qua data
    pub editor_layers: Vec<EditorLayerInfo>,
    /// CustomAudioSamples .qua data
    pub custom_audio_samples: Vec<CustomAudioSampleInfo>,
    /// SoundEffects .qua data
    pub sound_effects: Vec<SoundEffectInfo>,
    /// TimingPoint .qua data
    pub timing_points: Vec<TimingPointInfo>,
    /// Slider Velocity .qua data
    ///
    /// Note that SVs can be both in normalized and denormalized form, depending on BPMDoesNotAffectSV.
    /// Check WithNormalizedSVs if you need normalized SVs.
    pub slider_velocities: Vec<SliderVelocityInfo>,
    /// HitObject .qua data
    pub hit_objects: Vec<HitObjectInfo>,
}

pub enum GameMode {
    Keys4 = 1,
    Keys7 = 2,
}

pub struct EditorLayerInfo {
    /// The name of the layer
    pub name: String,
    /// Is the layer hidden in the editor?
    pub hidden: bool,
    /// The color of the layer (default is white)
    pub color_rgb: String,
}

/// CustomAudioSamples section of the .qua
pub struct CustomAudioSampleInfo {
    /// The path to the audio sample.
    pub path: String,
    /// If true, the audio sample is always played back at 1.0x speed, regardless of the rate.
    pub unaffected_by_rate: bool,
}

pub struct SoundEffectInfo {
    /// The time at which to play the sound sample.
    pub start_time: f32,
    /// The one-based index of the sound sample in the CustomAudioSamples array.
    pub sample: i32,
    /// The volume of the sound sample. Defaults to 100.
    pub volume: i32,
}

/// TimingPoints section of the .qua
pub struct TimingPointInfo {
    /// The time in milliseconds for when this timing point begins
    pub start_time: i32,
    /// The BPM during this timing point
    pub bpm: f32,
}

/// SliderVelocities section of the .qua
pub struct SliderVelocityInfo {
    /// The time in milliseconds when the new SliderVelocity section begins
    pub start_time: i32,
    /// The velocity multiplier relative to the current timing section's BPM
    pub multiplier: f32,
    /// The signature during this timing point
    pub signature: TimeSignature,
    /// Whether timing lines during this timing point should be hidden or not
    pub hidden: bool,
}

pub enum TimeSignature {
    Quadruple = 4,
    Triple = 3,
}

pub struct HitObjectInfo {
    /// The time in milliseconds when the HitObject is supposed to be hit.
    pub start_time: i32,
    /// The lane the HitObject falls in
    pub lane: i32,
    /// The endtime of the HitObject (if greater than 0, it's considered a hold note.)
    pub end_time: i32,
    /// Bitwise combination of hit sounds for this object
    pub hit_sound: HitSounds,
    /// Key sounds to play when this object is hit.
    pub key_sounds: Vec<KeySoundInfo>,
    /// The layer in the editor that the object belongs to.
    pub editor_layer: i32,
}

pub enum HitSounds {
    Normal = 1 << 0,  // This is 1, but Normal should be played regardless if it's 0 or 1.
    Whistle = 1 << 1, // 2
    Finish = 1 << 2,  // 4
    Clap = 1 << 3,    // 8
}

/// KeySounds property of hit objects.
pub struct KeySoundInfo {
    pub sample: i32,
    pub volume: i32,
}
