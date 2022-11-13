use arcstr::ArcStr;
use color_eyre::eyre::{Error, Result};

#[config_derive]
#[derive(AutomaticConfig)]
#[location = "config/wit-notify.yml"]
pub struct Config {
  #[educe(Default = false)]
  pub enable: bool,
  #[educe(Default = "")]
  pub locale: ArcStr,
  pub room_address: ArcStr,
  pub cipher_key: ArcStr,
}
