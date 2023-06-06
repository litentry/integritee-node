
//! Autogenerated weights for `pallet_treasury`
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
// --pallet=pallet_treasury
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=runtime/src/weights/pallet_treasury.rs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions for pallet_treasury.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_treasury::WeightInfo for WeightInfo<T> {
	// Storage: Treasury ProposalCount (r:1 w:1)
	// Storage: Treasury Proposals (r:0 w:1)
	fn propose_spend() -> Weight {
		Weight::from_parts(76_542_000, 0u64)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Treasury Proposals (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn reject_proposal() -> Weight {
		Weight::from_parts(77_597_000, 0u64)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Treasury Proposals (r:1 w:0)
	// Storage: Treasury Approvals (r:1 w:1)
	fn approve_proposal(p: u32, ) -> Weight {
		Weight::from_parts(22_285_000, 0u64)
			// Standard Error: 47_000
			.saturating_add(Weight::from_parts(502_000, 0u64)).saturating_mul(p.into())
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Treasury Approvals (r:1 w:1)
	// Storage: Treasury Proposals (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn on_initialize_proposals(p: u32, ) -> Weight {
		Weight::from_parts(82_560_000, 0u64)
			// Standard Error: 648_000
			.saturating_add(Weight::from_parts(103_284_000, 0u64)).saturating_mul(p.into())
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads(3)).saturating_mul(p.into())
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes(3)).saturating_mul(p.into())
	}
	// Storage: Treasury Approvals (r:1 w:1)
	fn remove_approval() -> Weight {
		Weight::from_parts(3_827_000, 0u64)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}

	fn spend() -> Weight {
		Weight::from_parts(22_063_000, 0u64)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
