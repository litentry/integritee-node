#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		CreditScoreReport(T::AccountId, u32),
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn report_credit_score(
			origin: OriginFor<T>,
			account: T::AccountId,
			score: u32,
		) -> DispatchResult {
			// TODO add the is_registered_enclave as a trait in teerex pallet.
			// then call it to verify the credit score report from verified TEE device.
			let _who = ensure_signed(origin)?;

			Self::deposit_event(Event::CreditScoreReport(account, score));
			Ok(())
		}
	}
}
