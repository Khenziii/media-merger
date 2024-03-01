mod files;
mod video;

fn main() {
    files::validate_env("./input", "./output");

    let passed_files = files::get_files("./input");
    let audio_file_duration = files::get_length_of_file(&passed_files.audio.to_str().unwrap().to_string());

    let segments = video::split_video_to_equal_parts(
        passed_files.video.to_str().unwrap().to_string(),
        "./output".to_string(),
        audio_file_duration,
        true,
    );

    println!("segments: {:?}", segments)
}
