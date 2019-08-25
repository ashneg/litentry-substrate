/// A runtime module template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references


/// For more guidance on Substrate modules, see the example module
/// https://github.com/paritytech/substrate/blob/master/srml/example/src/lib.rs

use support::{decl_module, decl_storage, decl_event, StorageValue, dispatch::Result};
use system::ensure_signed;
use sr_primitives::traits::Hash;
//use parity_codec::{Encode, Decode, WrapperTypeDecode};
use codec::{Encode, Decode};

/// The module's configuration trait.
pub trait Trait: balances::Trait + system::Trait {
	// TODO: Add other types and constants required configure this module.

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Identity<THash> {
	id: THash,
}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct AuthorizedToken<THash, TBalance> {
	id: THash,
	cost: TBalance,
	data: u64,
	datatype: u64,
	expired: u64,
}

decl_event!(
    pub enum Event<T>
        where
            <T as system::Trait>::AccountId,
            <T as system::Trait>::Hash,
//            <T as balances::Trait>::Balance
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
        Identities get(identity): map T::Hash => Identity<T::Hash>;
        IdentityOwner get(owner_of_identity): map T::Hash => Option<T::AccountId>;

        IdentitiesCount get(identities_count): u64;
        IdentitiesArray get(identity_by_index): map u64 => T::Hash;
        IdentitiesIndex get(identity_index): map T::Hash => u64;

        OwnedIdentitiesCount get(identities_count_of_owner): map T::AccountId => u64;
        OwnedIdentitiesArray get(identity_by_index_of_owner): map (T::AccountId, u64) => T::Hash;
        OwnedIdentitiesIndex get(identity_index_of_owner): map T::Hash => u64;

        // AuthorizedToken: Declare storage and getter functions here
        AuthorizedTokens get(token): map T::Hash => AuthorizedToken<T::Hash, T::Balance>;
        AuthorizedTokenOwner get(owner_of_token): map T::Hash => Option<T::AccountId>;
        AuthorizedTokenIdentity get(identity_of_token): map T::Hash => Option<T::Hash>;

        AuthorizedTokensCount get(tokens_count): u64;
        AuthorizedTokensArray get(token_by_index): map u64 => T::Hash;
        AuthorizedTokensIndex get(token_index): map T::Hash => u64;

        OwnedAuthorizedTokensCount get(tokens_count_of_owner): map T::AccountId => u64;
        OwnedAuthorizedTokensArray get(token_by_index_of_owner): map (T::AccountId, u64) => T::Hash;
        OwnedAuthorizedTokensIndex get(token_index_of_owner): map T::Hash => u64;

        // Identity to token map
        IdentityAuthorizedTokensCount get(tokens_count_of_identity): map T::Hash => u64;
        IdentityAuthorizedTokensArray get(token_by_index_of_identity): map (T::Hash, u64) => T::Hash;
        IdentityAuthorizedTokensIndex get(token_index_of_identity): map T::Hash => u64;

        Nonce: u64;
    }
}

// The module's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing events
		// this is needed only if you are using events in your module
		fn deposit_event<T>() = default;

		// Just a dummy entry point.
		// function that can be called by the external world as an extrinsics call
		// takes a parameter of the type `AccountId`, stores it and emits an event
		pub fn do_something(origin, something: u32) -> Result {
			// TODO: You only need this if you want to check it was signed.

			Ok(())
		}
	}
}



/// tests for this module
#[cfg(test)]
mod tests {
	use super::*;

	use runtime_io::with_externalities;
	use primitives::{H256, Blake2Hasher};
	use support::{impl_outer_origin, assert_ok, parameter_types};
	use sr_primitives::{traits::{BlakeTwo256, IdentityLookup}, testing::Header};
	use sr_primitives::weights::Weight;
	use sr_primitives::Perbill;

	impl_outer_origin! {
		pub enum Origin for Test {}
	}

	// For testing the module, we construct most of a mock runtime. This means
	// first constructing a configuration type (`Test`) which `impl`s each of the
	// configuration traits of modules we want to use.
	#[derive(Clone, Eq, PartialEq)]
	pub struct Test;
	parameter_types! {
		pub const BlockHashCount: u64 = 250;
		pub const MaximumBlockWeight: Weight = 1024;
		pub const MaximumBlockLength: u32 = 2 * 1024;
		pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
	}
	impl system::Trait for Test {
		type Origin = Origin;
		type Call = ();
		type Index = u64;
		type BlockNumber = u64;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type WeightMultiplierUpdate = ();
		type Event = ();
		type BlockHashCount = BlockHashCount;
		type MaximumBlockWeight = MaximumBlockWeight;
		type MaximumBlockLength = MaximumBlockLength;
		type AvailableBlockRatio = AvailableBlockRatio;
		type Version = ();
	}
	impl Trait for Test {
		type Event = ();
	}
	type TemplateModule = Module<Test>;

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
		system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
	}

	#[test]
	fn it_works_for_default_value() {
		with_externalities(&mut new_test_ext(), || {
			// Just a dummy test for the dummy funtion `do_something`
			// calling the `do_something` function with a value 42
			assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
			// asserting that the stored value is equal to what we stored
			assert_eq!(TemplateModule::something(), Some(42));
		});
	}
}
