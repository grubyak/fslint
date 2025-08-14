use crate::fs::{
  metadata::{exif, MetadataProvider, Value},
  node::Node,
  rule::RuleReport,
};

pub fn get_exif(node: &Node) -> Result<&exif::provider::Metadata, RuleReport> {
  let provider = exif::provider::Provider;

  match node.metadata.get(provider.key()) {
    Some(Value::Exif(metadata)) => Ok(metadata),

    Some(_) => Err(RuleReport {
      messages: vec!["exif metadata was not populated properly".into()],
      level: log::Level::Error,
    }),

    None => Err(RuleReport {
      messages: vec!["exif metadata is missing".into()],
      level: log::Level::Error,
    }),
  }
}
