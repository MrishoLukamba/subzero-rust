#![cfg_attr(not(feature = "std"), no_std)]


pub use frame_system::pallet::*;

#[cfg(test)]
//mod mock;

#[cfg(test)]
//mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;	

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	
	#[pallet::storage]
	
	
	pub type ItemStore<T: Config> = StorageMap<_,Blake2_128Concat, Vec<u8>, T::AccountId, ValueQuery>;

	
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		
		ItemRemoved(Vec<u8>, T::AccountId ),
		ItemStored(Vec<u8>, T::AccountId),
		
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		ItemExist,
		
		ItemNotFound,
		
		ItemNotUrs,
	}

	
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_item(origin: OriginFor<T>, item: Vec<u8>) -> DispatchResult {
			
			let owner = ensure_signed(origin)?;
			ensure!(ItemStore::<T>::contains_key(&item), Error::<T>::ItemExist);

			
			ItemStore::<T>::insert(&item,&owner);

			
			Self::deposit_event(Event::ItemStored(item, owner));
			
			Ok(())
		}

		
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn delete_item(origin: OriginFor<T>, item: Vec<u8>) -> DispatchResult {
			let owner = ensure_signed(origin)?;
			let existing_owner = ItemStore::<T>::get(&item);
			ensure!(owner != existing_owner, Error::<T>::ItemNotUrs);
			ItemStore::<T>::remove(&item);

			Self::deposit_event(Event::ItemRemoved(item, owner));

			Ok(())


			
		}
	}
}
