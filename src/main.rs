//!
//! The application binary.
//!

mod arguments;

use std::{fs, process::Command};

use tokio::prelude::{future::join_all, *};
use tokio_process::CommandExt;

use arguments::Arguments;

#[cfg(target_os = "windows")]
static FFMPEG_EXE: &'static str = "ffmpeg.exe";
#[cfg(target_os = "windows")]
static MKVMERGE_EXE: &'static str = "mkvmerge.exe";

#[cfg(target_os = "linux")]
static FFMPEG_EXE: &'static str = "ffmpeg";
#[cfg(target_os = "linux")]
static MKVMERGE_EXE: &'static str = "mkvmerge";

fn main() {
    let data = Arguments::from_cmd().unwrap();
    println!("{}", data);

    if data.dummy() {
        let mut args = Vec::with_capacity(data.inputs().len() * 2);
        for input in data.inputs().iter() {
            args.push("-i".to_owned());
            args.push(input.to_owned());
        }
        println!("{} {}", FFMPEG_EXE, args.join(" "));
        let future = Command::new(FFMPEG_EXE)
            .args(args)
            .spawn_async()
            .expect("Failed to spawn a command")
            .map(|_| ())
            .map_err(|error| panic!("{}", error));
        tokio::run(future);
        return;
    }

    let mut children =
        Vec::with_capacity(data.audio_streams().len() + data.subtitle_streams().len());
    children.extend(data.audio_streams().iter().map(|stream| {
        let mut args = Vec::with_capacity(data.inputs().len() * 2 + 9);
        for input in data.inputs().iter() {
            args.push("-i".to_owned());
            args.push(input.to_owned());
        }
        args.push("-map".to_owned());
        args.push(stream.to_owned());
        args.push("-c:a".to_owned());
        args.push("libopus".to_owned());
        args.push("-compression_level".to_owned());
        args.push("10".to_owned());
        args.push("-mapping_family".to_owned());
        args.push("255".to_owned());
        args.push(format!("{}.ogg", stream.replace(":", "_")));

        println!("{} {}", FFMPEG_EXE, args.join(" "));
        Command::new(FFMPEG_EXE)
            .args(args)
            .spawn_async()
            .expect("Failed to spawn a command")
    }));
    children.extend(data.subtitle_streams().iter().map(|stream| {
        let mut args = Vec::with_capacity(data.inputs().len() * 2 + 5);
        for input in data.inputs().iter() {
            args.push("-i".to_owned());
            args.push(input.to_owned());
        }
        args.push("-map".to_owned());
        args.push(stream.to_owned());
        args.push("-c:s".to_owned());
        args.push("copy".to_owned());
        args.push(format!("{}.srt", stream.replace(":", "_")));

        println!("{} {}", FFMPEG_EXE, args.join(" "));
        Command::new(FFMPEG_EXE)
            .args(args)
            .spawn_async()
            .expect("Failed to spawn a command")
    }));

    let mut futures = Vec::with_capacity(children.len());
    for child in children {
        let future = child.map(|_| ()).map_err(|error| panic!("{}", error));
        futures.push(future);
    }
    let encoding = join_all(futures).map(|_| ()).map_err(|error| panic!(error));
    tokio::run(encoding);

    let mut args =
        Vec::with_capacity(16 + data.audio_streams().len() * 5 + data.subtitle_streams().len() * 7);
    args.push("--default-language".to_owned());
    args.push(data.language().to_owned());
    args.push("--title".to_owned());
    args.push(data.title().to_owned());
    args.push("-o".to_owned());
    args.push(data.output_path().to_owned());
    args.push("--language".to_owned());
    args.push(format!(
        "{}:{}",
        data.video_stream(),
        data.language().to_owned()
    ));
    args.push("-A".to_owned());
    args.push("-S".to_owned());
    args.push("-T".to_owned());
    args.push("-M".to_owned());
    args.push("-B".to_owned());
    args.push(data.inputs().get(0).unwrap().to_owned());
    let mut track_name_iter = data.track_names().iter();
    for stream in data.english_streams().iter() {
        args.push("--language".to_owned());
        args.push("0:eng".to_owned());
        args.push("--track-name".to_owned());
        args.push(format!(
            "0:{}",
            track_name_iter.next().unwrap_or(&"Original".to_owned())
        ));
        args.push(format!("{}.ogg", stream.replace(":", "_")));
    }
    for stream in data.ukrainian_streams().iter() {
        args.push("--language".to_owned());
        args.push("0:ukr".to_owned());
        args.push("--track-name".to_owned());
        args.push(format!(
            "0:{}",
            track_name_iter.next().unwrap_or(&"Dub".to_owned())
        ));
        args.push(format!("{}.ogg", stream.replace(":", "_")));
    }
    for stream in data.russian_streams().iter() {
        let title = if data.language() == "rus" {
            "Original"
        } else {
            "Dub"
        };
        args.push("--language".to_owned());
        args.push("0:rus".to_owned());
        args.push("--track-name".to_owned());
        args.push(format!(
            "0:{}",
            track_name_iter.next().unwrap_or(&title.to_owned())
        ));
        args.push(format!("{}.ogg", stream.replace(":", "_")));
    }
    for stream in data.other_streams().iter() {
        let title = if data.language() == "eng" {
            "Dub"
        } else {
            "Original"
        };
        args.push("--language".to_owned());
        args.push(format!("0:{}", data.language().to_owned()));
        args.push("--track-name".to_owned());
        args.push(format!(
            "0:{}",
            track_name_iter.next().unwrap_or(&title.to_owned())
        ));
        args.push(format!("{}.ogg", stream.replace(":", "_")));
    }
    for stream in data.subtitle_streams().iter() {
        let title = if data.language() != "eng" {
            "Dub"
        } else {
            "Original"
        };
        args.push("--language".to_owned());
        args.push("0:eng".to_owned());
        args.push("--track-name".to_owned());
        args.push(format!(
            "0:{}",
            track_name_iter.next().unwrap_or(&title.to_owned())
        ));
        args.push("--default-track".to_owned());
        args.push("0:false".to_owned());
        args.push(format!("{}.srt", stream.replace(":", "_")));
    }

    println!("{} {}", MKVMERGE_EXE, args.join(" "));
    let status = Command::new(MKVMERGE_EXE)
        .args(args)
        .spawn()
        .expect("Failed to spawn a command")
        .wait()
        .expect("Failed to wait for a command");
    if status.success() {
        for stream in data.audio_streams().iter() {
            fs::remove_file(format!("{}.ogg", stream.replace(":", "_")))
                .unwrap_or_else(|_| panic!("Failed to cleanup the {} stream", stream));
        }
        for stream in data.subtitle_streams().iter() {
            fs::remove_file(format!("{}.srt", stream.replace(":", "_")))
                .unwrap_or_else(|_| panic!("Failed to cleanup the {} stream", stream));
        }
    }
}
