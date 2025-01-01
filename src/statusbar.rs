use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

const INTERVAL: u64 = 1;

fn cpu() -> String {
    let output = Command::new("cat")
        .arg("/proc/loadavg")
        .output()
        .expect("Failed to get CPU load")
        .stdout;
    let cpu_val = String::from_utf8_lossy(&output).split_whitespace().next().unwrap().to_string();
    cpu_val
}

fn battery() -> String {
    let get_capacity = std::fs::read_to_string("/sys/class/power_supply/BAT0/capacity")
        .expect("Failed to get battery capacity")
        .trim()
        .to_string();
    format!("{}%", get_capacity)
}

fn brightness() -> String {
    let brightness_val = std::fs::read_to_string("/sys/class/backlight/amdgpu_bl0/brightness")
        .expect("Failed to get brightness")
        .trim()
        .to_string();
    brightness_val
}

fn mem() -> String {
    let output = Command::new("free")
        .arg("-h")
        .output()
        .expect("Failed to get memory usage")
        .stdout;
    let mem_val = String::from_utf8_lossy(&output).lines().nth(1).unwrap().split_whitespace().nth(2).unwrap().to_string();
    mem_val
}

fn wlan() -> String {
    let output = Command::new("iwctl")
        .arg("station")
        .arg("wlan0")
        .arg("show")
        .output()
        .expect("Failed to get WiFi status")
        .stdout;
    let wifi_status = String::from_utf8_lossy(&output).lines().find(|line| line.contains("State")).unwrap().split_whitespace().nth(1).unwrap().to_string();
    if wifi_status == "connected" {
        let wifi_name = String::from_utf8_lossy(&output).lines().find(|line| line.contains("Connected network")).unwrap().split_whitespace().nth(2).unwrap().to_string();
        wifi_name
    } else {
        String::from("Not connected")
    }
}

fn volume() -> String {
    let output = Command::new("pactl")
        .arg("list")
        .arg("sinks")
        .output()
        .expect("Failed to get volume status")
        .stdout;
    let vol = String::from_utf8_lossy(&output).lines().find(|line| line.contains("Volume:")).unwrap().split_whitespace().nth(4).unwrap().to_string().replace('%', "");
    let mute = String::from_utf8_lossy(&output).lines().find(|line| line.contains("Mute:")).unwrap().split_whitespace().nth(1).unwrap().to_string();

    if mute == "yes" {
        String::from("Muted")
    } else {
        format!("{}%", vol)
    }
}

fn clock() -> String {
    let now = chrono::Local::now();
    format!("{} ", now.format("%A, %B %d, %Y %H:%M:%S"))
}

fn update_status() {
    let status = format!(
        "{} {} {} {} {} {} {}",
        battery(),
        brightness(),
        cpu(),
        mem(),
        wlan(),
        volume(),
        clock()
    );
    Command::new("xsetroot")
        .arg("-name")
        .arg(status)
        .output()
        .expect("Failed to update status bar");
}

fn main() {
    if std::env::args().any(|arg| arg == "-d") {
        loop {
            update_status();
            sleep(Duration::from_secs(INTERVAL));
        }
    } else {
        update_status();
    }
}

