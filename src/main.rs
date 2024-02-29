mod files;

fn main() {
    files::validate_env("./input", "./output");

    let passed_files = files::get_files("./input");
    let audio_file_duration = files::get_length_of_audio_file(passed_files.audio.to_str().unwrap().to_string());
    println!("{}", audio_file_duration);
}
