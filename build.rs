#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    //res.set_resource_file("./resources/clock-3-128.ico");
    //res.set_icon("./resources/clock-3-128.ico");
    //Set up Default App Icon with ID , the same will be used from main code to set up systray icon.
    res.set_icon_with_id("./resources/clock-3-128.ico","IDI_APPLICATION");
    res.compile().unwrap();
    //res.compile()?;
}

#[cfg(unix)]
fn main() {
}