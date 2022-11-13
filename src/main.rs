mod bbs;
mod config;
pub mod db;
mod log;

use std::ops::ControlFlow;

use crate::config::CONFIG;
use crate::db::DB;
use color_eyre::eyre::Result;
use config::Config;
use dashmap::DashMap;
use futures_util::FutureExt;
use mesagisto_client::{data::Packet, server::SERVER, MesagistoConfig, MesagistoConfigBuilder};

#[macro_use]
extern crate automatic_config;
#[macro_use]
extern crate educe;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
  if cfg!(feature = "color") {
    color_eyre::install()?;
  } else {
    color_eyre::config::HookBuilder::new()
      .theme(color_eyre::config::Theme::new())
      .install()?;
  }
  log::init().await?;
  run().await?;
  Ok(())
}
async fn run() -> Result<()> {
  Config::reload().await?;
  let remotes = DashMap::new();
  remotes.insert(
    arcstr::literal!("mesagisto"),
    "wss://center.mesagisto.org".into(),
  );
  DB.init()?;
  MesagistoConfigBuilder::default()
    .name("wit-notify")
    .cipher_key(&CONFIG.cipher_key)
    .proxy(None)
    .remote_address(remotes)
    .skip_verify(false)
    .custom_cert(None)
    .same_side_deliver(false)
    .build()?
    .apply()
    .await?;
  MesagistoConfig::packet_handler(|pkt| async { packet_handler(pkt).await }.boxed());

  let room_id = SERVER.room_id(CONFIG.room_address.to_owned());
  tokio::spawn(async move { bbs::start(room_id).await });
  tokio::signal::ctrl_c().await?;
  Ok(())
}
async fn packet_handler(_: Packet) -> Result<ControlFlow<Packet>> {
  Ok(ControlFlow::Continue(()))
}
