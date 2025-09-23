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

#[test]
fn config_missing() {
    let workspace = std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/config_missing"));
    assert!(matches!(
        devpp::build(workspace.to_path_buf(), None),
        Err(devpp::Error::DevContainer(devpp::devcontainer::Error::NotFoundConfig)),
    ));
}

#[test]
fn config_plain() {
    let workspace = std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/config_plain"));
    assert!(matches!(
        devpp::build(workspace.to_path_buf(), None),
        Err(devpp::Error::DevContainer(devpp::devcontainer::Error::NotFoundDotDev)),
    ));
}

#[test]
fn feature_mismatch_id() {
    let workspace = std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/feature_mismatch_id"));
    assert!(matches!(
        devpp::build(workspace.to_path_buf(), None),
        Err(devpp::Error::Feature(devpp::feature::Error::MismatchId { .. })),
    ));
}

#[test]
fn featureref_absolute() {
    let workspace = std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/featureref_absolute"));
    assert!(matches!(
        devpp::build(workspace.to_path_buf(), None),
        Err(devpp::Error::Feature(devpp::feature::Error::PathAbsolute { .. })),
    ));
}

#[test]
fn featureref_local_illegal() {
    let workspace = std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/featureref_local_illegal"));
    assert!(matches!(
        devpp::build(workspace.to_path_buf(), None),
        Err(devpp::Error::Feature(devpp::feature::Error::PathIllegal { .. })),
    ));
}

#[test]
fn featureref_local_traversal() {
    let workspace = std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/featureref_local_traversal"));
    let config = workspace.join(".devcontainer/my_image/devcontainer.json");
    assert!(matches!(devpp::build(workspace.to_path_buf(), Some(config)), Ok(())));
}

#[test]
fn featureref_missing_install() {
    let workspace = std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/featureref_missing_install"));
    assert!(matches!(
        devpp::build(workspace.to_path_buf(), None),
        Err(devpp::Error::Feature(devpp::feature::Error::NotFoundEntry { .. })),
    ));
}

#[test]
fn featureref_missing_metadata() {
    let workspace = std::path::PathBuf::from(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/featureref_missing_metadata"
    ));
    assert!(matches!(
        devpp::build(workspace.to_path_buf(), None),
        Err(devpp::Error::Feature(devpp::feature::Error::NotFoundMetadata { .. })),
    ));
}

#[test]
fn featureref_remote() {
    let workspace = std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/featureref_remote"));
    assert!(matches!(devpp::build(workspace.to_path_buf(), None), Ok(())));
}
