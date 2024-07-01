
//! Autogenerated weights for `pallet_asset_manager`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-06-26, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-172-31-15-118`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: 1024

// Executed Command:
// target/release/parachain-template-node
// benchmark
// pallet
// --steps=50
// --repeat=20
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --json-file=benchmarking/results/results-pallet_asset_manager.json
// --pallet=pallet_asset_manager
// --chain=dev
// --output=benchmarking/new-benchmarks/pallet_asset_manager.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_asset_manager`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_asset_manager::WeightInfo for WeightInfo<T> {
	/// Storage: `AssetManager::AssetIdType` (r:1 w:1)
	/// Proof: `AssetManager::AssetIdType` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(174), added: 2649, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Metadata` (r:1 w:1)
	/// Proof: `Assets::Metadata` (`max_values`: None, `max_size`: Some(152), added: 2627, mode: `MaxEncodedLen`)
	/// Storage: `AssetManager::AssetTypeId` (r:0 w:1)
	/// Proof: `AssetManager::AssetTypeId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn register_foreign_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `82`
		//  Estimated: `3639`
		// Minimum execution time: 39_040_000 picoseconds.
		Weight::from_parts(39_649_000, 0)
			.saturating_add(Weight::from_parts(0, 3639))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `AssetManager::AssetTypeId` (r:1 w:0)
	/// Proof: `AssetManager::AssetTypeId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AssetManager::SupportedFeePaymentAssets` (r:1 w:1)
	/// Proof: `AssetManager::SupportedFeePaymentAssets` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `AssetManager::AssetTypeUnitsPerSecond` (r:0 w:1)
	/// Proof: `AssetManager::AssetTypeUnitsPerSecond` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `x` is `[5, 100]`.
	fn set_asset_units_per_second(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `611 + x * (9 ±0)`
		//  Estimated: `4000 + x * (10 ±0)`
		// Minimum execution time: 24_022_000 picoseconds.
		Weight::from_parts(24_974_937, 0)
			.saturating_add(Weight::from_parts(0, 4000))
			// Standard Error: 3_934
			.saturating_add(Weight::from_parts(658_977, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 10).saturating_mul(x.into()))
	}
	/// Storage: `AssetManager::SupportedFeePaymentAssets` (r:1 w:1)
	/// Proof: `AssetManager::SupportedFeePaymentAssets` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `AssetManager::AssetIdType` (r:1 w:1)
	/// Proof: `AssetManager::AssetIdType` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AssetManager::AssetTypeUnitsPerSecond` (r:1 w:2)
	/// Proof: `AssetManager::AssetTypeUnitsPerSecond` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AssetManager::AssetTypeId` (r:0 w:2)
	/// Proof: `AssetManager::AssetTypeId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `x` is `[5, 100]`.
	fn change_existing_asset_type(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `926 + x * (13 ±0)`
		//  Estimated: `4309 + x * (15 ±0)`
		// Minimum execution time: 37_311_000 picoseconds.
		Weight::from_parts(38_598_190, 0)
			.saturating_add(Weight::from_parts(0, 4309))
			// Standard Error: 3_797
			.saturating_add(Weight::from_parts(702_853, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(6))
			.saturating_add(Weight::from_parts(0, 15).saturating_mul(x.into()))
	}
	/// Storage: `AssetManager::SupportedFeePaymentAssets` (r:1 w:1)
	/// Proof: `AssetManager::SupportedFeePaymentAssets` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `AssetManager::AssetTypeUnitsPerSecond` (r:0 w:1)
	/// Proof: `AssetManager::AssetTypeUnitsPerSecond` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `x` is `[5, 100]`.
	fn remove_supported_asset(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `196 + x * (5 ±0)`
		//  Estimated: `1678 + x * (5 ±0)`
		// Minimum execution time: 19_119_000 picoseconds.
		Weight::from_parts(19_212_212, 0)
			.saturating_add(Weight::from_parts(0, 1678))
			// Standard Error: 3_489
			.saturating_add(Weight::from_parts(562_250, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 5).saturating_mul(x.into()))
	}
	/// Storage: `AssetManager::SupportedFeePaymentAssets` (r:1 w:1)
	/// Proof: `AssetManager::SupportedFeePaymentAssets` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `AssetManager::AssetIdType` (r:1 w:1)
	/// Proof: `AssetManager::AssetIdType` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AssetManager::AssetTypeUnitsPerSecond` (r:0 w:1)
	/// Proof: `AssetManager::AssetTypeUnitsPerSecond` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AssetManager::AssetTypeId` (r:0 w:1)
	/// Proof: `AssetManager::AssetTypeId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `x` is `[5, 100]`.
	fn remove_existing_asset_type(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `482 + x * (10 ±0)`
		//  Estimated: `3955 + x * (10 ±0)`
		// Minimum execution time: 26_737_000 picoseconds.
		Weight::from_parts(27_944_215, 0)
			.saturating_add(Weight::from_parts(0, 3955))
			// Standard Error: 4_073
			.saturating_add(Weight::from_parts(587_318, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(Weight::from_parts(0, 10).saturating_mul(x.into()))
	}
}
