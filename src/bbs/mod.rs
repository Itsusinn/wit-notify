pub mod data;
use crate::db::DB;
use arcstr::ArcStr;
use color_eyre::eyre::Result;
use mesagisto_client::{
  data::{
    message::{Message, MessageType, Profile},
    Packet,
  },
  server::SERVER,
  EitherExt, ResultExt,
};
use std::{sync::Arc, time::Duration};
use uuid::Uuid;

use self::data::{Dynamic, GetDynamic};

async fn get_latest() -> Result<Vec<Dynamic>> {
  tracing::info!("Getting the latest 10 dynamics");
  let url = "https://apis.windstormlab.com/v2/dynamic/queryList?page=1&limit=10&order_by=time";
  let resp = reqwest::get(url).await?.json::<GetDynamic>().await?;
  resp.parse()
}

pub async fn start(room_id: Arc<Uuid>) {
  let mut interval = tokio::time::interval(Duration::from_secs(60 * 3));
  let server_id: ArcStr = "mesagisto".into();
  tracing::info!("Begaining the main loop");
  loop {
    let room_id = room_id.clone();
    interval.tick().await;
    if let Some(dynamic_list) = get_latest().await.log() {
      let mut packets = Vec::new();
      process(&room_id, dynamic_list, &mut packets).await.log();
      let packets: Vec<Packet> = packets
        .into_iter()
        .flatten()
        .collect();
      if packets.is_empty() {
        tracing::info!("No new dynamics or comments");
      }
      for packet in packets {
        SERVER.send(packet, &server_id).await.log();
        tokio::time::sleep(Duration::from_secs(1)).await;
      }
    } else {
      continue;
    };
  }
}
async fn process(
  room_id: &Arc<Uuid>,
  dynamic_list: Vec<Dynamic>,
  packets: &mut Vec<Option<Packet>>,
) -> Result<()> {
  for dynamic in dynamic_list {
    if DB.record(&dynamic.id) {
      let profile = Profile {
        id: dynamic.id.as_bytes().to_vec(),
        username: Some(format!("{} via HiWIT",dynamic.sender_name)),
        nick: None,
      };
      let message = Message {
        profile,
        from: Vec::new(),
        id: dynamic.id.as_bytes().to_vec(),
        reply: None,
        chain: vec![MessageType::Text {
          content: dynamic.content.to_string(),
        }],
      };
      let packet = Packet::new(room_id.clone(), message.to_left()).log();
      packets.push(packet);
    }
    for comment in dynamic.replies.list {
      if DB.record(&comment.id) {
        let profile = Profile {
          id: comment.id.as_bytes().to_vec(),
          username: Some(format!("{} via Hi-WIT",comment.sender_name)),
          nick: None,
        };
        let message = Message {
          profile,
          from: Vec::new(),
          id: comment.id.as_bytes().to_vec(),
          reply: Some(dynamic.id.as_bytes().to_vec()),
          chain: vec![MessageType::Text {
            content: comment.content.to_string(),
          }],
        };
        let packet = Packet::new(room_id.clone(), message.to_left()).log();
        packets.push(packet);
      }
    }
  }
  Ok(())
}
