use std::process::Command;

#[test]
fn version_subcommand_prints_version() {
    let out = Command::new(env!("CARGO_BIN_EXE_workctl"))
        .arg("version")
        .output()
        .unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8(out.stdout).unwrap();
    assert_eq!(
        stdout.trim(),
        concat!("workctl ", env!("CARGO_PKG_VERSION"))
    );
}

fn init_in_empty_data_dir(args: &[&str]) -> std::process::Output {
    let dir = std::env::temp_dir().join(format!("workctl-cli-test-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let out = Command::new(env!("CARGO_BIN_EXE_workctl"))
        .arg("init")
        .args(args)
        .env("XDG_DATA_HOME", &dir)
        .env("LOCALAPPDATA", &dir)
        .output()
        .unwrap();
    // A bad argument must be rejected before anything is created.
    assert!(!dir.exists(), "nothing should be written for {args:?}");
    out
}

#[test]
fn init_rejects_a_release_date_as_python_version() {
    let out = init_in_empty_data_dir(&["--python-version", "20261111"]);
    assert!(!out.status.success());
    let stderr = String::from_utf8(out.stderr).unwrap();
    assert!(stderr.contains("invalid Python version"), "{stderr}");
    assert!(stderr.contains("3.14.15"), "{stderr}");
}

#[test]
fn init_has_no_release_flag() {
    let out = init_in_empty_data_dir(&["--release", "20261111"]);
    assert!(!out.status.success());
    let stderr = String::from_utf8(out.stderr).unwrap();
    assert!(stderr.contains("--release"), "{stderr}");
}
