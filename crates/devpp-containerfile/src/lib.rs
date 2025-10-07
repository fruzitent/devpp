#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    DevppSpec(#[from] devpp_spec::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Path(#[from] std::path::StripPrefixError),
    #[error(transparent)]
    Regex(#[from] regex::Error),
    #[error("target stage must be set")]
    TargetNotFound,
}

type Result<T> = std::result::Result<T, Error>;

pub const TARGET: &str = "devpp-base";

pub fn write_base(mut w: impl std::io::Write, build_info: &devpp_spec::devc::BuildInfo) -> Result<()> {
    let s = std::fs::read_to_string(&build_info.containerfile)?;
    let re = regex::Regex::new(&format!(
        "(?im)AS(\\s+){target}$",
        target = build_info.target.as_ref().ok_or(Error::TargetNotFound)?,
    ))?;
    let s = re.replace(&s, format!("AS {TARGET}"));
    writeln!(&mut w, "{s}")?;
    Ok(())
}

type Options = std::collections::BTreeMap<String, String>;

pub fn write_feature(
    mut w: impl std::io::Write,
    build_info: &devpp_spec::devc::BuildInfo,
    #[cfg(feature = "devpp")] customizations: &devpp_spec::devpp::Customizations,
    feature: &devpp_spec::feat::Feature,
    features: &std::collections::BTreeMap<&String, (devpp_spec::feat::Feature, &Options)>,
    options: &Options,
) -> Result<()> {
    writeln!(&mut w, "FROM {TARGET} AS devpp-feature-{id}", id = feature.inner.id)?;
    writeln!(&mut w)?;

    for dep_id in &feature.inner.installs_after {
        let (dep_feature, dep_options) = features.get(dep_id).unwrap();
        write_feature_dep(
            &mut w,
            #[cfg(feature = "devpp")]
            build_info,
            dep_feature,
            dep_options,
        )?;
        writeln!(&mut w)?;
    }

    for (key, option) in &feature.inner.options {
        let default = match option {
            devpp_spec::feat::generated::FeatureOption::Variant0 { .. } => unimplemented!(),
            devpp_spec::feat::generated::FeatureOption::Variant1 { default, .. } => default,
            devpp_spec::feat::generated::FeatureOption::Variant2 { default, .. } => default,
        };
        let value = options.get(key).unwrap_or(default);
        writeln!(&mut w, "ARG {key}=\"{value}\"", key = key.to_uppercase())?;
    }
    if !feature.inner.options.is_empty() {
        writeln!(&mut w)?;
    }

    for (key, value) in &feature.inner.container_env {
        writeln!(&mut w, "ENV {key}=\"{value}\"")?;
    }
    if !feature.inner.container_env.is_empty() {
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

    #[cfg(feature = "devpp")]
    if let Some(devpp) = &customizations.0.devpp {
        for mount in &devpp.mounts {
            match mount {
                devpp_spec::devpp::generated::DevppCustomizationsDevppMountsItem::Variant0(mount) => {
                    writeln!(
                        &mut w,
                        "--mount=type={type_},target={target},sharing={sharing} \\",
                        sharing = mount.sharing,
                        target = mount.target,
                        type_ = mount.type_,
                    )?;
                }
                devpp_spec::devpp::generated::DevppCustomizationsDevppMountsItem::Variant1(_) => unimplemented!(),
            }
        }
    }

    writeln!(
        &mut w,
        "/features/{entrypoint}",
        entrypoint = file_name.to_str().unwrap(),
    )?;

    Ok(())
}

pub fn write_feature_dep(
    mut w: impl std::io::Write,
    #[cfg(feature = "devpp")] build_info: &devpp_spec::devc::BuildInfo,
    feature: &devpp_spec::feat::Feature,
    options: &Options,
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

    #[cfg(feature = "devpp")]
    {
        let dir_name = feature.merger.parent().unwrap();
        let file_name = feature.merger.file_name().unwrap();
        writeln!(&mut w, "RUN \\")?;
        writeln!(
            &mut w,
            "--mount=type=bind,source={source},target=/features/ \\",
            source = dir_name.strip_prefix(&build_info.context)?.to_str().unwrap(),
        )?;
        writeln!(&mut w, "/features/{merger}", merger = file_name.to_str().unwrap(),)?;
    }

    Ok(())
}
