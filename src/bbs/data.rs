use arcstr::ArcStr;
use color_eyre::eyre::{self, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetDynamic {
  pub code: i64,
  pub msg: ArcStr,
  pub data: Option<DynamicData>,
}
impl GetDynamic {
  pub fn parse(self) -> Result<Vec<Dynamic>> {
    if let Some(data) = self.data {
      Ok(data.list)
    } else {
      Err(eyre::eyre!(
        "Empty dynamic data,code {},msg {}",
        self.code,
        self.msg
      ))
    }
  }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct DynamicData {
  pub is_total: bool,
  pub list: Vec<Dynamic>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Dynamic {
  #[serde(rename = "dynamic_content")]
  pub content: ArcStr,
  #[serde(rename = "dynamic_id")]
  pub id: ArcStr,
  #[serde(rename = "from_user_id")]
  pub user_id: ArcStr,
  #[serde(rename = "nickname")]
  pub sender_name: ArcStr,
  #[serde(rename = "image_list")]
  pub images: Vec<DynamicImage>,
  pub replies: CommentData,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CommentData {
  pub list: Vec<DynamicComment>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DynamicComment {
  #[serde(rename = "comment_content")]
  pub content: ArcStr,
  #[serde(rename = "comment_id")]
  pub id: ArcStr,
  #[serde(rename = "from_user_id")]
  pub user_id: ArcStr,
  #[serde(rename = "nickname")]
  pub sender_name: ArcStr,
  #[serde(rename = "to_dynamic_id")]
  pub reply_to: ArcStr,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct DynamicImage {
  #[serde(rename = "original_image")]
  pub data:DynamicOriginalImage
}
#[derive(Serialize, Deserialize, Debug)]
pub struct DynamicOriginalImage {
  pub id: ArcStr,
  pub url: ArcStr
}
