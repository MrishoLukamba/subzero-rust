#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {

    use frame_support::{
        sp_runtime::traits::Hash,
        dispatch::DispatchResult,
        traits::Randomness,
        pallet_prelude::*
    };
    
    use frame_system::pallet_prelude::*;
    use sp_io::hashing::blake2_128;
	use scale_info::TypeInfo;
    use sp_std::vec::Vec;
    use frame_support::sp_runtime::traits::Printable;
    use frame_support::sp_runtime::print;
    

    #[pallet::config]
    pub trait Config: frame_system::Config {

        //this palet uses random function, so we must declare a random type and implemen randomness trait
        type Random: Randomness<Self::Hash, Self::BlockNumber>;
        //declaring event type
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    //custom struct
    
    #[derive(Encode, Decode,TypeInfo, Clone, RuntimeDebug)]
    #[scale_info(skip_type_params(T))]
    pub struct Profile<T: Config> {
        pub name: Vec<u8>,
        pub age: u32,
        pub id: T::Hash,
        pub gender: Vec<u8>,
        pub time_created: T::BlockNumber,
        pub lucky: T::Hash,
    }

    


    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn getProfile)]
    pub type ProfileDb<T: Config> = StorageMap<_,Blake2_128Concat,T::Hash, Profile<T>>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {

        ProfileCreated,
        ProfileDeleted,
    }

    #[pallet::error]
    pub enum Error<T> {

        ProfileExist,
    }

    #[pallet::hooks]
    impl <T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {

        #[pallet::weight(100)]
        pub fn createProfile(origin: OriginFor<T>, name: Vec<u8>, age: u32, gender: Vec<u8>,lucky_number:u8 ) -> DispatchResult {

            let _= ensure_signed(origin)?;
            let get_lucky = T::Random::random(&[lucky_number]).0;

            let time = <frame_system::Pallet<T>>::block_number();
            let profile_id = T::Hashing::hash_of(&name);
            
            let Outline = Profile {
                name : name,
                age: age,
                id: profile_id,
                gender: gender,
                time_created: time,
                lucky: get_lucky,
            };

            let key = T::Hashing::hash_of(&Outline);

            Self::deposit_event(Event::ProfileCreated);

            <ProfileDb<T>>::insert(key, Outline);
            

            Ok(())
        }

    }

}

