#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("sqlx error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("builder paramas error: {0}")]
    BuilderParamsError(String),

    #[error("tonic status: {0}")]
    TonicStatus(#[from] tonic::Status),

    #[error("tonic error: {0}")]
    TonicError(#[from] tonic::transport::Error),
}

impl From<abi::LearnWordQueryBuilderError> for Error {
    fn from(e: abi::LearnWordQueryBuilderError) -> Self {
        Self::BuilderParamsError(e.to_string())
    }
}
