pub mod error;

use std::collections::BTreeMap;
use std::fs::read_to_string;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use containerfile::Containerfile;
use containerfile::instr::Instr;
use containerfile::instr::copy::CopyOptions;
use containerfile::instr::from::FromKind;
use containerfile::instr::run::RunOptions;
use containerfile::instr::run::mount::Mount;
use containerfile::instr::run::mount::bind::BindOptions;
use containerfile::instr::run::mount::cache::CacheOptions;
use containerfile::instr::run::mount::cache::Sharing;
use devpp_spec::devc::Config;
use devpp_spec::devc::DevContainer;
use devpp_spec::devc::IsCompose;
use devpp_spec::devc::IsImage;
use devpp_spec::devc::generated::DockerfileContainer;
use devpp_spec::devpp::Customizations;
use devpp_spec::devpp::generated::DevppCustomizationsDevppMountsItem;
use devpp_spec::devpp::generated::Mount as DevppMount;
use devpp_spec::devpp::generated::MountSharing as DevppMountSharing;
use devpp_spec::devpp::generated::MountType as DevppMountType;
use devpp_spec::feat::Feature;
use devpp_spec::feat::Reference;
use devpp_spec::feat::generated::FeatureOption;
use stable_topo_sort::stable_topo_sort;

use crate::error::Error;
use crate::error::Result;

pub fn build(mut w: impl Write, workspace: &Path, config: Option<&Path>) -> Result<()> {
    let config = Config::find_config(workspace, config)?;
    let config_dir = config.path.parent().unwrap(); // TODO: handle error
    let devc = DevContainer::new(read_to_string(&config.path)?)?;

    let mut features = BTreeMap::new();
    for (id, options) in &devc.common.features {
        let reference = Reference::new(id, &config)?;
        let feature = Feature::new(&reference)?;
        features.insert(
            id,
            Entry {
                cstm: Customizations::new(&feature),
                feat: feature,
                opts: options,
            },
        );
    }

    let (mut nodes, mut edges) = (vec![], vec![]);
    for (id, entry) in &features {
        nodes.push(*id);
        for dep_id in &entry.feat.inner.installs_after {
            edges.push((dep_id, *id));
        }
    }
    let ids = stable_topo_sort(&nodes, &edges)?;

    let mut base_sink = vec![
        Instr::Comment(String::from(
            "@help: https://github.com/moby/buildkit/blob/dockerfile/1.20.0-labs/frontend/dockerfile/docs/reference.md",
        )),
        Instr::Comment(String::from(
            "@help: https://github.com/containers/common/blob/main/docs/Containerfile.5.md",
        )),
        Instr::Empty,
    ];
    let mut feat_sink = vec![];

    let base_stage = String::from("devpp-base");
    let (context, target) = match &devc.is_compose {
        IsCompose::Compose(_compose) => unimplemented!(),
        IsCompose::NonCompose(non_compose) => match &non_compose.is_image {
            IsImage::Dockerfile(dockerfile) => match dockerfile {
                DockerfileContainer::Variant0 { build } => {
                    let path = Path::new(build.context.as_deref().unwrap_or("."));
                    let context = if path.is_relative() {
                        config_dir.join(path).canonicalize()?
                    } else {
                        path.canonicalize()?
                    };
                    base_sink.push(Instr::Comment(String::from("TODO: parse file and patch base stage")));
                    (context, build.target.clone())
                }
                DockerfileContainer::Variant1 { .. } => unimplemented!(),
            },
            IsImage::Image(image) => {
                base_sink.push(Instr::From {
                    kind: FromKind::Image {
                        digest: None,
                        image: image.image.clone(),
                        repo: None,
                        tag: None,
                    },
                    name: Some(base_stage.clone()),
                    platform: None,
                });
                (config_dir.to_path_buf(), None)
            }
        },
    };
    base_sink.push(Instr::Empty);

    for id in &ids {
        let entry = features.get(id).expect("entry exists");

        if entry.is_merge() {
            if !entry.feat.inner.installs_after.is_empty() {
                return Err(Error::NestedMergeNotSupported);
            }
            entry.push_feature(&mut base_sink, &context)?;
            base_sink.push(Instr::Empty);
            continue;
        }

        feat_sink.push(Instr::From {
            kind: FromKind::Stage(base_stage.clone()),
            name: Some(entry.get_feature_id()),
            platform: None,
        });
        feat_sink.push(Instr::Empty);

        for id in &entry.feat.inner.installs_after {
            let entry = features.get(id).expect("entry exists");
            if entry.is_merge() {
                continue;
            }
            entry.push_dependency(&mut feat_sink, &context)?;
            feat_sink.push(Instr::Empty);
        }

        entry.push_feature(&mut feat_sink, &context)?;
        feat_sink.push(Instr::Empty);
    }

    feat_sink.push(Instr::From {
        kind: FromKind::Stage(base_stage.clone()),
        name: target,
        platform: None,
    });
    feat_sink.push(Instr::Empty);

    for id in &ids {
        let entry = features.get(id).expect("entry exists");
        if entry.is_merge() {
            continue;
        }
        entry.push_dependency(&mut feat_sink, &context)?;
        feat_sink.push(Instr::Empty);
    }

    if matches!(feat_sink.last(), Some(Instr::Empty)) {
        feat_sink.pop();
    }

    let mut cf = Containerfile::default();
    cf.append(&mut base_sink);
    cf.append(&mut feat_sink);
    writeln!(w, "{cf}")?;

    Ok(())
}

struct Entry<'a> {
    cstm: Customizations,
    feat: Feature,
    opts: &'a BTreeMap<String, String>,
}

impl<'a> Entry<'a> {
    fn get_devpp_mounts(&self) -> Vec<Mount> {
        let mut mounts = vec![];
        if let Some(devpp) = &self.cstm.0.devpp {
            for mount in &devpp.mounts {
                match mount {
                    DevppCustomizationsDevppMountsItem::Variant0(mount) => {
                        let DevppMount { sharing, target, type_ } = mount;
                        match type_ {
                            DevppMountType::Cache => {
                                mounts.push(Mount::Cache {
                                    destination: PathBuf::from(&target),
                                    options: Some(CacheOptions {
                                        sharing: Some(match sharing {
                                            DevppMountSharing::Locked => Sharing::Locked,
                                            DevppMountSharing::Private => Sharing::Private,
                                            DevppMountSharing::Shared => Sharing::Shared,
                                        }),
                                        ..Default::default()
                                    }),
                                });
                            }
                        }
                    }
                    DevppCustomizationsDevppMountsItem::Variant1(_) => unimplemented!(),
                }
            }
        };
        mounts
    }

    fn get_feature_id(&self) -> String {
        format!("devpp-feature-{}", self.feat.inner.id)
    }

    fn is_merge(&self) -> bool {
        match &self.cstm.0.devpp {
            Some(devpp) => devpp.merge,
            None => false,
        }
    }

    fn push_args(&self, sink: &mut Vec<Instr>) {
        let mut args = vec![];

        for (key, option) in &self.feat.inner.options {
            let (default, description) = match option {
                FeatureOption::Variant0 { .. } => unimplemented!(),
                FeatureOption::Variant1 {
                    default, description, ..
                } => (default, description),
                FeatureOption::Variant2 {
                    default, description, ..
                } => (default, description),
            };

            if let Some(description) = description {
                sink.push(Instr::Comment(format!("@help({key}): {description}")));
            };

            args.push((key.to_uppercase(), Some(self.opts.get(key).unwrap_or(default).clone())))
        }

        if !args.is_empty() {
            sink.push(Instr::Arg(args))
        }
    }

    fn push_copy(&self, sink: &mut Vec<Instr>) {
        // TODO: unhardcode install path
        let path = PathBuf::from(format!("/opt/{}", self.feat.inner.id));
        sink.push(Instr::Copy {
            destination: path.clone(),
            options: Some(CopyOptions {
                from: Some(FromKind::Stage(self.get_feature_id())),
                link: true,
                ..Default::default()
            }),
            source: vec![path.clone()],
        });
    }

    fn push_dependency(&self, sink: &mut Vec<Instr>, context: &Path) -> Result<()> {
        sink.push(Instr::Comment(String::from(
            "@see: [acquire.sh](https://github.com/devcontainers/spec/issues/21)",
        )));

        self.push_envs(sink);
        self.push_copy(sink);

        if let Some(merger) = &self.feat.merger {
            self.push_run(sink, context, merger)?;
        }

        Ok(())
    }

    fn push_envs(&self, sink: &mut Vec<Instr>) {
        let mut envs = vec![];

        for (key, value) in &self.feat.inner.container_env {
            envs.push((key.clone(), value.clone()));
        }

        if !envs.is_empty() {
            sink.push(Instr::Env(envs));
        }
    }

    fn push_feature(&self, sink: &mut Vec<Instr>, context: &Path) -> Result<()> {
        self.push_args(sink);
        self.push_envs(sink);
        self.push_run(sink, context, &self.feat.entrypoint)?;
        Ok(())
    }

    fn push_run(&self, sink: &mut Vec<Instr>, context: &Path, path: &Path) -> Result<()> {
        // TODO: handle errors
        let dir_name = path.parent().unwrap();
        let file_name = path.file_name().unwrap();

        let mut mounts = vec![Mount::Bind {
            destination: PathBuf::from("/features/"),
            options: Some(BindOptions {
                source: Some(dir_name.strip_prefix(context)?.to_path_buf()),
                ..Default::default()
            }),
        }];
        mounts.extend(self.get_devpp_mounts());

        sink.push(Instr::Run {
            command: vec![
                String::from("sh"), // TODO: chmod 0755
                format!("/features/{}", file_name.to_str().expect("UTF-8")),
            ],
            options: Some(RunOptions {
                mount: Some(mounts),
                ..Default::default()
            }),
        });
        Ok(())
    }
}
