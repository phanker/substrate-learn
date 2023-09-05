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

    // Declare the pallet type
    // This is a placeholder to implement traits and methods.
    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    pub struct Pallet<T>(_);

    // Add the runtime configuration trait
    // All types and constants go here.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        //（1）声明了StudentNumberType和StudentNameType
        //声明StudentNumber类型
        type StudentNumberType: Member
        + Parameter
        + AtLeast32BitUnsigned
        + Codec
        + Copy
        + Debug
        + Default
        + MaxEncodedLen
        + MaybeSerializeDeserialize;

        //声明StudentName类型
        type StudentNameType: Parameter
        + Member
        + AtLeast32BitUnsigned
        + Codec
        + Default
        + From<u128>
        + Into<u128>
        + Copy
        + MaxEncodedLen
        + MaybeSerializeDeserialize
        + Debug;
    }


    // 4. Runtime Storage
    // use storageValue store class.
    #[pallet::storage]
    #[pallet::getter(fn my_class)]
    pub type Class<T: Config> = StorageValue<_, u32>;

    // use storageMap store (student number -> student name).
    #[pallet::storage]
    #[pallet::getter(fn students_info)]
    pub type StudentsInfo<T: Config> =
    StorageMap<_, Blake2_128Concat, T::StudentNumberType, T::StudentNameType, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn dorm_info)]
    pub type DormInfo<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        u32, //dorm number
        Blake2_128Concat,
        u32, //bed number
        u32, // student number
        ValueQuery,
    >;

    // Add runtime events
    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        SetClass(u32),
        SetStudentInfo(T::StudentNumberType, T::StudentNameType),
        SetDormInfo(u32, u32, u32),
    }

    // Add hooks to define some logic that should be executed
// in a specific context, for example on_initialize.
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // Add functions that are callable from outside the runtime.
    #[pallet::call]
    impl<T: Config> Pallet<T> {

        #[pallet::weight(0)]
        pub fn set_class_info(origin: OriginFor<T>, class: u32)
                              -> DispatchResultWithPostInfo {
            // determine if the root user?
            ensure_root(origin)?;

            Class::<T>::put(class);
            Self::deposit_event(Event::SetClass(class));

            Ok(().into())
        }

        #[pallet::weight(0)]
        pub fn set_student_info(
            origin: OriginFor<T>,
            student_number: T::StudentNumberType,
            student_name: T::StudentNameType,
        ) -> DispatchResultWithPostInfo {
            ensure_signed(origin)?;

            StudentsInfo::<T>::insert(&student_number, &student_name);
            Self::deposit_event(Event::SetStudentInfo(
                student_number,
                student_name));

            Ok(().into())
        }

        #[pallet::weight(0)]
        pub fn set_dorm_info(
            origin: OriginFor<T>,
            dorm_number: u32,
            bed_number: u32,
            student_number: u32,
        ) -> DispatchResultWithPostInfo {
            ensure_signed(origin)?;

            DormInfo::<T>::insert(&dorm_number,
                                  &bed_number,
                                  &student_number);
            Self::deposit_event(Event::SetDormInfo(
                dorm_number,
                bed_number,
                student_number));

            Ok(().into())
        }
    }
}
