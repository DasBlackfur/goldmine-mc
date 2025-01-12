use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum GoldmineError {}

pub type Result<T, E = GoldmineError> = std::result::Result<T, E>;
