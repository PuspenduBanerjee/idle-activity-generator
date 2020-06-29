cargo clean && ^
cargo build --release && ^
mkdir target\\release\\stripped && ^
strip target/release/activity_maker_windows.exe -o target/release/stripped/activity_maker_windows.exe