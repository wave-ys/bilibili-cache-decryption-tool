use std::error::Error;
use std::fs::{File, remove_file};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;
use std::process::Command;

use clap::Parser;

#[derive(Parser)]
#[command(name = "Bilibili Cache Decryption Tool", bin_name = "bcdt", version, long_about = None)]
#[command(about = "\
    Bilibili Cache Decryption Tool\n\n\
    You can use this tool to decrypt the M4S file and convert it to MP4/MP3.\n\
    Note: You need to install FFmpeg if you want to merge the video and audio.\
")]
struct Cli {
    /// Path of the output file
    #[arg(short, long)]
    output: String,

    /// Path of the video M4S file
    #[arg(short, long)]
    video: Option<String>,

    /// Path of the audio M4S file
    #[arg(short, long)]
    audio: Option<String>,
}

fn decrypt_file(input_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(input_path)?;

    let mut header = vec![0; 23];
    file.seek(SeekFrom::Start(9))?;
    file.read_exact(&mut header)?;

    for i in header.iter_mut() {
        if *i == b'$' {
            *i = b' ';
        }
    }
    let mut new_header = Vec::<u8>::new();
    let mut index = 0;
    while index < header.len() {
        if header[index] == b'$' {
            header[index] = b' ';
        }
        if index < header.len() - 4 && &header[index..(index + 4)] == b"avc1" {
            index += 4;
            continue;
        }
        new_header.push(header[index]);
        index += 1;
    }

    let mut decrypted = File::create(output_path)?;
    decrypted.write_all(new_header.as_slice())?;

    let mut buffer = vec![0; 256 * 1024 * 1024];
    loop {
        let length = file.read(&mut buffer)?;
        if length == 0 {
            break;
        }
        decrypted.write(&buffer[..length])?;
    }

    Ok(())
}

fn merge_video_and_audio(video_path: &str, audio_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    let check = Command::new("ffmpeg")
        .arg("-version")
        .output();
    if check.is_err() {
        remove_file(video_path)?;
        remove_file(audio_path)?;
        return Err("You need to install FFmpeg if you want to merge the video and audio".into());
    }

    let output = Command::new("ffmpeg")
        .arg("-i").arg(video_path)
        .arg("-i").arg(audio_path)
        .arg("-c").arg("copy")
        .arg("-y")
        .arg(output_path)
        .output()?;
    if !output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stderr));
        return Err("Failed to execute FFmpeg".into());
    }
    remove_file(video_path)?;
    remove_file(audio_path)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let output_path = Path::new(cli.output.as_str());
    if output_path.is_dir() {
        return Err("Output path must be a file path, not a directory".into());
    }

    let output_dir = output_path.parent().unwrap();
    let output_file = output_path.file_name().unwrap().to_str().unwrap();

    if let Some(video) = cli.video.as_deref() {
        let tmp = output_file.to_owned() + ".tmp.mp4";
        decrypt_file(video, output_dir.join(
            if cli.audio.is_none() { output_file } else { tmp.as_str() }
        ).to_str().unwrap())?;
    }

    if let Some(audio) = cli.audio.as_deref() {
        let tmp = output_file.to_string() + ".tmp.mp3";
        decrypt_file(audio, output_dir.join(
            if cli.video.is_none() { output_file } else { tmp.as_str() }
        ).to_str().unwrap())?;
    }

    if cli.audio.is_some() && cli.video.is_some() {
        merge_video_and_audio(
            output_dir.join(output_file.to_string() + ".tmp.mp4").to_str().unwrap(),
            output_dir.join(output_file.to_string() + ".tmp.mp3").to_str().unwrap(),
            output_dir.join(output_file).to_str().unwrap())?;
    }

    Ok(())
}
