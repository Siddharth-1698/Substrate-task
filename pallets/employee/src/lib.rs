#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    //use sp_std::vec::Vec;
    use sp_std::prelude::*;

    type BVec = BoundedVec<u8, ConstU32<100>>;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]

    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }
    // Pallets use events to inform users when important changes are made.
    // Event documentation should end with an array that provides descriptive names for parameters.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event emitted when a claim has been created.
        ClaimCreated { who: T::AccountId, claim: T::Hash },
        /// Event emitted when a claim is revoked by the owner.
        ClaimRevoked { who: T::AccountId, claim: T::Hash },
    }
    #[pallet::error]
    pub enum Error<T> {
        /// Wrong input is given
        WrongInput,
        /// Already created info provided
        AlreadyExists,
        /// No Value Provided.
        NullValue,
    }
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct Employee {
        id: u32,
        name: BVec,
        company_name: BVec,
        dob: BVec,
    }

    #[pallet::storage]
    pub type Employees<T: Config> = StorageMap<_, Blake2_128Concat, u32, Employee, OptionQuery>;
    // Dispatchable functions allow users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(50_000_000)]
        pub fn create_employee(
            origin: OriginFor<T>,
            id: u32,
            name: Vec<u8>,
            company_name: Vec<u8>,
            dob: Vec<u8>,
        ) -> DispatchResult {
            //use std::convert::TryInto;
            ensure_root(origin)?;

            let bounded_name: BVec = name.try_into().map_err(|_| Error::<T>::NullValue)?;
            let bounded_company_name: BVec =
                company_name.try_into().map_err(|_| Error::<T>::NullValue)?;
            let bounded_dob: BVec = dob.try_into().map_err(|_| Error::<T>::NullValue)?;
            let emp = Employee {
                id: id,
                name: bounded_name,
                company_name: bounded_company_name,
                dob: bounded_dob,
            };

            Employees::<T>::insert(id, emp);

            Ok(())
        }
        #[pallet::call_index(1)]
        #[pallet::weight(50_000_000)]
        pub fn update_employee_info(
            origin: OriginFor<T>,
            id: u32,
            name: Vec<u8>,
            company_name: Vec<u8>,
            dob: Vec<u8>,
        ) -> DispatchResult {

            ensure_root(origin)?;
            let bounded_name: BVec = name.try_into().map_err(|_| Error::<T>::NullValue)?;
            let bounded_company_name: BVec =
                company_name.try_into().map_err(|_| Error::<T>::NullValue)?;
            let bounded_dob: BVec = dob.try_into().map_err(|_| Error::<T>::NullValue)?;
            let emp = Employee {
                id: id,
                name: bounded_name,
                company_name: bounded_company_name,
                dob: bounded_dob,
            };
            Employees::<T>::insert(id, emp);
            Ok(())

            // match Employees::<T>::get(id) {
            //     Some(emp) => {

            //     }
            //     _ => return Err((Error::<T>::NullValue))
            // }
        }
        #[pallet::call_index(2)]
        #[pallet::weight(50_000_000)]
        pub fn delete_employee(
            origin: OriginFor<T>,
            id: u32,
        ) -> DispatchResult {
            
            ensure_root(origin)?;
          
            Employees::<T>::remove(id);
            Ok(())

            // match Employees::<T>::get(id) {
            //     Some(emp) => {

            //     }
            //     _ => return Err((Error::<T>::NullValue))
            // }
        }
    }
}
