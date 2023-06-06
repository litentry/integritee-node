
//! Autogenerated weights for `pallet_claims`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-11-11, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("integritee-solo-fresh"), DB CACHE: 128

// Executed Command:
// ./integritee-node
// benchmark
// --chain=integritee-solo-fresh
// --steps=50
// --repeat=20
// --pallet=pallet_claims
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=runtime/src/weights/pallet_claims.rs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for pallet_claims.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_claims::WeightInfo for WeightInfo<T> {
	// Storage: Claims Claims (r:1 w:1)
	// Storage: Claims Signing (r:1 w:1)
	// Storage: Claims Total (r:1 w:1)
	// Storage: Claims Vesting (r:1 w:1)
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: System Account (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	fn claim() -> Weight {
		Weight::from_parts(761_780_000, 0u64)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: Claims Total (r:1 w:1)
	// Storage: Claims Vesting (r:0 w:1)
	// Storage: Claims Claims (r:0 w:1)
	// Storage: Claims Signing (r:0 w:1)
	fn mint_claim() -> Weight {
		Weight::from_parts(27_006_000, 0u64)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Claims Claims (r:1 w:1)
	// Storage: Claims Signing (r:1 w:1)
	// Storage: Claims Total (r:1 w:1)
	// Storage: Claims Vesting (r:1 w:1)
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: System Account (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	fn claim_attest() -> Weight {
		Weight::from_parts(775_524_000, 0u64)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: Claims Preclaims (r:1 w:1)
	// Storage: Claims Signing (r:1 w:1)
	// Storage: Claims Claims (r:1 w:1)
	// Storage: Claims Total (r:1 w:1)
	// Storage: Claims Vesting (r:1 w:1)
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: System Account (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	fn attest() -> Weight {
		Weight::from_parts(310_377_000, 0u64)
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	// Storage: Claims Claims (r:1 w:2)
	// Storage: Claims Vesting (r:1 w:2)
	// Storage: Claims Signing (r:1 w:2)
	// Storage: Claims Preclaims (r:1 w:1)
	fn move_claim() -> Weight {
		Weight::from_parts(64_975_000, 0u64)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(7))
	}
}
