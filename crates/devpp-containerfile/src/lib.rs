pub mod error;

use std::collections::BTreeMap;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fs::read_to_string;
use std::str::FromStr;

use devpp_spec::devc::BuildInfo;
use devpp_spec::devc::generated::Mount as DevcMount;
use devpp_spec::devc::generated::MountType as DevcMountType;
#[cfg(feature = "devpp")]
use devpp_spec::devpp::Customizations;
#[cfg(feature = "devpp")]
use devpp_spec::devpp::generated::DevppCustomizationsDevppMountsItem;
use devpp_spec::devpp::generated::Mount as DevppMount;
use devpp_spec::devpp::generated::MountType as DevppMountType;
use devpp_spec::feat::Feature;
use devpp_spec::feat::Features;
use devpp_spec::feat::Options;
use devpp_spec::feat::generated::FeatureOption;
use dockerfile_parser_rs::Dockerfile;
use dockerfile_parser_rs::Instruction;

use crate::error::Error;
use crate::error::Result;

pub fn find_stage<'a>(ast: &'a mut Dockerfile, target: &String) -> Result<&'a mut [Instruction]> {
    let mut start = None;
    let mut end = ast.instructions.len();
    for (i, instr) in ast.instructions.iter().enumerate() {
        if let Instruction::From { alias, .. } = instr {
            if alias.as_ref() == Some(target) {
                start = Some(i);
                continue;
            }
            if start.is_some() {
                end = i;
                break;
            }
        }
    }
    start.ok_or(Error::StageNotFound).map(|i| &mut ast.instructions[i..end])
}

pub struct Containerfile<'a> {
    ast: Dockerfile,
    build_info: &'a BuildInfo,
}

impl<'a> Containerfile<'a> {
    pub fn new(build_info: &'a BuildInfo) -> Result<Self> {
        Ok(Self {
            ast: Dockerfile::from_str(&read_to_string(&build_info.containerfile)?)?,
            build_info,
        })
    }

    pub fn apply_feature(&mut self, feature: &Feature, options: &Options, features: &Features) -> Result<()> {
        if !matches!(self.ast.instructions.last(), None | Some(Instruction::Empty {})) {
            self.ast.instructions.push(Instruction::Empty {});
        }

        self.apply_feature_begin(feature, options)?;
        self.ast.instructions.push(Instruction::Empty {});

        for id in &feature.inner.installs_after {
            let (feature, options) = features.get(id).unwrap();
            self.apply_feature_dep(feature, options)?;
            self.ast.instructions.push(Instruction::Empty {});
        }

        self.apply_feature_end(
            feature,
            options,
            #[cfg(feature = "devpp")]
            &Customizations::new(feature),
        )?;

        Ok(())
    }

    fn apply_feature_begin(&mut self, feature: &Feature, _options: &Options) -> Result<()> {
        self.ast.instructions.push(Instruction::From {
            alias: Some(Self::feature_target(&feature.inner.id)),
            image: Self::BASE_TARGET.to_string(),
            platform: None,
        });
        Ok(())
    }

    fn apply_feature_dep(&mut self, feature: &Feature, options: &Options) -> Result<()> {
        let comment = "# @see: [acquire.sh](https://github.com/devcontainers/spec/issues/21)";
        self.ast.instructions.push(Instruction::Comment(comment.to_string()));

        self.push_args(feature, options);
        self.push_envs(feature);

        let path = format!("/opt/{id}/", id = feature.inner.id);
        self.ast.instructions.push(Instruction::Copy {
            chmod: None,
            chown: None,
            destination: path.clone(),
            from: Some(Self::feature_target(&feature.inner.id)),
            link: None,
            sources: vec![path.clone()],
        });

        #[cfg(feature = "devpp")]
        if let Some(merger) = &feature.merger {
            let dir_name = merger.parent().unwrap();
            let file_name = merger.file_name().unwrap();
            self.ast.instructions.push(Instruction::Run {
                command: vec![String::from("<<EOF")],
                heredoc: Some(vec![
                    format!("/features/{merger}", merger = file_name.to_str().unwrap()),
                    String::from("EOF"),
                ]),
                mount: Some(ToString::to_string(&Mount::Devc(DevcMount {
                    source: Some(ToString::to_string(
                        &dir_name.strip_prefix(&self.build_info.context)?.to_str().unwrap(),
                    )),
                    target: String::from("/features/"),
                    type_: DevcMountType::Bind,
                }))),
                network: None,
                security: None,
            });
        }

        Ok(())
    }

    fn apply_feature_end(
        &mut self,
        feature: &Feature,
        options: &Options,
        #[cfg(feature = "devpp")] customizations: &Customizations,
    ) -> Result<()> {
        self.push_args(feature, options);
        if !feature.inner.options.is_empty() {
            self.ast.instructions.push(Instruction::Empty {})
        }

        self.push_envs(feature);
        if !feature.inner.container_env.is_empty() {
            self.ast.instructions.push(Instruction::Empty {})
        }

        let dir_name = feature.entrypoint.parent().unwrap();
        let file_name = feature.entrypoint.file_name().unwrap();

        let mut mounts = vec![Mount::Devc(DevcMount {
            source: Some(
                dir_name
                    .strip_prefix(&self.build_info.context)?
                    .to_str()
                    .unwrap()
                    .to_string(),
            ),
            target: String::from("/features/"),
            type_: DevcMountType::Bind,
        })];

        #[cfg(feature = "devpp")]
        if let Some(devpp) = &customizations.0.devpp {
            for mount in &devpp.mounts {
                match mount {
                    DevppCustomizationsDevppMountsItem::Variant0(mount) => mounts.push(Mount::Devpp(mount.clone())),
                    DevppCustomizationsDevppMountsItem::Variant1(_) => unimplemented!(),
                }
            }
        }

        self.ast.instructions.push(Instruction::Run {
            command: vec![String::from("<<EOF")],
            heredoc: Some(vec![
                format!("/features/{entrypoint}", entrypoint = file_name.to_str().unwrap()),
                String::from("EOF"),
            ]),
            mount: Some(
                mounts
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" --mount="),
            ),
            network: None,
            security: None,
        });

        Ok(())
    }

    const BASE_TARGET: &'static str = "devpp-base";

    fn feature_target(id: &str) -> String {
        format!("devpp-feature-{}", id)
    }

    pub fn patch_base(&mut self) -> Result<()> {
        let target = self.build_info.target.as_ref().ok_or(Error::TargetNotFound)?;
        let stage = find_stage(&mut self.ast, target)?;
        match stage.first_mut().ok_or(Error::InstructionNotFound)? {
            Instruction::From { alias, .. } => {
                *alias = Some(Self::BASE_TARGET.to_string());
                Ok(())
            }
            _ => Err(Error::FromNotFound),
        }
    }

    fn push_args(&mut self, feature: &Feature, options: &Options) {
        let args = BTreeMap::from_iter(feature.inner.options.iter().map(|(key, option)| {
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
                let comment = format!("# @see: {description}");
                self.ast.instructions.push(Instruction::Comment(comment));
            };
            (key.to_uppercase(), Some(options.get(key).unwrap_or(default).clone()))
        }));
        if !args.is_empty() {
            self.ast.instructions.push(Instruction::Arg(args));
        }
    }

    fn push_envs(&mut self, feature: &Feature) {
        let envs = BTreeMap::from_iter(
            feature
                .inner
                .container_env
                .iter()
                .map(|(key, value)| (key.clone(), value.clone())),
        );
        if !envs.is_empty() {
            self.ast.instructions.push(Instruction::Env(envs));
        }
    }
}

impl<'a> Display for Containerfile<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for instr in &self.ast.instructions {
            writeln!(f, "{}", instr)?;
        }
        Ok(())
    }
}

// TODO: handle invariants
pub enum Mount {
    Devc(DevcMount),
    Devpp(DevppMount),
}

impl Display for Mount {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Mount::Devc(mount) => {
                let DevcMount { source, target, type_ } = mount;
                match type_ {
                    DevcMountType::Bind => {
                        let source = source.as_ref().unwrap();
                        write!(f, "type=bind,source={source},target={target}")?
                    }
                    DevcMountType::Volume => write!(f, "type=volume,target={target}")?,
                }
            }
            Mount::Devpp(mount) => {
                let DevppMount { sharing, target, type_ } = mount;
                match type_ {
                    DevppMountType::Cache => write!(f, "type=cache,target={target},sharing={sharing}")?,
                }
            }
        }
        Ok(())
    }
}
