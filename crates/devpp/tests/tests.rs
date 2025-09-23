#[test]
fn config_illegal_path() {
    let workspace = std::path::PathBuf::from(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/config_illegal_path/workspace"
    ));
    let config = workspace.join("../devcontainer.json");
    assert!(matches!(
        devpp::devcontainer::find_config(workspace.to_path_buf(), Some(config)),
        Err(devpp::devcontainer::Error::NotFoundConfigArg { .. }),
    ));
}
