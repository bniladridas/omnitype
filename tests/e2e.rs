use std::process::Command;

#[test]
fn test_check_sample_py() {
    let output = Command::new("cargo")
        .args(&["run", "--", "check", "tests/sample.py"])
        .output()
        .expect("Failed to run omnitype check");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let _stderr = String::from_utf8_lossy(&output.stderr);

    // Check that it found functions and classes
    assert!(stdout.contains("functions=8, classes=1"), "Expected function/class count in output");

    // Check for warnings about missing annotations
    assert!(stdout.contains("warning Missing"), "Expected warnings about missing annotations");

    // Should exit with code 1 due to diagnostics
    assert!(!output.status.success(), "Expected non-zero exit code for diagnostics");
}

#[test]
fn test_check_classes_py() {
    let output = Command::new("cargo")
        .args(&["run", "--", "check", "tests/classes.py"])
        .output()
        .expect("Failed to run omnitype check");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check that it found functions and classes
    assert!(stdout.contains("functions=7, classes=1"), "Expected function/class count in output");

    // Check for warnings about missing annotations
    assert!(stdout.contains("warning Missing"), "Expected warnings about missing annotations");

    // Should exit with code 1 due to diagnostics
    assert!(!output.status.success(), "Expected non-zero exit code for diagnostics");
}