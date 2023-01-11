use borsh::BorshDeserialize;
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;

use crate::Result;

pub trait Fetch
where
    Self: Sized,
{
    fn get_account_data(client: &RpcClient, pubkey: &Pubkey) -> Result<Vec<u8>> {
        Ok(client.get_account(pubkey)?.data)
    }

    fn fetch(client: &RpcClient, pubkey: &Pubkey) -> Result<Self>;
}

#[derive(BorshDeserialize, Debug)]
pub struct Board {
    pub discriminant: [u8; 8],
    pub authority: Pubkey,
    pub last_updated: i64,
    pub board_data_account: Pubkey,
}

impl Fetch for Board {
    fn fetch(client: &RpcClient, pubkey: &Pubkey) -> Result<Self> {
        Self::try_from_slice(&Self::get_account_data(client, pubkey)?).map_err(|e| e.into())
    }
}

// NOTE: data is too big to use BorshDeserialize
#[derive(Debug)]
pub struct BoardData {
    pub discriminant: [u8; 8],
    // NOTE: too big, so try_into causes stake overflow
    //pub data: [u8; BOARD_SIZE * BOARD_SIZE * COLOR_SIZE],
    pub data: Vec<u8>,
}

impl Fetch for BoardData {
    fn fetch(client: &RpcClient, pubkey: &Pubkey) -> Result<Self> {
        let data = Self::get_account_data(client, pubkey)?;

        Ok(Self {
            discriminant: data[..8].try_into()?,
            data: data[8..].to_vec(),
        })
    }
}
