/// 参考:
/// pallet-contract
/// pallet-transaction-payment
///
/// 引入一些通用的依赖
///
/// 编码相关
use codec::Codec;
use jsonrpc_core::{Error, ErrorCode, Result};
use jsonrpc_derive::rpc;

/// substrate primitives 相关
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_rpc::number::NumberOrHex;
use sp_runtime::{
	generic::BlockId,
	traits::{Block as BlockT},
};

use std::fmt::Display;
/// std相关
use std::{convert::TryInto, sync::Arc};
//
pub use pallet_kitties_rpc_runtime_api::KittiesApi as KittiesRuntimeApi;

// 引入自定义类型
use pallet_kitties::{GetKittyMarketResult, KittyInfoById, KittyInfo};

//
pub struct MarketKittyqueryError(pallet_kitties::MarketKittyqueryError);
impl From<MarketKittyqueryError> for Error {
	fn from(e: MarketKittyqueryError) -> Error {
		use pallet_kitties::MarketKittyqueryError::*;
		match e.0 {
			DoNotExistKitty => Error {
				code: ErrorCode::ServerError(Errors::DecodeError.into()),
				message: "no kitty exist on tha chain ,please create a kitty~~~~".into(),
				data: None,
			},
		}
	}
}

/// 定义Kitties的RPC方法
#[rpc]
pub trait KittiesApi<BlockHash, AccountId, Balance>
{
	#[rpc(name = "kitty_querykittiymarketinfo")]
	fn query_kittiy_market_info(
		&self,
		at: Option<BlockHash>,
	) -> Result<GetKittyMarketResult<AccountId, NumberOrHex>, >;
}

/// 定义一个结构体，用于实现KittiesApi这个trait
pub struct HelloKitty<C, P> {
	client: Arc<C>,
	_marker: std::marker::PhantomData<P>,
}

impl<C, P> HelloKitty<C, P> {
	/// Create new `HelloKitty` with the given reference to the client.
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

/// 定义RPC的错误类型
pub enum Errors {
	/// The transaction was not decodable.
	DecodeError,
	/// The call to runtime failed.
	RuntimeError,
}

// 通过From这个trait为Error实现类型转换(调用的时候使用into)
impl From<Errors> for i64 {
	fn from(e: Errors) -> i64 {
		match e {
			Errors::RuntimeError => 1,
			Errors::DecodeError => 2,
		}
	}
}

// 为HellioKitty实现KittiesApi这个trait
impl<C, Block, AccountId, Balance> KittiesApi<<Block as BlockT>::Hash, AccountId, Balance>
	for HelloKitty<C, Block>
where
	Block: BlockT,
	C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
	C::Api: KittiesRuntimeApi<Block, AccountId, Balance>,
	AccountId: Codec + Display,
	Balance: Codec + Display + Copy + TryInto<NumberOrHex>,
{
	fn query_kittiy_market_info(
		&self,
		at: Option<<Block as BlockT>::Hash>,
	) -> Result<GetKittyMarketResult<AccountId, NumberOrHex>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

		// Balance u128 可序列化处理
		let try_into_rpc_balance = |value: Balance| {
			value.try_into().map_err(|_| Error {
				code: ErrorCode::InvalidParams,
				message: format!("{} doesn't fit in NumberOrHex representation", value),
				data: None,
			})
		};

		// 遍历这个vec，从其中取出所有的Balance进行转换
		Ok(api.query_kittiy_market_info(&at)
			.map(|kitty_market_info|{
				kitty_market_info
					.into_iter()
					.map(|kitty_info_byid| KittyInfoById{
							kitty_index: kitty_info_byid.kitty_index,
							info: KittyInfo{
								owner: kitty_info_byid.info.owner,
								price: try_into_rpc_balance(kitty_info_byid.info.price).unwrap(),
							},
					})
					.collect::<Vec<_>>()
			})
			.map_err(runtime_error_into_rpc_err)?
		)
	}
}

/// Converts a runtime trap into an RPC error.
pub fn runtime_error_into_rpc_err(err: impl std::fmt::Debug) -> Error {
	Error {
		code: ErrorCode::ServerError(Errors::RuntimeError.into()),
		message: "Runtime trapped， unable to get the result".into(),
		data: Some(format!("{:?}", err).into()),
	}
}
