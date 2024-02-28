use std::fs;

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
