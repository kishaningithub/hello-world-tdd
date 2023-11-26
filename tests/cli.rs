use assert_cmd::Command;

#[test]
fn test_cli_app_should_print_hello_tdd_world() {
    // The article at https://dev.to/kishanbsh/used-tdd-approach-for-a-hello-world-rust-cli-app-1b18
    // has a typo `cargo new hello-tdd-world`.
    // Use the name in Cargo.toml
    let mut cmd = Command::cargo_bin("hello-world-tdd").unwrap();
    cmd.assert().success().stdout("Hello TDD world!\n");
}
