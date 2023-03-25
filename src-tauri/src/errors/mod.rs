use crate::model::data_scaffold::{
    learn_word::LearnWordQueryBuilderError, word_list::WordListQueryBuilderError,
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("sqlx error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("builder paramas error: {0}")]
    BuilderParamsError(String),

    #[error("supermemo2 error: {0}")]
    Supermemo2Error(#[from] supermemo2::Error),

    #[error("chrono parse error: {0}")]
    ChronoParaseError(#[from] chrono::ParseError),
}

impl From<LearnWordQueryBuilderError> for Error {
    fn from(e: LearnWordQueryBuilderError) -> Self {
        Error::BuilderParamsError(e.to_string())
    }
}

impl From<WordListQueryBuilderError> for Error {
    fn from(e: WordListQueryBuilderError) -> Self {
        Error::BuilderParamsError(e.to_string())
    }
}
