#[cfg(test)]
mod tests {
    use std::fs::{self, File};

    use qua_format::*;

    #[test]
    fn test_read() {
        let path = "./map_files/1416.qua";
        let qua = Qua::from_file(path).expect("Could not parse qua");

        assert_eq!("Csikos Post", qua.title);
        assert_eq!("zetoban", qua.artist);
        assert_eq!(1416, qua.map_id);

        assert_eq!(1, qua.timing_points.len());
        assert_eq!(0., qua.timing_points[0].start_time); // Default value

        assert_eq!(167, qua.hit_objects.len());
        assert_eq!(0, qua.hit_objects[0].start_time); // Default value
        assert_eq!(1, qua.hit_objects[0].lane);

        assert!(!qua.bpm_does_not_affect_scroll_velocity); // Default value

        assert_eq!(4, qua.game_mode.get_key_count());
    }

    #[test]
    fn test_write() {
        let qua = Qua {
            title: "Freedom Dive".to_string(),
            artist: "xi".to_string(),
            ..Default::default()
        };

        let new_path = "test.qua";
        let new_file = File::create(&new_path).expect("Could not create new file");
        qua.to_writer(new_file).expect("Could not write to file");

        fs::remove_file(&new_path).expect("Could not remove file");
    }

    #[test]
    fn test_read_write() {
        let path = "./map_files/1416.qua";
        let mut qua = Qua::from_file(path).expect("Could not parse qua");

        qua.title = "Freedom Dive".to_string();

        let new_path = "test.qua";
        let new_file = File::create(&new_path).expect("Could not create new file");
        qua.to_writer(new_file).expect("Could not write to file");

        fs::remove_file(&new_path).expect("Could not remove file");
    }
}
