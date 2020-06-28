#![windows_subsystem = "windows"]

#[cfg(windows)]
extern crate winapi;
// use std::ffi::CString;
use std::io::Error;
use std::thread;
use std::time::Duration;

use enigo::*;


#[cfg(windows)]
fn print_message(msg: &str) -> Result<i32, Error> {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr::null_mut;
    use winapi::um::winuser::{MB_OK, MessageBoxW};
    let wide: Vec<u16> = OsStr::new(msg).encode_wide().chain(once(0)).collect();
    let ret = unsafe {
        MessageBoxW(null_mut(), wide.as_ptr(), wide.as_ptr(), MB_OK)
    };
    if ret == 0 { Err(Error::last_os_error()) }
    else { Ok(ret) }
}


fn last_activity_time() -> u32 {
    use user_idle::UserIdle;
    let idle = UserIdle::get_time().unwrap();
    let idle_seconds = idle.as_seconds();
    // let idle_minutes = idle.as_minutes();
    return idle_seconds;
}

#[cfg(not(windows))]
fn print_message(msg: &str) -> Result<(), Error> {
    println!("{}", msg);
    Ok(())
}

fn main() -> Result<(), systray::Error> {
    let mut app;
    match systray::Application::new() {
        Ok(w) => app = w,
        Err(_) => panic!("Can't create window!"),
    }
    // w.set_icon_from_file(&"C:\\Users\\qdot\\code\\git-projects\\systray-rs\\resources\\rust.ico".to_string());
    // w.set_tooltip(&"Whatever".to_string());
    //app.set_icon_from_file("./resources/clock-3-128.ico")?;
    app.set_icon_from_resource("IDI_APPLICATION_ICON")?;
    app.set_tooltip("Alive and Kicking")?;

    app.add_menu_item("Pause", |_| {
        println!("Pausing!");
        Ok::<_, systray::Error>(())
    })?;
    //
    // app.add_menu_item("Add Menu Item", |window| {
    //     window.add_menu_item("Interior item", |_| {
    //         println!("what");
    //         Ok::<_, systray::Error>(())
    //     })?;
    //     window.add_menu_separator()?;
    //     Ok::<_, systray::Error>(())
    // })?;

    //app.add_menu_separator()?;

    app.add_menu_item("Quit", |window| {
        window.quit();
        Ok::<_, systray::Error>(())
    })?;

    thread::spawn(|| {
        let mut enigo = Enigo::new();
        let threshold_duration_secs:u32=59;
        let activity_duration_millis=1000*10;
        let scroll_length=5; //in number of notches ,it will be multiplied by WHEEL_DELTA=120
        // let activity_steps=activity_duration_millis/(scroll_length*2);
        let sleep_per_step =
            Duration::from_millis(activity_duration_millis/ scroll_length);

        loop {
            let last_activity_sec=last_activity_time();
            println!("last activity {}s ago",last_activity_sec );
            if last_activity_sec>threshold_duration_secs{
                generate_activity(&mut enigo, scroll_length, sleep_per_step)
            }
            thread::sleep(Duration::from_secs(61));
        }
    });
    println!("Waiting on message!");
    app.wait_for_message()?;
    Ok(())
}

fn generate_activity(enigo: &mut Enigo, scroll_length: u64, sleep_per_step: Duration) {
    println!("Generating Activity. Steps {}, Sleep Time per Step {}ms",
             scroll_length, sleep_per_step.as_millis());

    for _i in 1..scroll_length {mouse_scroll_y(enigo,1,sleep_per_step)}
    for _i in 1..scroll_length {mouse_scroll_y(enigo,-1,sleep_per_step)}
}

fn mouse_scroll_y(enigo: &mut Enigo, scroll_length:i32,sleep_per_step:Duration){
    &enigo.mouse_scroll_y(scroll_length);
    thread::sleep(sleep_per_step);
}

// fn main() {
//     print_message(last_activity_time().to_string().as_str()).unwrap();
// }

// fn main() {
//     println!("Hello, world!");
// }