#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::sp_runtime::{traits::Hash, RuntimeDebug};
use frame_support::traits::Randomness;
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure,
    StorageMap, StorageValue,
};
use frame_system::{self as system, ensure_signed};
use pallet_balances;
use sp_std::prelude::*;
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
    AuthorizedToken<<T as system::Trait>::Hash, <T as pallet_balances::Trait>::Balance>;
type IdentityOf<T> = Identity<<T as system::Trait>::Hash>;

pub trait Trait: pallet_balances::Trait + system::Trait {
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
            AuthorizedTokenCreated(Hash, Hash, Hash),
            AuthorizedTokenTransferred(AccountId, Hash, Hash),
            AuthenticaterRequest(AccountId, Hash, Vec<u8>),
            //ACTION: Create a `Transferred` event here
        }
);

decl_storage! {
    trait Store for Module<T: Trait> as LitentryStorage {
        // Identity: Declare storage and getter functions here
        Identities get(fn identity): map hasher(blake2_128_concat) T::Hash => IdentityOf<T>;
        IdentityOwner get(fn owner_of_identity): map hasher(blake2_128_concat) T::Hash => Option<T::AccountId>;

        IdentitiesCount get(fn identities_count): u64;
        IdentitiesArray get(fn identity_by_index): map hasher(blake2_128_concat) u64 => T::Hash;
        IdentitiesIndex get(fn identity_index): map hasher(blake2_128_concat) T::Hash => u64;

        OwnedIdentitiesCount get(fn identities_count_of_owner): map hasher(blake2_128_concat) T::AccountId => u64;
        OwnedIdentitiesArray get(fn identity_by_index_of_owner): map hasher(blake2_128_concat) (T::AccountId, u64) => T::Hash;
        OwnedIdentitiesIndex get(fn identity_index_of_owner): map hasher(blake2_128_concat) T::Hash => u64;

        // AuthorizedToken: Declare storage and getter functions here
        AuthorizedTokens get(fn token): map hasher(blake2_128_concat) T::Hash => AuthorizedTokenOf<T>;
        AuthorizedTokenOwner get(fn owner_identity_of_token): map hasher(blake2_128_concat) T::Hash => Option<T::Hash>;
        AuthorizedTokenIdentity get(fn issuer_identity_of_token): map hasher(blake2_128_concat) T::Hash => Option<T::Hash>;

        AuthorizedTokensCount get(fn tokens_count): u64;
        AuthorizedTokensArray get(fn token_by_index): map hasher(blake2_128_concat) u64 => T::Hash;
        AuthorizedTokensIndex get(fn token_index): map hasher(blake2_128_concat) T::Hash => u64;

        OwnedAuthorizedTokensCount get(fn owned_tokens_count_of_identity): map hasher(blake2_128_concat) T::Hash => u64;
        OwnedAuthorizedTokensArray get(fn owned_token_by_index_of_identity): map hasher(blake2_128_concat) (T::Hash, u64) => T::Hash;
        OwnedAuthorizedTokensIndex get(fn token_index_of_owner_identity): map hasher(blake2_128_concat) T::Hash => u64;

        // Identity to token map
        IdentityAuthorizedTokensCount get(fn issued_tokens_count_of_identity): map hasher(blake2_128_concat) T::Hash => u64;
        IdentityAuthorizedTokensArray get(fn issued_token_by_index_of_identity): map hasher(blake2_128_concat) (T::Hash, u64) => T::Hash;
        IdentityAuthorizedTokensIndex get(fn token_index_of_issuer_identity): map hasher(blake2_128_concat) T::Hash => u64;

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
        #[weight = 700]
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

        #[weight = 700]
        fn register_identity_with_id(origin, identity_id: T::Hash) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let new_identity = Identity {
                id: identity_id,
            };
            // sp_runtime::print("new identity registered");

            Self::mint_identity(sender, identity_id, new_identity)

            // Ok(())
        }

        #[weight = 1200]
        fn issue_token(
            origin,
            to: T::Hash,
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

        #[weight = 900]
        fn transfer_token(origin, to: T::Hash, token_id: T::Hash ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            Self::token_transfer_from(sender.clone(), to, token_id)?;
            <frame_system::Module<T>>::inc_account_nonce(&sender);

            Ok(())
        }

        #[weight = 100]
        fn request_authentication(origin, token_id: T::Hash, url:Vec<u8>) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            Self::deposit_event(RawEvent::AuthenticaterRequest(sender, token_id, url));
            Ok(())
        }

        // fn recall_token(origin, token: T::Hash) -> DispatchResult {}

        // fn recall_all_identity_token(origin, identity_id: T::Hash ) -> DispatchResult {}

        // fn update_token(origin, token: T::Hash, identity_data: String) -> DispatchResult {}

    }
}

impl<T: Trait> Module<T> {
    // fn is_token_owner(token: &T::Hash, identity: &T::Hash) -> bool {
    //     <IdentityAuthorizedTokensIndex<T>>::get(token).contains(who)
    // }
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
        receiver_identity: T::Hash,
        issuer_identity: T::Hash,
        token_id: T::Hash,
        new_token: AuthorizedToken<T::Hash, T::Balance>,
    ) -> DispatchResult {
        ensure!(
            <IdentityOwner<T>>::contains_key(issuer_identity),
            "Identity doesn't exist."
        );
        ensure!(
            !<AuthorizedTokenOwner<T>>::contains_key(token_id),
            "Token already exists"
        );

        let owned_tokens_count = Self::owned_tokens_count_of_identity(receiver_identity);
        let new_owned_tokens_count = owned_tokens_count
            .checked_add(1)
            .ok_or("Overflow adding a new token to owner")?;

        let identity_tokens_count = Self::owned_tokens_count_of_identity(issuer_identity);
        let new_identity_tokens_count = identity_tokens_count
            .checked_add(1)
            .ok_or("Overflow adding a new token to identity")?;

        let all_tokens_count = Self::tokens_count();
        let new_all_tokens_count = all_tokens_count
            .checked_add(1)
            .ok_or("Overflow adding a new token to total supply")?;

        <AuthorizedTokens<T>>::insert(token_id, new_token);
        <AuthorizedTokenOwner<T>>::insert(token_id, receiver_identity);
        <AuthorizedTokenIdentity<T>>::insert(token_id, issuer_identity);

        <AuthorizedTokensArray<T>>::insert(all_tokens_count, token_id);
        <AuthorizedTokensIndex<T>>::insert(token_id, all_tokens_count);
        <AuthorizedTokensCount>::put(new_all_tokens_count);

        <OwnedAuthorizedTokensArray<T>>::insert((receiver_identity, owned_tokens_count), token_id);
        <OwnedAuthorizedTokensIndex<T>>::insert(token_id, owned_tokens_count);
        <OwnedAuthorizedTokensCount<T>>::insert(receiver_identity, new_owned_tokens_count);

        <IdentityAuthorizedTokensArray<T>>::insert(
            (issuer_identity, identity_tokens_count),
            token_id,
        );
        <IdentityAuthorizedTokensIndex<T>>::insert(token_id, identity_tokens_count);
        <IdentityAuthorizedTokensCount<T>>::insert(issuer_identity, new_identity_tokens_count);

        Self::deposit_event(RawEvent::AuthorizedTokenCreated(
            receiver_identity,
            issuer_identity,
            token_id,
        ));

        Ok(())
    }

    fn token_transfer_from(
        from_account: T::AccountId,
        receiver_identity: T::Hash,
        token_id: T::Hash,
    ) -> DispatchResult {
        let sender_identity =
            Self::owner_identity_of_token(token_id).ok_or("No owner identity for this token")?;
        let owner_account =
            Self::owner_of_identity(sender_identity).ok_or("No owner account for this token")?;
        ensure!(
            owner_account == from_account,
            "sender do not own this token"
        );

        let owned_token_count_sender = Self::owned_tokens_count_of_identity(&sender_identity);
        let owned_token_count_receiver = Self::owned_tokens_count_of_identity(&receiver_identity);

        let new_owned_token_count_receiver = owned_token_count_receiver
            .checked_add(1)
            .ok_or("Transfer causes overflow of 'to' token balance")?;

        let new_owned_token_count_sender = owned_token_count_sender
            .checked_sub(1)
            .ok_or("Transfer causes underflow of 'from' token balance")?;

        let token_index_sender = Self::token_index_of_owner_identity(token_id);
        if token_index_sender != new_owned_token_count_sender {
            let last_token_id = Self::owned_token_by_index_of_identity((
                sender_identity,
                new_owned_token_count_sender,
            ));
            <OwnedAuthorizedTokensArray<T>>::insert(
                (sender_identity, token_index_sender),
                last_token_id,
            );
            <OwnedAuthorizedTokensIndex<T>>::insert(last_token_id, token_index_sender);
        }

        <AuthorizedTokenOwner<T>>::insert(token_id, receiver_identity);
        <OwnedAuthorizedTokensIndex<T>>::insert(token_id, owned_token_count_receiver);

        <OwnedAuthorizedTokensArray<T>>::remove((
            sender_identity,
            new_owned_token_count_sender,
        ));
        <OwnedAuthorizedTokensArray<T>>::insert(
            (receiver_identity, owned_token_count_receiver),
            token_id,
        );

        <OwnedAuthorizedTokensCount<T>>::insert(sender_identity, new_owned_token_count_sender);
        <OwnedAuthorizedTokensCount<T>>::insert(receiver_identity, new_owned_token_count_receiver);

        Self::deposit_event(RawEvent::AuthorizedTokenTransferred(
            from_account,
            receiver_identity,
            token_id,
        ));

        Ok(())
    }
}
