use crate::files::get_length_of_file;
use std::process::{Command, ExitStatus, exit, Stdio};
use std::io;
use std::path::{PathBuf, Path};
use std::fs::{remove_file, rename};

struct GeneratedVideoInfo {
    status: ExitStatus,
    path: PathBuf,
}

fn split_video(start_milliseconds: i32, length_milliseconds: i32, input_file: &String, output_file: &String) -> GeneratedVideoInfo {
    let exit_status = Command::new("ffmpeg")
        // .stdout(Stdio::null())
        // .stderr(Stdio::null())
        .arg("-i")
        .arg(input_file)
        .arg("-ss")
        .arg(format!("{}", start_milliseconds / 1000))
        .arg("-t")
        .arg(format!("{}", length_milliseconds / 1000))
        .arg(output_file)
        .status()
        .expect("Something went wrong while creating video's segments");
    let video_path = Path::new(output_file);

    return GeneratedVideoInfo {
        status: exit_status,
        path: video_path.to_path_buf(),
    }
}

fn add_audio_to_video(video: &String, audio: &String) -> ExitStatus {
    let video_without_extension = Path::new(video)
        .file_name()
        .expect("Couldn't find passed video!")
        .to_str()
        .expect("Wasn't able to convert to &str");
    let video_parent_folder = Path::new(video)
        .parent()
        .expect("Failed to get video's parent dir!")
        .to_str()
        .expect("Wasn't able to convert video's parent directory name to &str");
    let output_filename = format!("{}/{}_temp.mp4", video_parent_folder, video_without_extension);

    let exit_status = Command::new("ffmpeg")
        .arg("-i")
        .arg(video)
        .arg("-i")
        .arg(audio)
        .arg("-filter_complex")
        .arg("[0:a][1:a]amix=inputs=2:duration=longest[outa]")
        .arg("-map")
        .arg("0:v")
        .arg("-map")
        .arg("[outa]")
        .arg("-c:v")
        .arg("copy")
        .arg(&output_filename)
        .status()
        .expect("Something went wrong while applying audio to videos.");

    // replace old video with the new one
    remove_file(video).expect("Failed to remove a video!");
    rename(output_filename.to_string(), video).expect("Failed to rename a video!");

    return exit_status;
}

fn add_image_to_video(video: &String, image: &String) -> ExitStatus {
    let video_without_extension = Path::new(video)
        .file_name()
        .expect("Couldn't find passed video!")
        .to_str()
        .expect("Wasn't able to convert to &str");
    let video_parent_folder = Path::new(video)
        .parent()
        .expect("Failed to get video's parent dir!")
        .to_str()
        .expect("Wasn't able to convert video's parent directory name to &str");
    let output_filename = format!("{}/{}_temp.mp4", video_parent_folder, video_without_extension);

    let exit_status = Command::new("ffmpeg")
        .arg("-i")
        .arg(video)
        .arg("-i")
        .arg(image)
        .arg("-filter_complex")
        .arg("[0:v][1:v] overlay=(W-w)/2:(H-h)/2")
        .arg("-c:a")
        .arg("copy")
        .arg(&output_filename)
        .status()
        .expect("Something went wrong while adding image to videos.");

    // replace old video with the new one
    remove_file(video).expect("Failed to remove a video!");
    rename(output_filename.to_string(), video).expect("Failed to rename a video!");

    return exit_status;
}

pub fn split_video_to_equal_parts(video: String, output_dir: String, part_length: i32, warning: bool) -> Vec<PathBuf> {
    let videos_length = get_length_of_file(&video);
    let amount_of_videos_to_generate = videos_length / part_length;
    let mut videos: Vec<PathBuf> = vec![];

    if warning {
        let mut input = String::new();
        println!(
            "You're about to generate {} videos, are you sure that you want to continue? [Y/n]",
            amount_of_videos_to_generate
        );
        io::stdin().read_line(&mut input).expect("Failed to read line.");

        let proceeding = input.trim().to_lowercase() == String::from("y");
        if proceeding {
            println!("Proceeding..");
        } else {
            println!("Exiting..");
            exit(0);
        }
    }

    for i in 1..=amount_of_videos_to_generate {
        let start_time = (i - 1) * part_length;
        let output_filename = format!("{}/{}.mp4", output_dir, i);

        let video = split_video(start_time, part_length, &video, &output_filename);
        videos.push(video.path);
    }

    return videos;
}

pub fn add_audio_to_videos(videos: &Vec<PathBuf>, audio: String) {
    for video in videos {
        add_audio_to_video(
            &video
                .to_str()
                .expect("Failed to convert path to &str")
                .to_string(),
            &audio,
        );
    }
}

pub fn add_image_to_videos(videos: &Vec<PathBuf>, image: String) {
    for video in videos {
        add_image_to_video(
            &video
                .to_str()
                .expect("Failed to convert path to &str")
                .to_string(),
            &image,
        );
    }
}
