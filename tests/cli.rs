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
