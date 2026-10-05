use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::process::Command;

use serde_json::json;

fn main() -> Result<(), Box<dyn Error>> {
    if std::env::args().count() != 2 {
        return Err(
            "Usage: ascii_to_dur <ascii_file_path>\ntype 'ascii_to_dur help' for info".into(),
        );
    }

    let ascii_filename = std::env::args().nth(1).ok_or("Could not read argument")?;

    if &ascii_filename == "help" {
        println!(
            "Transforms an ASCII text into a .dur file (version 7) with the same name as the file in input"
        );
        return Ok(());
    }

    let ascii = fs::read_to_string(&ascii_filename)?;

    let mut size_x: usize = 0;
    ascii.lines().for_each(|l| {
        size_x = std::cmp::max(size_x, l.len());
    });
    size_x += 1;

    let content_vec: Vec<String> = ascii
        .lines()
        .map(|l| {
            let base = String::from(l);
            let padding_amt = size_x - base.len();
            let pad = String::from(' ').repeat(padding_amt);
            base + &pad
        })
        .collect();

    let size_y: usize = content_vec.len();

    let unit = Vec::from([(8usize, 0usize)]).repeat(size_y);
    let mut color_map = Vec::new();
    for _ in 0..size_x {
        color_map.push(unit.clone());
    }

    let json_value = json!({
        "DurMovie": {
            "formatVersion": 7,
            "colorFormat": "16",
            "preferredFont": "fixed",
            "encoding": "utf-8",
            "name": "",
            "artist": "",
            "framerate": 6.0,
            "sizeY": size_y,
            "sizeX": size_x,
            "extra": null,
            "frames": [
            {
                "frameNumber": 1,
                "delay": 0,
                "contents": content_vec,
                "colorMap": color_map
            }
            ]
        }
    });

    let json_str = serde_json::to_string_pretty(&json_value)?;
    let mut output = File::create("__output.json")?;
    output.write_all(json_str.as_bytes())?;

    Command::new("gzip").arg("__output.json").status()?;

    fs::rename("__output.json.gz", format!("{ascii_filename}.dur"))?;

    Ok(())
}
