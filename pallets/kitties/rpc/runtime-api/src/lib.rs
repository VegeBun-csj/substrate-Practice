#![cfg_attr(not(feature = "std"), no_std)]


use codec::Codec;

// use 自定义类型，主要是返回值的类型
pub use pallet_kitties::GetKittyMarketResult;

sp_api::decl_runtime_apis!{
    pub trait KittiesApi<AccountId, Balance> where
        AccountId: Codec,
        Balance: Codec,
    {
        fn query_kittiy_market_info() -> GetKittyMarketResult<AccountId, Balance>;
    }
}