use std::process::Command;

#[test]
fn help_command_lists_available_actions() {
    let output = Command::new(env!("CARGO_BIN_EXE_leonix"))
        .arg("help")
        .output()
        .expect("LEONIX help command should start");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("info"));
    assert!(stdout.contains("bench"));
}

#[test]
fn info_command_reports_platform() {
    let output = Command::new(env!("CARGO_BIN_EXE_leonix"))
        .arg("info")
        .output()
        .expect("LEONIX info command should start");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Operating system"));
    assert!(stdout.contains("Architecture"));
}

#[test]
fn zero_benchmark_iterations_are_rejected() {
    let output = Command::new(env!("CARGO_BIN_EXE_leonix"))
        .args(["bench", "0"])
        .output()
        .expect("LEONIX benchmark command should start");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("iteration count must be greater than zero"));
}
