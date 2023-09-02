#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	// Declare the pallet type
	// This is a placeholder to implement traits and methods.
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}


	#[pallet::storage]
	pub type Proofs<T: Config> =
	StorageMap<_, Blake2_128Concat, u32, u128>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ClaimCreated(u32, u128),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {

	}

	#[pallet::call]
	impl<T:Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn create_claim(origin: OriginFor<T>,
							id: u32,
							claim: u128) -> DispatchResultWithPostInfo {
			ensure_signed(origin)?;
			Proofs::<T>::insert(
				&id,
				&claim,
			);

			Self::deposit_event(Event::ClaimCreated(id, claim));
			Ok(().into())
		}
	}
}
