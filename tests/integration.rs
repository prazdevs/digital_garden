use assert_cmd::Command;
use color_eyre::eyre::Result;

#[test]
fn test_help() -> Result<()> {
  let mut cmd = Command::cargo_bin("garden")?;
  cmd.arg("--help");
  let assert = cmd.assert();
  assert.success().stderr("");
  Ok(())
}
