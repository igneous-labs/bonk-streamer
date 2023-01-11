use dotenv::dotenv;
use solana_program::pubkey::Pubkey;

use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub rpc_url: String,
    pub check_interval_sec: u64,
    pub board_data_account: Pubkey,
    pub board_account: Pubkey,
    pub image_file_path: String,
    pub rtmp_endpoint: String,
    pub frame_rate: String,
    pub stream_width: u16,
    pub stream_height: u16,
    pub bit_rate: u16,
}

impl Config {
    pub fn load_dot_env() -> Self {
        dotenv().ok();
        let rpc_url = env::var("RPC_URL").expect("RPC_URL must be set");
        let check_interval_sec: u64 = env::var("CHECK_INTERVAL_SEC")
            .expect("CHECK_INTERVAL_SEC must be set")
            .parse()
            .expect("failed to parse CHECK_INTERVAL_SEC");
        let board_data_account: Pubkey = env::var("BOARD_DATA_ACCOUNT")
            .expect("BOARD_DATA_ACCOUNT must be set")
            .as_str()
            .try_into()
            .expect("failed to parse BOARD_DATA_ACCOUNT");
        let board_account = env::var("BOARD_ACCOUNT")
            .expect("BOARD_ACCOUNT must be set")
            .as_str()
            .try_into()
            .expect("failed to parse BOARD_ACCOUNT");
        let image_file_path = env::var("IMAGE_FILE_PATH").expect("IMAGE_FILE_PATH must be set");
        let rtmp_endpoint = env::var("RTMP_ENDPOINT").expect("RTMP_ENDPOINT must be set");
        let frame_rate = env::var("FRAME_RATE").expect("FRAME_RATE must be set");
        let stream_width: u16 = env::var("STREAM_WIDTH")
            .expect("STREAM_WIDTH must be set")
            .parse()
            .expect("failed to parse STREAM_WIDTH");
        let stream_height = env::var("STREAM_HEIGHT")
            .expect("STREAM_HEIGHT must be set")
            .parse()
            .expect("failed to parse STREAM_HEIGHT");
        let bit_rate = env::var("BIT_RATE")
            .expect("BIT_RATE must be set")
            .parse()
            .expect("failed to parse BIT_RATE");

        Self {
            rpc_url,
            check_interval_sec,
            board_data_account,
            board_account,
            image_file_path,
            rtmp_endpoint,
            frame_rate,
            stream_width,
            stream_height,
            bit_rate,
        }
    }
}
