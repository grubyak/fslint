use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Id3Metadata {
  pub title: Option<String>,
  pub artist: Option<String>,
  pub album: Option<String>,
  pub year: Option<u32>,
}
