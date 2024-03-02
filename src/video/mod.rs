use crate::files::{get_length_of_file, get_videos_fps};
extern crate pbr;
use std::process::{Command, exit, Stdio, ChildStdout};
use std::io;
use std::path::{PathBuf, Path};
use std::fs::{remove_file, rename};
use std::io::{BufRead, BufReader, Stdout};
use pbr::ProgressBar;

fn update_progress_bar(stdout: &mut ChildStdout, pb: &mut ProgressBar<Stdout>) {
    let reader = BufReader::new(stdout);

    for potential_line in reader.lines() {
        let line = potential_line.expect("Failed to read a line!");
        if !line.starts_with("frame=") { continue };

        let current_frame = &line
            .split("=")
            .collect::<Vec<&str>>()[1]
            .parse::<u64>()
            .expect("Failed to convert to u32!");

        pb.set(*current_frame);
    }
}

fn split_video(start_milliseconds: i32, length_milliseconds: i32, input_file: &String, output_file: &String) -> PathBuf {
    let length_seconds = length_milliseconds / 1000;
    let total_frames = length_seconds * get_videos_fps(input_file);
    let mut pb = ProgressBar::new(total_frames as u64);
    pb.format("[=>#]");

    let mut output = Command::new("ffmpeg")
        .arg("-i")
        .arg(input_file)
        .arg("-ss")
        .arg(format!("{}", start_milliseconds / 1000))
        .arg("-t")
        .arg(format!("{}", length_milliseconds / 1000))
        .arg("-progress")
        .arg("pipe:1")
        .arg(output_file)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Something went wrong while creating video's segments");

    if let Some(ref mut stdout) = output.stdout {
        update_progress_bar(stdout, &mut pb);
    }

    pb.finish_print("Success!");

    let video_path = Path::new(output_file);
    return video_path.to_path_buf();
}

fn add_audio_to_video(video: &String, audio: &String) {
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

    let length_seconds = get_length_of_file(video) / 1000;
    let total_frames = length_seconds * get_videos_fps(video);
    let mut pb = ProgressBar::new(total_frames as u64);
    pb.format("[=>#]");

    let mut output = Command::new("ffmpeg")
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
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Something went wrong while applying audio to videos.");

    if let Some(ref mut stdout) = output.stdout {
        update_progress_bar(stdout, &mut pb);
    }

    // replace old video with the new one
    remove_file(video).expect("Failed to remove a video!");
    rename(output_filename.to_string(), video).expect("Failed to rename a video!");

    pb.finish_print("Success!");
}

fn add_image_to_video(video: &String, image: &String) {
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

    let length_seconds = get_length_of_file(video) / 1000;
    let total_frames = length_seconds * get_videos_fps(video);
    let mut pb = ProgressBar::new(total_frames as u64);
    pb.format("[=>#]");

    let mut output = Command::new("ffmpeg")
        .arg("-i")
        .arg(video)
        .arg("-i")
        .arg(image)
        .arg("-filter_complex")
        .arg("[0:v][1:v] overlay=(W-w)/2:(H-h)/2")
        .arg("-c:a")
        .arg("copy")
        .arg(&output_filename)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Something went wrong while adding image to videos.");

    if let Some(ref mut stdout) = output.stdout {
        update_progress_bar(stdout, &mut pb);
    }

    // replace old video with the new one
    remove_file(video).expect("Failed to remove a video!");
    rename(output_filename.to_string(), video).expect("Failed to rename a video!");

    pb.finish_print("Success!");
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

        let video_path = split_video(start_time, part_length, &video, &output_filename);
        videos.push(video_path);
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
