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
    println!("The video has been split to equal parts.");
    video::add_audio_to_videos(
        &segments,
        passed_files.audio
            .to_str()
            .expect("Failed to convert PathBuf to &str")
            .to_string()
    );
    println!("Audio has been applied to all videos.");
    video::add_image_to_videos(
        &segments,
        passed_files.image
            .to_str()
            .expect("Failed to convert PathBuf to &str")
            .to_string()
    );
    println!("The image has been added to every video.");

    println!("Done! Thank you for using media-merger.");
}
