#![windows_subsystem = "windows"]

#[cfg(windows)]
extern crate winapi;

use std::thread;
use std::time::Duration;
use enigo::*;
use std::sync::{Arc, Mutex};


// #[cfg(windows)]
// fn print_message(msg: &str) -> Result<i32, Error> {
//     use std::ffi::OsStr;
//     use std::iter::once;
//     use std::os::windows::ffi::OsStrExt;
//     use std::ptr::null_mut;
//     use winapi::um::winuser::{MB_OK, MessageBoxW};
//     let wide: Vec<u16> = OsStr::new(msg).encode_wide().chain(once(0)).collect();
//     let ret = unsafe {
//         MessageBoxW(null_mut(), wide.as_ptr(), wide.as_ptr(), MB_OK)
//     };
//     if ret == 0 { Err(Error::last_os_error()) }
//     else { Ok(ret) }
// }


fn last_activity_time() -> u32 {
    use user_idle::UserIdle;
    let idle = UserIdle::get_time().unwrap();
    let idle_seconds = idle.as_seconds();
    // let idle_minutes = idle.as_minutes();
    return idle_seconds;
}

// #[cfg(not(windows))]
// fn print_message(msg: &str) -> Result<(), Error> {
//     println!("{}", msg);
//     Ok(())
// }

fn main() -> Result<(), systray::Error> {
    let mut app;
    match systray::Application::new() {
        Ok(w) => app = w,
        Err(_) => panic!("Can't create window!"),
    }
    app.set_icon_from_resource("IDI_APPLICATION")?;
    app.set_tooltip("Alive and Kicking")?;

    let should_pause = Arc::new(Mutex::new(false));
    {
        let mt_should_pause = Arc::clone(&should_pause);
        app.add_menu_item("Pause", move |window| {
            let mut should_pause2 =mt_should_pause.lock().unwrap();
            *should_pause2=!*should_pause2;
            println!("Paused => {}",*should_pause2);
            if *should_pause2 {
                window.set_icon_from_resource("PAUSED_ICON")?;
                window.set_tooltip("Paused")?;

            }else{
                window.set_icon_from_resource("IDI_APPLICATION")?;
                window.set_tooltip("Alive and Kicking")?;
            }
            Ok::<_, systray::Error>(())
        })?;
    }
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

    //Increment reference counter before passing to thread
    //let t_should_pause = Arc::clone(&should_pause);
    //As we will be reading the value of should_pause, so avoiding cloning.
    thread::spawn(move || {
        let mut enigo = Enigo::new();
        let threshold_duration_secs: u32 = 59;
        const CHECK_INTERVAL_SECS: u64 = 61;
        let activity_duration_millis = 1000 * 5;
        let scroll_length = 6; //in number of notches ,it will be multiplied by WHEEL_DELTA=120
        // let activity_steps=activity_duration_millis/(scroll_length*2);
        let sleep_per_step =
            Duration::from_millis(activity_duration_millis / scroll_length);

        loop {
            let  should_pause_val = *should_pause.lock().unwrap();
            if !should_pause_val {
                let last_activity_sec = last_activity_time();
                println!("last activity {}s ago", last_activity_sec);
                if last_activity_sec > threshold_duration_secs {
                    generate_activity(&mut enigo, scroll_length, sleep_per_step)
                }
            }
            thread::sleep(Duration::from_secs(CHECK_INTERVAL_SECS));
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

