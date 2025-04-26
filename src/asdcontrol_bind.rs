pub fn get_bg_value(device: &str) -> i32 {
    let output = std::process::Command::new("asdcontrol")
        .arg("-s")
        .arg("-b")
        .arg(device)
        .output()
        .expect("Failed to execute command");

    let output_str = String::from_utf8_lossy(&output.stdout);
    // trim output_str and convert into integer
    let value = output_str.trim().parse::<i32>().unwrap_or(0);

    let normalized = value / 1000;

    normalized
}

pub fn set_bg_value(device: &str, value: i32) {
    let value_raw = value * 1000;
    
    let output = std::process::Command::new("asdcontrol")
        .arg("-s")
        .arg("-b")
        .arg(device)
        .arg(value_raw.to_string())
        .output()
        .expect("Failed to execute command");

    let output_str = String::from_utf8_lossy(&output.stdout);
    // trim output_str and convert into integer
    let value = output_str.trim().parse::<i32>().unwrap_or(0);

    println!("Set background value: {}", value);
}