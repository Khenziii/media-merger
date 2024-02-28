use std::fs;
use std::path::Path;

pub fn get_files(from_dir: &str) -> Vec<String> {
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
