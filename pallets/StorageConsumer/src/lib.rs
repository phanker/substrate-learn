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
    // use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use storage_provider::storage::StorageInterface;

    // Add the runtime configuration trait
    // All types and constants go here.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Value: Member
        + Parameter
        + AtLeast32BitUnsigned
        + Codec
        + From<u32>
        + Into<u32>
        + Copy
        + Debug
        + Default
        + MaxEncodedLen
        + MaybeSerializeDeserialize;
        //定义了ProxyStorage类型，要求其实现trait StorageInterface
        type ProxyStorage: StorageInterface<Value = Self::Value>;
    }

    // Declare the pallet type
    // This is a placeholder to implement traits and methods.
    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    pub struct Pallet<T>(_);



    // 4. Runtime Storage
    // use storageValue store class.
    #[pallet::storage]
    #[pallet::getter(fn my_class)]
    pub type Class<T: Config> = StorageValue<_, u32>;





    // Add runtime events
    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        StoreEvent,
    }

    // Add hooks to define some logic that should be executed
// in a specific context, for example on_initialize.
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // Add functions that are callable from outside the runtime.
    #[pallet::call]
    impl<T: Config> Pallet<T> {

        #[pallet::weight(0)]
        pub fn sava_data(origin: OriginFor<T>, value: T::Value)
                              -> DispatchResultWithPostInfo {
            //确保有管理员权限才能操作
            ensure_signed(origin)?;
            T::ProxyStorage::set_param(value);
            // let paramValue = T::ProxyStorage::get_param();
            // log::info!(target: "other-pallet","current operational value is {:?}",paramValue);
            //使用trait StorageInterface中的函数
            let v:T::Value= T::ProxyStorage::get_param();
            log::info!(target: "storage consumer",
           "Value get from storage is: {:?}", v);



            Ok(().into())
        }
    }
}
