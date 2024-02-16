// create the error type that represents all errors possible in our program
#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  Io(#[from] std::io::Error),
  #[error(transparent)]
  Http(#[from] reqwest::Error),
  #[error(transparent)]
  Json(#[from] serde_json::Error),
  #[error(transparent)]
  Selector(#[from] scraper::error::SelectorErrorKind<'static>),
  #[error(transparent)]
  Regex(#[from] regex::Error),
  #[error(transparent)]
  Dateparser(#[from] anyhow::Error),
}

// we must manually implement serde::Serialize
impl serde::Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::ser::Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}
