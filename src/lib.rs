use std::{fs::File, path::Path};

use serde::Deserialize;

#[derive(Debug)]
pub enum QuaError {
    IoError(std::io::Error),
    SerdeError(serde_yaml::Error),
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
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
    pub map_id: i32,
    /// The unique Map Set identifier (-1 if not submitted)
    pub map_set_id: i32,
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
    pub slider_velocities: Vec<ScrollVelocityInfo>,
    /// HitObject .qua data
    pub hit_objects: Vec<HitObjectInfo>,
}

impl Qua {
    fn from_file<P: AsRef<Path>>(path: P) -> Result<Qua, QuaError> {
        let path = Path::new(path.as_ref());
        let file = File::open(path).map_err(QuaError::IoError)?;
        let qua = Qua::from_reader(file)?;
        Ok(qua)
    }

    fn from_reader<R>(reader: R) -> Result<Qua, QuaError>
    where
        R: std::io::Read,
    {
        let qua: Qua = serde_yaml::from_reader(reader).map_err(QuaError::SerdeError)?;
        Ok(qua)
    }

    fn from_str(s: &str) -> Result<Qua, QuaError> {
        let qua: Qua = serde_yaml::from_str(s).map_err(QuaError::SerdeError)?;
        Ok(qua)
    }
}

impl Default for Qua {
    fn default() -> Self {
        Self {
            audio_file: "".to_string(),
            song_preview_time: 0,
            background_file: "".to_string(),
            banner_file: "".to_string(),
            map_id: -1,
            map_set_id: -1,
            game_mode: GameMode::Keys4,
            title: "".to_string(),
            artist: "".to_string(),
            source: "".to_string(),
            tags: "".to_string(),
            creator: "".to_string(),
            difficulty_name: "".to_string(),
            description: "".to_string(),
            genre: "".to_string(),
            bpm_does_not_affect_scroll_velocity: false,
            initial_scroll_velocity: 1.0,
            has_scratch_key: false,
            editor_layers: Vec::new(),
            custom_audio_samples: Vec::new(),
            sound_effects: Vec::new(),
            timing_points: Vec::new(),
            slider_velocities: Vec::new(),
            hit_objects: Vec::new(),
        }
    }
}

#[derive(Deserialize)]
pub enum GameMode {
    Keys4 = 1,
    Keys7 = 2,
}

impl GameMode {
    fn from_key_count(key_count: i32) -> Option<GameMode> {
        match key_count {
            4 => Some(GameMode::Keys4),
            7 => Some(GameMode::Keys7),
            _ => None,
        }
    }

    fn get_key_count(self) -> i32 {
        match self {
            GameMode::Keys4 => 4,
            GameMode::Keys7 => 7,
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct EditorLayerInfo {
    /// The name of the layer
    pub name: String,
    /// Is the layer hidden in the editor?
    pub hidden: bool,
    /// The color of the layer (default is white) in rrr,ggg,bbb format
    pub color_rgb: String,
}

impl Default for EditorLayerInfo {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            hidden: false,
            color_rgb: "255,255,255".to_string(),
        }
    }
}

/// CustomAudioSamples section of the .qua
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct CustomAudioSampleInfo {
    /// The path to the audio sample.
    pub path: String,
    /// If true, the audio sample is always played back at 1.0x speed, regardless of the rate.
    pub unaffected_by_rate: bool,
}

impl Default for CustomAudioSampleInfo {
    fn default() -> Self {
        Self {
            path: "".to_string(),
            unaffected_by_rate: false,
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct SoundEffectInfo {
    /// The time at which to play the sound sample.
    pub start_time: f32,
    /// The one-based index of the sound sample in the CustomAudioSamples array.
    pub sample: i32,
    /// The volume of the sound sample. Defaults to 100.
    pub volume: i32,
}

impl Default for SoundEffectInfo {
    fn default() -> Self {
        Self {
            start_time: 0.0,
            sample: 0,
            volume: 0,
        }
    }
}

/// TimingPoints section of the .qua
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct TimingPointInfo {
    /// The time in milliseconds for when this timing point begins
    pub start_time: i32,
    /// The BPM during this timing point
    pub bpm: f32,
    /// The signature during this timing point
    pub signature: TimeSignature,
    /// Whether timing lines during this timing point should be hidden or not
    pub hidden: bool,
}

impl Default for TimingPointInfo {
    fn default() -> Self {
        Self {
            start_time: 0,
            bpm: 0.0,
            signature: TimeSignature::Quadruple,
            hidden: false,
        }
    }
}

/// SliderVelocities section of the .qua
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ScrollVelocityInfo {
    /// The time in milliseconds when the new SliderVelocity section begins
    pub start_time: i32,
    /// The velocity multiplier relative to the current timing section's BPM
    pub multiplier: f32,
}

#[derive(Deserialize)]
pub enum TimeSignature {
    Quadruple = 4,
    Triple = 3,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct HitObjectInfo {
    /// The time in milliseconds when the HitObject is supposed to be hit.
    pub start_time: i32,
    /// The lane the HitObject falls in
    pub lane: i32,
    /// The endtime of the HitObject (if greater than 0, it's considered a hold note.)
    pub end_time: i32,
    /// Bitwise combination of hit sounds for this object
    // TODO: Handle hitsound bitflags
    pub hit_sound: u8,
    /// Key sounds to play when this object is hit.
    pub key_sounds: Vec<KeySoundInfo>,
    /// The layer in the editor that the object belongs to.
    pub editor_layer: i32,
}

impl Default for HitObjectInfo {
    fn default() -> Self {
        Self {
            start_time: 0,
            lane: 1,
            end_time: 0,
            hit_sound: 0,
            key_sounds: Vec::new(),
            editor_layer: 0,
        }
    }
}

/// KeySounds property of hit objects.
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct KeySoundInfo {
    pub sample: i32,
    pub volume: i32,
}

impl Default for KeySoundInfo {
    fn default() -> Self {
        Self {
            sample: 0,
            volume: 100,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        let qua = Qua::from_file("./map_files/1416.qua").expect("Could not parse qua");

        assert_eq!("Csikos Post", qua.title);
        assert_eq!("zetoban", qua.artist);
        assert_eq!(1416, qua.map_id);

        assert_eq!(1, qua.timing_points.len());
        assert_eq!(0, qua.timing_points[0].start_time); // Default value

        assert_eq!(167, qua.hit_objects.len());
        assert_eq!(0, qua.hit_objects[0].start_time); // Default value
        assert_eq!(1, qua.hit_objects[0].lane);

        assert!(!qua.bpm_does_not_affect_scroll_velocity); // Default value

        assert_eq!(4, qua.game_mode.get_key_count());
    }
}
