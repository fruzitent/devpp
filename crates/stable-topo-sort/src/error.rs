#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Graph has at least one cycle")]
    CycleDetected,
}

pub type Result<T> = std::result::Result<T, Error>;
