extern crate dotenv;

use dotenv::dotenv;
use flexi_logger;
use log::{debug, info, warn};
use solana_client::rpc_client::RpcClient;

use std::{env, sync::mpsc, thread, time::Duration};

mod accounts;
mod consts;
mod streamer;
mod utils;

use streamer::*;
use utils::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    flexi_logger::Logger::try_with_env_or_str("info")
        .unwrap()
        .start()
        .unwrap();
    dotenv().ok();

    let rpc_url = env::var("RPC_URL").expect("RPC_URL must be set");
    let check_interval_sec: u64 = env::var("CHECK_INTERVAL_SEC")
        .expect("CHECK_INTERVAL_SEC must be set")
        .parse()
        .expect("failed to parse CHECK_INTERVAL_SEC");
    let board_data_account =
        env::var("BOARD_DATA_ACCOUNT").expect("BOARD_DATA_ACCOUNT must be set");
    let board_account = env::var("BOARD_ACCOUNT").expect("BOARD_ACCOUNT must be set");

    let image_file_path = env::var("IMAGE_FILE_PATH").expect("IMAGE_FILE_PATH must be set");
    let rtmp_endpoint = env::var("RTMP_ENDPOINT").expect("RTMP_ENDPOINT must be set");
    let frame_rate = env::var("FRAME_RATE").expect("FRAME_RATE must be set");
    let stream_width = env::var("STREAM_WIDTH").expect("STREAM_WIDTH must be set");
    let stream_height = env::var("STREAM_HEIGHT").expect("STREAM_HEIGHT must be set");
    let bit_rate = env::var("BIT_RATE").expect("BIT_RATE must be set");

    info!("started");
    let mut local_last_updated: Option<i64> = None;
    let client = RpcClient::new(&rpc_url);
    let (tx, rx) = mpsc::channel::<Message>();
    spawn_streamer(
        rx,
        &image_file_path,
        &rtmp_endpoint,
        &frame_rate,
        &stream_width,
        &stream_height,
        &bit_rate,
    );

    loop {
        debug!("fetching Board account ...");
        match check_last_updated(&client, &board_account, local_last_updated) {
            Ok(Some(last_updated)) => {
                info!(
                    "board was updated at {}, fetching BoardData ...",
                    last_updated
                );
                save_bonk_board(&client, &board_data_account, &image_file_path)
                    .expect("failed to save the image");
                local_last_updated = Some(last_updated);
                tx.send(Message::Update).expect("failed to send message");
            }
            Ok(None) => debug!("no updates since last check"),
            Err(_) => warn!(
                "failed to retrive Board account, retrying in {} sec",
                check_interval_sec
            ),
        }
        thread::sleep(Duration::from_secs(check_interval_sec));
    }
}
