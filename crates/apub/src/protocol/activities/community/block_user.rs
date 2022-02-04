use crate::{
  objects::{community::ApubCommunity, person::ApubPerson},
  protocol::Unparsed,
};
use activitystreams_kinds::activity::BlockType;
use chrono::{DateTime, FixedOffset};
use lemmy_apub_lib::object_id::ObjectId;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockUserFromCommunity {
  pub(crate) actor: ObjectId<ApubPerson>,
  #[serde(deserialize_with = "crate::deserialize_one_or_many")]
  pub(crate) to: Vec<Url>,
  pub(crate) object: ObjectId<ApubPerson>,
  #[serde(deserialize_with = "crate::deserialize_one_or_many")]
  pub(crate) cc: Vec<Url>,
  pub(crate) target: ObjectId<ApubCommunity>,
  #[serde(rename = "type")]
  pub(crate) kind: BlockType,
  pub(crate) id: Url,
  #[serde(flatten)]
  pub(crate) unparsed: Unparsed,
  pub(crate) expires: Option<DateTime<FixedOffset>>,
}
