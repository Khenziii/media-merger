use std::fs;
use std::path::Path;
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

fn get_files(from_dir: &str) -> Vec<String> {
    let files = fs::read_dir(from_dir)
        .expect(&*format!("Failed to read contents of the directory {}!", from_dir))
        .filter_map(|entry| {
            return match entry {
                Ok(entry) => {
                    Some(entry.file_name().into_string().expect("Failed to get filenames string!"))
                },
                Err(e) => {
                    eprintln!("Error reading directory entry: {}", e);
                    None
                }
            }
        })
        .collect::<Vec<_>>();

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
    let present_filenames: Vec<&str> = present_files
        .iter()
        .map(|file| {
            return file.split(".").collect::<Vec<_>>()[0];
        })
        .collect();

    for file in &expected_files {
        let exists = present_filenames.contains(&file);

        if !exists {
            panic!("File: {} is missing in the {} directory", file, input_dir);
        }
    }
}

pub fn get_length_of_audio_file(file_path: &str) -> i32 {
    let metadata = MediaFileMetadata::new(&Path::new(file_path)).unwrap();
    let duration_string: String = match metadata.duration {
        Some(s) => s,
        None => panic!("The audio file is probably corrupted."),
    };
    let duration_milliseconds = convert_duration_to_milliseconds_from_string(duration_string);

    return duration_milliseconds;
}
