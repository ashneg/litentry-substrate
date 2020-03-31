#![cfg_attr(not(feature = "std"), no_std)]

use balances;
use codec::{Decode, Encode};
use frame_support::traits::Randomness;
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage,
    dispatch::DispatchResult,
    ensure,
    weights::{SimpleDispatchInfo},
    StorageMap, StorageValue,
};
use frame_system::{self as system, ensure_signed};
use sp_runtime::{
    traits::{
        AtLeast32Bit, Bounded, Hash, Member,
        Saturating, StaticLookup, Zero,
    },
    RuntimeDebug,
};
// use litentry_weights::Linear;

#[cfg(test)]
mod mock;

// mod claim;
#[cfg(test)]
mod tests;

#[derive(Encode, Decode, Default, Clone, PartialEq, RuntimeDebug)]
pub struct Identity<Hash> {
    id: Hash,
}

#[derive(Encode, Decode, Default, Clone, PartialEq, RuntimeDebug)]
pub struct AuthorizedToken<Hash, Balance> {
    id: Hash,
    cost: Balance,
    data: u64,
    datatype: u64,
    expired: u64,
}

type AuthorizedTokenOf<T> =
    AuthorizedToken<<T as system::Trait>::Hash, <T as balances::Trait>::Balance>;
type IdentityOf<T> = Identity<<T as system::Trait>::Hash>;

pub trait Trait: balances::Trait + system::Trait {
    // Add other types and constants required to configure this pallet.
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    type Randomness: Randomness<Self::Hash>;
}

decl_event!(
    pub enum Event<T>
        where
            <T as frame_system::Trait>::AccountId,
            <T as frame_system::Trait>::Hash,
            //<T as balances::Trait>::Balance
        {
            IdentityCreated(AccountId, Hash),
            AuthorizedTokenCreated(AccountId, Hash, Hash),
            AuthorizedTokenTransferred(AccountId, AccountId, Hash),
            //ACTION: Create a `Transferred` event here
        }
);

decl_storage! {
    trait Store for Module<T: Trait> as LitentryStorage {
        // Identity: Declare storage and getter functions here
        Identities get(identity): map hasher(blake2_128_concat) T::Hash => IdentityOf<T>;
        IdentityOwner get(owner_of_identity): map hasher(blake2_128_concat) T::Hash => Option<T::AccountId>;

        IdentitiesCount get(identities_count): u64;
        IdentitiesArray get(identity_by_index): map hasher(blake2_128_concat) u64 => T::Hash;
        IdentitiesIndex get(identity_index): map hasher(blake2_128_concat) T::Hash => u64;

        OwnedIdentitiesCount get(identities_count_of_owner): map hasher(blake2_128_concat) T::AccountId => u64;
        OwnedIdentitiesArray get(identity_by_index_of_owner): map hasher(blake2_128_concat) (T::AccountId, u64) => T::Hash;
        OwnedIdentitiesIndex get(identity_index_of_owner): map hasher(blake2_128_concat) T::Hash => u64;

        // AuthorizedToken: Declare storage and getter functions here
        AuthorizedTokens get(token): map hasher(blake2_128_concat) T::Hash => AuthorizedTokenOf<T>;
        AuthorizedTokenOwner get(owner_of_token): map hasher(blake2_128_concat) T::Hash => Option<T::AccountId>;
        AuthorizedTokenIdentity get(identity_of_token): map hasher(blake2_128_concat) T::Hash => Option<T::Hash>;

        AuthorizedTokensCount get(tokens_count): u64;
        AuthorizedTokensArray get(token_by_index): map hasher(blake2_128_concat) u64 => T::Hash;
        AuthorizedTokensIndex get(token_index): map hasher(blake2_128_concat) T::Hash => u64;

        OwnedAuthorizedTokensCount get(tokens_count_of_owner): map hasher(blake2_128_concat) T::AccountId => u64;
        OwnedAuthorizedTokensArray get(token_by_index_of_owner): map hasher(blake2_128_concat) (T::AccountId, u64) => T::Hash;
        OwnedAuthorizedTokensIndex get(token_index_of_owner): map hasher(blake2_128_concat) T::Hash => u64;

        // Identity to token map
        IdentityAuthorizedTokensCount get(tokens_count_of_identity): map hasher(blake2_128_concat) T::Hash => u64;
        IdentityAuthorizedTokensArray get(token_by_index_of_identity): map hasher(blake2_128_concat) (T::Hash, u64) => T::Hash;
        IdentityAuthorizedTokensIndex get(token_index_of_identity): map hasher(blake2_128_concat) T::Hash => u64;

        Nonce: u64;
    }
}

// The pallet's errors
decl_error! {
    pub enum Error for Module<T: Trait> {
        /// Value was None
        NoneValue,
        /// Value reached maximum and cannot be incremented further
        StorageOverflow,
    }
}

decl_module! {

    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        type Error = Error<T>;

        fn deposit_event() = default;

        // public functions
        #[weight = SimpleDispatchInfo::FixedNormal(700)]
        pub fn register_identity(origin) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            let random_hash = Self::get_hash(&sender);
            let new_identity = Identity {
                id: random_hash
            };
            Self::mint_identity(sender, random_hash, new_identity)

            // <Nonce<T>>::mutate(|n| *n += 1);
            // Ok(())
        }

        #[weight = SimpleDispatchInfo::FixedNormal(700)]
        fn register_identity_with_id(origin, identity_id: T::Hash) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let new_identity = Identity {
                id: identity_id,
            };
            // sp_runtime::print("new identity registered");

            Self::mint_identity(sender, identity_id, new_identity)

            // Ok(())
        }

        #[weight = SimpleDispatchInfo::FixedNormal(1200)]
        fn issue_token(
            origin,
            to: T::AccountId,
            identity_id: T::Hash,
            cost: T::Balance,
            data: u64,
            datatype:u64,
            expired: u64) -> DispatchResult {

            let _sender = ensure_signed(origin)?;
            let id = Self::get_hash(&_sender);

            let new_token = AuthorizedToken {
                id,
                cost,
                data,
                datatype,
                expired
            };

            Self::mint_token(to, identity_id, id, new_token)?;
            <frame_system::Module<T>>::inc_account_nonce(&_sender);

            Ok(())
        }

        #[weight = SimpleDispatchInfo::FixedNormal(900)]
        fn transfer_token(origin, to: T::AccountId, token_id: T::Hash ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let owner = Self::owner_of_token(token_id).ok_or("No owner for this token")?;
            ensure!(owner == sender, "You do not own this token");

            Self::token_transfer_from(sender.clone(), to, token_id)?;
            <frame_system::Module<T>>::inc_account_nonce(&sender);

            Ok(())
        }

        // fn recall_token(origin, token: T::Hash) -> DispatchResult {}

        // fn recall_all_identity_token(origin, identity_id: T::Hash ) -> DispatchResult {}

        // fn update_token(origin, token: T::Hash, identity_data: String) -> DispatchResult {}

    }
}

impl<T: Trait> Module<T> {
    fn get_hash(sender: &T::AccountId) -> T::Hash {
        let nonce = <frame_system::Module<T>>::account_nonce(&sender);
        let random_hash_bytes = (T::Randomness::random_seed(), &sender, nonce)
            // .using_encoded(blake2_256);(
            .using_encoded(T::Hashing::hash);
        T::Hash::from(random_hash_bytes)
    }

    fn mint_identity(
        to: T::AccountId,
        identity_id: T::Hash,
        new_identity: IdentityOf<T>,
    ) -> DispatchResult {
        ensure!(
            !<IdentityOwner<T>>::contains_key(identity_id),
            "Identity already exists"
        );

        let owned_identities_count = Self::identities_count_of_owner(&to);
        let new_owned_identities_count = owned_identities_count
            .checked_add(1)
            .ok_or("Overflow adding a new identity to owner")?;

        let all_identities_count = Self::identities_count();
        let new_all_identities_count = all_identities_count
            .checked_add(1)
            .ok_or("Overflow adding a new identity to total supply")?;

        <Identities<T>>::insert(identity_id, new_identity);
        <IdentityOwner<T>>::insert(identity_id, &to);

        <IdentitiesArray<T>>::insert(all_identities_count, identity_id);

        <IdentitiesIndex<T>>::insert(identity_id, all_identities_count);
        <IdentitiesCount>::put(new_all_identities_count);

        <OwnedIdentitiesArray<T>>::insert((to.clone(), owned_identities_count), identity_id);
        <OwnedIdentitiesIndex<T>>::insert(identity_id, owned_identities_count);
        <OwnedIdentitiesCount<T>>::insert(&to, new_owned_identities_count);
        <frame_system::Module<T>>::inc_account_nonce(&to);
        Self::deposit_event(RawEvent::IdentityCreated(to, identity_id));

        Ok(())
    }

    fn mint_token(
        to: T::AccountId,
        identity_id: T::Hash,
        token_id: T::Hash,
        new_token: AuthorizedToken<T::Hash, T::Balance>,
    ) -> DispatchResult {
        ensure!(
            <IdentityOwner<T>>::contains_key(identity_id),
            "Identity doesn't exist."
        );
        ensure!(
            !<AuthorizedTokenOwner<T>>::contains_key(token_id),
            "Token already exists"
        );

        let owned_tokens_count = Self::tokens_count_of_owner(&to);
        let new_owned_tokens_count = owned_tokens_count
            .checked_add(1)
            .ok_or("Overflow adding a new token to owner")?;

        let identity_tokens_count = Self::tokens_count_of_identity(identity_id);
        let new_identity_tokens_count = identity_tokens_count
            .checked_add(1)
            .ok_or("Overflow adding a new token to identity")?;

        let all_tokens_count = Self::tokens_count();
        let new_all_tokens_count = all_tokens_count
            .checked_add(1)
            .ok_or("Overflow adding a new token to total supply")?;

        <AuthorizedTokens<T>>::insert(token_id, new_token);
        <AuthorizedTokenOwner<T>>::insert(token_id, &to);
        <AuthorizedTokenIdentity<T>>::insert(token_id, &identity_id);

        <AuthorizedTokensArray<T>>::insert(all_tokens_count, token_id);
        <AuthorizedTokensIndex<T>>::insert(token_id, all_tokens_count);
        <AuthorizedTokensCount>::put(new_all_tokens_count);

        <OwnedAuthorizedTokensArray<T>>::insert((to.clone(), owned_tokens_count), token_id);
        <OwnedAuthorizedTokensIndex<T>>::insert(token_id, owned_tokens_count);
        <OwnedAuthorizedTokensCount<T>>::insert(&to, new_owned_tokens_count);

        <IdentityAuthorizedTokensArray<T>>::insert((identity_id, identity_tokens_count), token_id);
        <IdentityAuthorizedTokensIndex<T>>::insert(token_id, identity_tokens_count);
        <IdentityAuthorizedTokensCount<T>>::insert(identity_id, new_identity_tokens_count);

        Self::deposit_event(RawEvent::AuthorizedTokenCreated(to, identity_id, token_id));

        Ok(())
    }

    fn token_transfer_from(
        from: T::AccountId,
        to: T::AccountId,
        token_id: T::Hash,
    ) -> DispatchResult {
        let owner = Self::owner_of_token(token_id).ok_or("No owner for this token")?;

        ensure!(owner == from, "'from' account does not own this token");

        let owned_token_count_from = Self::tokens_count_of_owner(&from);
        let owned_token_count_to = Self::tokens_count_of_owner(&to);

        let new_owned_token_count_to = owned_token_count_to
            .checked_add(1)
            .ok_or("Transfer causes overflow of 'to' token balance")?;

        let new_owned_token_count_from = owned_token_count_from
            .checked_sub(1)
            .ok_or("Transfer causes underflow of 'from' token balance")?;

        let token_index = <OwnedAuthorizedTokensIndex<T>>::get(token_id);
        if token_index != new_owned_token_count_from {
            let last_token_id =
                <OwnedAuthorizedTokensArray<T>>::get((from.clone(), new_owned_token_count_from));
            <OwnedAuthorizedTokensArray<T>>::insert((from.clone(), token_index), last_token_id);
            <OwnedAuthorizedTokensIndex<T>>::insert(last_token_id, token_index);
        }

        <AuthorizedTokenOwner<T>>::insert(&token_id, &to);
        <OwnedAuthorizedTokensIndex<T>>::insert(token_id, owned_token_count_to);

        <OwnedAuthorizedTokensArray<T>>::remove((from.clone(), new_owned_token_count_from));
        <OwnedAuthorizedTokensArray<T>>::insert((to.clone(), owned_token_count_to), token_id);

        <OwnedAuthorizedTokensCount<T>>::insert(&from, new_owned_token_count_from);
        <OwnedAuthorizedTokensCount<T>>::insert(&to, new_owned_token_count_to);

        Self::deposit_event(RawEvent::AuthorizedTokenTransferred(from, to, token_id));

        Ok(())
    }
}
