#![cfg_attr(not(feature = "std"), no_std)]

// Add required imports and dependencies
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use codec::Codec;
    use frame_support::{
        pallet_prelude::*,
        sp_runtime::traits::AtLeast32BitUnsigned,
        sp_std::fmt::Debug,
    };
    use frame_system::pallet_prelude::*;
    use frame_system::offchain::*;

    // Declare the pallet type
    // This is a placeholder to implement traits and methods.
    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    pub struct Pallet<T>(_);

    // Add the runtime configuration trait
    // All types and constants go here.
    #[pallet::config]
    // pub trait Config:frame_system::Config + CreateSignedTransaction<Call<Self>>{
    pub trait Config:frame_system::Config  {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        // type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
    }


    // Add runtime events
    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {}

    // Add hooks to define some logic that should be executed
// in a specific context, for example on_initialize.
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        /// Offchain worker entry point.
        ///
        /// By implementing `fn offchain_worker` you declare a new offchain worker.
        /// This function will be called when the node is fully synced and a new best block is
        /// successfully imported.
        /// Note that it's not guaranteed for offchain workers to run on EVERY block, there might
        /// be cases where some blocks are skipped, or for some the worker runs twice (re-orgs),
        /// so the code should be able to handle that.
        fn offchain_worker(block_number: BlockNumberFor<T>) {
            log::info!("Hello from pallet-ocw.");
            // The entry point of your code called by offchain worker
        }
    }

    // Add functions that are callable from outside the runtime.
    #[pallet::call]
    impl<T: Config> Pallet<T> {}
}


