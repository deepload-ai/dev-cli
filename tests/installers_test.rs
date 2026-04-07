use anyhow::Result;
use devenv_cli::installers::lang;

#[test]
fn test_java_installer_exists() -> Result<()> {
    // TDD Step 1: Red -> Green. Ensure function exists and compiles.
    let _ = lang::install_java();
    Ok(())
}

#[test]
fn test_android_sdk_installer_exists() -> Result<()> {
    // TDD Step 1: Red -> Green. Ensure function exists and compiles.
    let _ = lang::install_android_sdk();
    Ok(())
}

#[test]
fn test_flutter_installer_exists() -> Result<()> {
    // TDD Step 1: Red -> Green. Ensure function exists and compiles.
    let _ = lang::install_flutter();
    Ok(())
}
