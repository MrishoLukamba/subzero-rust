#![cfg_attr(not(feature = "std"), no_std)]




#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		dispatch::DispatchResult,
		pallet_prelude::*,
		storage::bounded_vec::BoundedVec,

	};
	use frame_system::pallet_prelude::*;


	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(_);


	#[pallet::storage]
	#[pallet::getter(fn get_value)]
	pub type Number<T> = StorageValue<_, u32,ValueQuery>;


	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		IncrementedNumber(u32,T::AccountId),
		DecrementedNumber(u32,T::AccountId),
		NumberStored(u32, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		StorageUnderflow,
		StorageOverflow,
	}


	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(1000)]
		pub fn get_number(origin:OriginFor<T>) -> DispatchResult{
			let owner = ensure_signed(origin)?;

			let number = Self::get_value();

			Self::deposit_event(Event::NumberStored(number, owner));
			Ok(())
		}

		#[pallet::weight(1000)]
		pub fn increment(origin:OriginFor<T>, number:u32) -> DispatchResult{
			let owner = ensure_signed(origin)?;

			let count = Self::get_value().checked_add(number).ok_or(<Error<T>>::StorageOverflow)?;

			<Number<T>>::put(count);
			Self::deposit_event(Event::IncrementedNumber(count,owner));
			Ok(())

		}

		#[pallet::weight(1000)]
		pub fn decrement(origin:OriginFor<T>, number:u32) -> DispatchResult{
			let owner = ensure_signed(origin)?;

			let count = Self::get_value().checked_sub(number).ok_or(<Error<T>>::StorageUnderflow)?;

			<Number<T>>::put(count);
			Self::deposit_event(Event::DecrementedNumber(count,owner));

			Ok(())

		}
	}
  }


