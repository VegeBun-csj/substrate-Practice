#![cfg_attr(not(feature = "std"), no_std)]
// use bitflags::bitflags;
use codec::{Decode, Encode};

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use sp_runtime::{RuntimeDebug};
use sp_std::prelude::*;

pub type KittyId = u32;

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, PartialOrd)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct Kitty(pub [u8; 16]);

/// 定义rpc返回的消息结构
/// 
/// [
/// 	
/// 	kitty_1:{
/// 		price:
/// 		owner:
/// 		kitty_dna:
/// 	},	
/// 	kitty_2:{
/// 		price:
/// 		owner:
/// 		kitty_dna:
/// 	},
/// 	kitty_3:{
/// 		price:
/// 		owner:
/// 		kitty_dna:
/// 	},
/// 	kitty_4:{
/// 		price:
/// 		owner:
/// 		kitty_dna:
/// 	},
/// 
/// ]
/// 
/// 
pub type GetKittyMarketResult<AccountId, Balance> = 
	Result<Vec<KittyInfoById<AccountId, Balance>>, MarketKittyqueryError>;

#[derive(Eq, PartialEq, Encode, Decode, RuntimeDebug)]
pub enum MarketKittyqueryError{
	DoesentExistKitty,
}


#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, PartialOrd)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct KittyInfoById<AccountId, Balance>{
	pub kitty_index: KittyId,
	pub info: KittyInfo<AccountId, Balance>,
}

// 定义返回消息的基础结构
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, PartialOrd)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct KittyInfo<AccountId,Balance>{
	pub owner: AccountId,
	pub price: Balance,
	pub kitty_dna: Kitty,
}




