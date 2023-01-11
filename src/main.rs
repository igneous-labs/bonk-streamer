use log::{debug, info, warn};
use solana_client::rpc_client::RpcClient;

use std::{sync::mpsc, thread, time::Duration};

use bonk_streamer::{config::*, streamer::*, utils::*};

fn main() {
    flexi_logger::Logger::try_with_env_or_str("info")
        .unwrap()
        .start()
        .unwrap();
    let config = Config::load_dot_env();

    info!("started");
    let mut local_last_updated: Option<i64> = None;
    let client = RpcClient::new(&config.rpc_url);
    let (tx, rx) = mpsc::channel::<Message>();
    spawn_streamer(rx, config.clone());

    loop {
        debug!("fetching Board account ...");
        match check_last_updated(&client, &config.board_account, local_last_updated) {
            Ok(Some(last_updated)) => {
                info!(
                    "board was updated at {}, fetching BoardData ...",
                    last_updated
                );
                if save_bonk_board(&client, &config.board_data_account, &config.image_file_path)
                    .is_ok()
                {
                    local_last_updated = Some(last_updated);
                    tx.send(Message::Update).expect("failed to send message");
                } else {
                    warn!(
                        "failed to save image to local file, retrying in {} sec",
                        config.check_interval_sec
                    );
                }
            }
            Ok(None) => debug!("no updates since last check"),
            Err(_) => warn!(
                "failed to retrive Board account, retrying in {} sec",
                config.check_interval_sec
            ),
        }
        thread::sleep(Duration::from_secs(config.check_interval_sec));
    }
}
