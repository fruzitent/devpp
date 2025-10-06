#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Path(#[from] std::path::StripPrefixError),
}

type Result<T> = std::result::Result<T, Error>;

pub const TARGET: &str = "devpp-base";

pub fn write_feature(
    mut w: impl std::io::Write,
    build_info: &devpp_spec::devc::BuildInfo,
    feature: &devpp_spec::feat::Feature,
    options: &std::collections::HashMap<String, String>,
) -> Result<()> {
    writeln!(&mut w, "FROM {TARGET} AS devpp-feature-{id}", id = feature.inner.id)?;
    writeln!(&mut w)?;

    for (key, option) in &feature.inner.options {
        let default = match option {
            devpp_spec::feat::generated::FeatureOption::Variant0 { .. } => unimplemented!(),
            devpp_spec::feat::generated::FeatureOption::Variant1 { default, .. } => default,
            devpp_spec::feat::generated::FeatureOption::Variant2 { default, .. } => default,
        };
        let value = options.get(key).unwrap_or(default);
        writeln!(&mut w, "ARG {key}=\"{value}\"")?;
    }
    writeln!(&mut w)?;

    for (key, value) in &feature.inner.container_env {
        writeln!(&mut w, "ENV {key}=\"{value}\"")?;
    }
    writeln!(&mut w)?;

    let dir_name = feature.entrypoint.parent().unwrap();
    let file_name = feature.entrypoint.file_name().unwrap();

    writeln!(&mut w, "RUN \\")?;
    writeln!(
        &mut w,
        "--mount=type=bind,source={source},target=/features/ \\",
        source = dir_name.strip_prefix(&build_info.context)?.to_str().unwrap(),
    )?;
    writeln!(
        &mut w,
        "/features/{entrypoint}",
        entrypoint = file_name.to_str().unwrap(),
    )?;

    Ok(())
}
