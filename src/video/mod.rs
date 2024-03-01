use crate::files::get_length_of_file;
use std::process::{Command, ExitStatus, exit};
use std::io;

fn split_video(start_milliseconds: i32, length_milliseconds: i32, input_file: &String, output_file: &String) -> ExitStatus {
    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(input_file)
        .arg("-ss")
        .arg(format!("{}", start_milliseconds / 1000))
        .arg("-t")
        .arg(format!("{}", length_milliseconds / 1000))
        .arg(output_file)
        .status()
        .expect("Something went wrong while creating video's segments");

    return status
}

pub fn split_video_to_equal_parts(video: String, output_dir: String, part_length: i32, warning: bool) {
    let videos_length = get_length_of_file(&video);
    let amount_of_videos_to_generate = videos_length / part_length;

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

    for i in 1..amount_of_videos_to_generate {
        let start_time = (i - 1) * part_length;
        let output_filename = format!("{}/{}.mp4", output_dir, i);

        split_video(start_time, part_length, &video, &output_filename);
    }
}
