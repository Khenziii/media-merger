mod files;

fn main() {
    files::validate_env("./input", "./output");
    let audio_file_duration = files::get_length_of_audio_file("./input/audio.mp3");
}
