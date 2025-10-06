#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    DevppSpec(#[from] devpp_spec::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Path(#[from] std::path::StripPrefixError),
    #[error("feature {id:?} is dependent on the {dep_id:?}, which was not found in the devcontainer.json")]
    FeatureNotFound { dep_id: String, id: String },
}

type Result<T> = std::result::Result<T, Error>;

pub const TARGET: &str = "devpp-base";

pub fn write_feature(
    mut w: impl std::io::Write,
    build_info: &devpp_spec::devc::BuildInfo,
    config: &devpp_spec::devc::Config,
    devc: &devpp_spec::devc::DevContainer,
    feature: &devpp_spec::feat::Feature,
    options: &std::collections::BTreeMap<String, String>,
) -> Result<()> {
    writeln!(&mut w, "FROM {TARGET} AS devpp-feature-{id}", id = feature.inner.id)?;
    writeln!(&mut w)?;

    for dep_id in &feature.inner.installs_after {
        let dep_key = devc.common.features.get(dep_id);
        let dep_options = dep_key.ok_or(Error::FeatureNotFound {
            dep_id: dep_id.to_string(),
            id: feature.inner.id.clone(),
        })?;
        let dep_reference = devpp_spec::feat::Reference::new(dep_id, config)?;
        let dep_feature = devpp_spec::feat::Feature::new(&dep_reference)?;
        write_feature_dep(&mut w, &dep_feature, dep_options)?;
        writeln!(&mut w)?;
    }

    let mut has_args = false;
    for (key, option) in &feature.inner.options {
        let default = match option {
            devpp_spec::feat::generated::FeatureOption::Variant0 { .. } => unimplemented!(),
            devpp_spec::feat::generated::FeatureOption::Variant1 { default, .. } => default,
            devpp_spec::feat::generated::FeatureOption::Variant2 { default, .. } => default,
        };
        let value = options.get(key).unwrap_or(default);
        writeln!(&mut w, "ARG {key}=\"{value}\"", key = key.to_uppercase())?;
        has_args = true;
    }
    if has_args {
        writeln!(&mut w)?;
    }

    let mut has_envs = false;
    for (key, value) in &feature.inner.container_env {
        writeln!(&mut w, "ENV {key}=\"{value}\"")?;
        has_envs = true;
    }
    if has_envs {
        writeln!(&mut w)?;
    }

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

pub fn write_feature_dep(
    mut w: impl std::io::Write,
    feature: &devpp_spec::feat::Feature,
    options: &std::collections::BTreeMap<String, String>,
) -> Result<()> {
    writeln!(
        &mut w,
        "# @see: [acquire.sh](https://github.com/devcontainers/spec/issues/21)"
    )?;

    for (key, option) in &feature.inner.options {
        let default = match option {
            devpp_spec::feat::generated::FeatureOption::Variant0 { .. } => unimplemented!(),
            devpp_spec::feat::generated::FeatureOption::Variant1 { default, .. } => default,
            devpp_spec::feat::generated::FeatureOption::Variant2 { default, .. } => default,
        };
        let value = options.get(key).unwrap_or(default);
        writeln!(&mut w, "ARG {key}=\"{value}\"", key = key.to_uppercase())?;
    }

    for (key, value) in &feature.inner.container_env {
        writeln!(&mut w, "ENV {key}=\"{value}\"")?;
    }

    writeln!(
        &mut w,
        "COPY --from=devpp-feature-{id} \"/opt/{id}\" \"/opt/{id}\"",
        id = feature.inner.id
    )?;

    Ok(())
}
