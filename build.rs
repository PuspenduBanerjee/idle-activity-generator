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
    let mut icon_resources=HashMap::new();
    icon_resources.insert("PAUSED_ICON","./resources/icons8_clock_64_9Ls_icon.ico");
    res.add_icon_with_id(icon_resources);
    res.compile().unwrap()
}

#[cfg(unix)]
fn main() {
}

trait WindowsResourceEx{
    fn add_icon_with_id(&mut self, icon_resources:HashMap<&str,&str>);
}

use std::collections::HashMap;
impl WindowsResourceEx for winres::WindowsResource{
    fn add_icon_with_id(&mut self, icon_resources:HashMap<&str,&str>) {
        let tmp_rc_file_path:String=tempfile().unwrap().path().unwrap().to_str().unwrap().to_string();
        eprintln!("rc file path => {}",tmp_rc_file_path);
        self.write_resource_file(&tmp_rc_file_path).unwrap();
        let mut file = OpenOptions::new().append(true).open(&tmp_rc_file_path)
            .expect("cannot open file");
        for (&icon_id, &path) in icon_resources.iter() {
            if let Err(e) = writeln!(file, "{} ICON \"{}\"", icon_id, path) {
                eprintln!("Couldn't write to file: {}", e);
            };
        }
        self.set_resource_file(&tmp_rc_file_path);
    }
}