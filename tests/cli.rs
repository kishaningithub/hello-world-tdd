use assert_cmd::Command;

#[test]
fn test_cli_app_should_print_hello_tdd_world() {
    let mut cmd = Command::cargo_bin("hello-world-tdd").unwrap();
    cmd.assert().success().stdout("Hello TDD world!\n");
}
