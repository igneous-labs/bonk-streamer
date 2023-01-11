use image;
use solana_client::rpc_client::RpcClient;

use std::path::Path;

use crate::{
    accounts::{Board, BoardData, Fetch},
    consts::BOARD_SIZE,
    Result,
};

pub fn save_bonk_board(
    client: &RpcClient,
    board_data_account: &str,
    image_file_path: &str,
) -> Result<()> {
    image::save_buffer(
        Path::new(image_file_path),
        &BoardData::fetch(&client, board_data_account)?.data,
        BOARD_SIZE as u32,
        BOARD_SIZE as u32,
        image::ColorType::Rgb8,
    )?;

    Ok(())
}

pub fn check_last_updated(
    client: &RpcClient,
    board_account: &str,
    local_last_updated: Option<i64>,
) -> Result<Option<i64>> {
    let on_chain_last_updated = Board::fetch(&client, board_account)?.last_updated;

    Ok(
        if local_last_updated.is_none() || on_chain_last_updated > local_last_updated.unwrap() {
            Some(on_chain_last_updated)
        } else {
            None
        },
    )
}
