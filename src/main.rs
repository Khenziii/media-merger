mod files;

fn main() {
    files::validate_env("./input", "./output");

    let files = files::get_files("./input");
    let audio_file_duration = files::get_length_of_audio_file(format!("./input/{:?}", files.audio));
}
