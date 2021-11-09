use codec::{Decode, Encode};

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use sp_runtime::{RuntimeDebug};
use sp_std::prelude::*;

pub type KittyIndex = u64;

// pub type KittyIndex<T> = pallet::Config::KittyIndex;
/// 定义rpc返回的消息结构
/// 
/// [
/// 	
/// 	kitty_1:{
/// 		price:
/// 		owner:
/// 	},	
/// 	kitty_2:{
/// 		price:
/// 		owner:
/// 	},
/// 
/// ]
/// 
/// -
pub type GetKittyMarketResult<AccountId, Balance> = 
	Vec<KittyInfoById<AccountId, Balance>>;

#[derive(Eq, PartialEq, Encode, Decode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]	
pub enum MarketKittyqueryError{
	DoNotExistKitty,
}


#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, PartialOrd)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct KittyInfoById<AccountId, Balance>{
	pub kitty_index: KittyIndex,
	pub info: KittyInfo<AccountId, Balance>,
}

// 定义返回消息的基础结构
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, PartialOrd)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct KittyInfo<AccountId,Balance>{
	pub owner: AccountId,
	pub price: Balance,
}




