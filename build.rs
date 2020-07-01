#[cfg(windows)]
extern crate winres;

use std::fs::OpenOptions;
use std::io::prelude::*;
use filepath::FilePath;
use tempfile::tempfile;

#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon_with_id("./resources/clock-3-128.ico","IDI_APPLICATION");
    let tmp_rc_file_path:String=tempfile().unwrap().path().unwrap().to_str().unwrap().to_string();
    eprintln!("rc file path => {}",tmp_rc_file_path);
    res.write_resource_file(&tmp_rc_file_path).unwrap();
    // We need more icons :P
    let mut file = OpenOptions::new().append(true).open(&tmp_rc_file_path)
        .expect("cannot open file");
    if let Err(e) = writeln!(file,
                             "PAUSED_ICON ICON \"./resources/icons8_clock_64_9Ls_icon.ico\"") {
        eprintln!("Couldn't write to file: {}", e);
    };
    res.set_resource_file(&tmp_rc_file_path);
    res.compile().unwrap()
}

#[cfg(unix)]
fn main() {
}
