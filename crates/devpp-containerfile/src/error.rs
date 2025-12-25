#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    DevppSpec(#[from] devpp_spec::error::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Path(#[from] std::path::StripPrefixError),
    #[error("{0}")]
    DockerfileParserRs(String),
    #[error("FROM must be a first instruction")]
    FromNotFound,
    #[error("stage must have at least 1 instruction")]
    InstructionNotFound,
    #[error("target stage does not exist")]
    StageNotFound,
    #[error("target stage must be set")]
    TargetNotFound,
}

pub type Result<T> = std::result::Result<T, Error>;

// TODO: make an issue to implement Error https://github.com/slimreaper35/dockerfile-parser-rs
impl From<dockerfile_parser_rs::ParseError> for Error {
    fn from(err: dockerfile_parser_rs::ParseError) -> Self {
        Error::DockerfileParserRs(err.to_string())
    }
}
