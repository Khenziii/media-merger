use std::{env, fs};
use std::path::{Path, PathBuf};
use ffprobe;

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

    // make sure, that output directory is empty
    let in_dev_env = env::var("MEDIA_MERGER_ENV").unwrap_or_default() == "DEV";

    let is_empty = fs::read_dir(output_dir)
        .unwrap()
        .count() == if in_dev_env { 1 } else { 0 }; // README.md..

    if !is_empty {
        panic!("Directory: {} is not empty! Please make sure, that it doesn't contain any files.", output_dir)
    }
}

pub fn get_length_of_file(file_path: &String) -> i32 {
    let metadata = ffprobe::ffprobe(Path::new(file_path)).unwrap();
    let duration = &metadata
        .streams
        .first()
        .expect("A file didn't contain any streams??")
        .duration
        .clone()
        .expect("Failed to get media file's duration!");

    return duration.parse::<f32>().expect("Got invalid media file's duration!").round() as i32;
}

pub fn get_videos_fps(path: &String) -> i32 {
    let metadata = ffprobe::ffprobe(Path::new(path)).unwrap();
    let fps = &metadata
        .streams
        .first()
        .expect("A file didn't contain any streams??")
        .avg_frame_rate
        .split("/")
        .collect::<Vec<&str>>()[0];

    return fps.parse::<f32>().expect("Got invalid fps count!").round() as i32;
}
