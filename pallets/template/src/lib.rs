#![cfg_attr(not(feature = "std"), no_std)]


pub use pallet::*;

/// 1.  Have a sudo account
///
/// 2. Have a storage with bounded vector
///
/// 3. Configure maximum no of members
///
/// 4. Only sudo account can add and remove members
///


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

		/// Maximum number of members
		#[pallet::constant]
		type MaxMembers: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(_);


	#[pallet::storage]
	#[pallet::getter(fn get_members)]
	pub type Members<T:Config> = StorageValue<_, BoundedVec<T::AccountId, T::MaxMembers>,ValueQuery>;


	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		//Time and AccountId for new member added event
		NewMemberAdded(T::BlockNumber, T::AccountId),
		// Time and AccountId for member removed
		MemberRemoved(T::BlockNumber, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		NotAuthorized,
		RootCannotBeMember,
		MembersLimitExceeded,
		MemberNotFound,

	}


	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn add_member(
			origin:OriginFor<T>,
			member: T::AccountId,

		) -> DispatchResult {

			ensure_signed(origin)?;


			//updating the storage
			<Members<T>>::try_mutate(|mem_vec| {
				mem_vec.try_push(member.clone())
			}).map_err(|_| <Error<T>>::MembersLimitExceeded)?;

			let time = <frame_system::Pallet<T>>::block_number();

			Self::deposit_event(Event::NewMemberAdded(time, member.clone()));
			Ok(())
		}

		#[pallet::weight(0)]
		pub fn remove_member(
			origin:OriginFor<T>,
			member: T::AccountId,

		) -> DispatchResult {
			ensure_signed(origin)?;
			// changing the storage
			<Members<T>>::try_mutate(|mem_vec|{
				if let Some(index) = mem_vec.iter().position(|mem| *mem == member.clone()){
					mem_vec.remove(index);
					return Ok(());
				}
				Err(())
			}).map_err(|_| <Error<T>>::MemberNotFound)?;


			let time = <frame_system::Pallet<T>>::block_number();

			Self::deposit_event(Event::MemberRemoved(time,member.clone()));

			Ok(())
		}
	}

  }


