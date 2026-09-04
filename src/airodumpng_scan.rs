use std::path::Path;
use std::process::Command;

fn wirte_scan_result(path: &Path) {
    let path = format!("{}/result", path.to_string_lossy());
    Command::new("airodump-ng")
        .arg("wlan0mon")
        .arg("--output-format")
        .arg("csv")
        .arg("-w")
        .arg(path)
        .output()
        .expect("Failed to start airodump-ng");
}
