use std::fs;
use std::path::{Path, PathBuf};
use metadata::media_file::MediaFileMetadata;

/// converts a string in this format: "HH:MM:SS.ss"
/// (where HH = hours, MM = minutes, SS = seconds, ss = milliseconds)
/// to a number of milliseconds
fn convert_duration_to_milliseconds_from_string(string: String) -> i32 {
    let parts: Vec<&str> = string.split(":").collect();
    if parts.len() == 3 {
        let error_message = "Something went wrong while parsing";

        let hours = parts[0]
            .parse::<i32>()
            .expect(&*format!("{} hours. Tried to parse: {}", error_message, parts[0]));
        let minutes = parts[1]
            .parse::<i32>()
            .expect(&*format!("{} minutes. Tried to parse: {}", error_message, parts[1]));

        let seconds_and_milliseconds: Vec<&str> = parts[2]
            .split(".")
            .collect();
        let seconds = seconds_and_milliseconds[0]
            .parse::<i32>()
            .expect(&*format!("{} seconds. Tried to parse: {}", error_message, seconds_and_milliseconds[0]));
        let milliseconds = seconds_and_milliseconds[1]
            .parse::<i32>()
            .expect(&*format!("{} milliseconds. Tried to parse: {}", error_message, seconds_and_milliseconds[1]));

        return hours * 1000 * 60 * 60 + minutes * 1000 * 60 + seconds * 1000 + milliseconds;
    }

    panic!("Failed to convert the duration String to i32.");
}

#[derive(Debug)]
pub struct Files {
    pub(crate) audio: PathBuf,
    pub(crate) image: PathBuf,
    pub(crate) video: PathBuf,
}

impl Files {
    fn get_values(&self) -> Vec<PathBuf> {
        return vec![
            self.audio.clone(),
            self.image.clone(),
            self.video.clone(),
        ];
    }
}

pub fn get_files(from_dir: &str) -> Files {
    let dir_content: Vec<String> = fs::read_dir(from_dir)
        .expect(&*format!("Failed to read contents of the directory {}!", from_dir))
        .map(|entry| {
            return match entry {
                Ok(entry) => {
                    entry.file_name().into_string().expect("Failed to get filenames string!")
                },
                Err(e) => {
                    panic!("Error reading directory: {}, entry: {}", from_dir, e);
                }
            }
        })
        .collect::<Vec<_>>();

    let mut files = Files {
        audio: PathBuf::new(),
        image: PathBuf::new(),
        video: PathBuf::new(),
    };

    for file in dir_content {
        let filename_without_extension = file.split(".").collect::<Vec<&str>>()[0];

        if ["audio", "image", "video"].contains(&filename_without_extension) {
            let formatted_path = &format!("{}/{}", from_dir, file);
            let path = Path::new(&formatted_path);
            match filename_without_extension {
                "audio" => files.audio = path.to_path_buf(),
                "image" => files.image = path.to_path_buf(),
                "video" => files.video = path.to_path_buf(),
                _ => (),
            }
        }
    }

    return files;
}

pub fn validate_env(input_dir: &str, output_dir: &str) {
    // validate presence of required directories
    let input_dir_exists = Path::new(input_dir).exists();
    if !input_dir_exists {
        panic!("Directory: '{}' does not exist! Please make sure, that it's in the same folder as the executable", input_dir);
    }

    let output_dir_exists = Path::new(output_dir).exists();
    if !output_dir_exists {
        panic!("Directory: '{}' does not exist! Please make sure, that it's in the same folder as the executable", output_dir);
    }

    // validate presence of required files in the input directory
    let expected_files = ["audio", "image", "video"];
    let present_files = get_files(&input_dir);
    let present_filenames: Vec<String> = present_files
        .get_values()
        .into_iter()
        .filter_map(|entry| {
            return entry
                .with_extension("")
                .display()
                .to_string()
                .into()
        })
        .collect();

    for file in &expected_files {
        let formatted_file = format!("{}/{}", input_dir, &file);
        let exists = present_filenames.contains(&formatted_file);

        if !exists {
            panic!("File: {} is missing in the {} directory", file, input_dir);
        }
    }
}

pub fn get_length_of_audio_file(file_path: String) -> i32 {
    let metadata = MediaFileMetadata::new(&Path::new(&file_path)).unwrap();
    let duration_string: String = match metadata.duration {
        Some(s) => s,
        None => panic!("The audio file is probably corrupted."),
    };
    let duration_milliseconds = convert_duration_to_milliseconds_from_string(duration_string);

    return duration_milliseconds;
}
