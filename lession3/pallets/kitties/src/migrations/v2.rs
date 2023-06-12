use frame_support::{
    pallet_prelude::*,
    storage::StoragePrefixedMap,
    traits::GetStorageVersion,
    weights::Weight,
};

use frame_system::pallet_prelude::*;
use frame_support::{migration::storage_key_iter, Blake2_128Concat};

use crate::{Config, Pallet, Kitties, Kitty, KittyId};

#[derive(Encode, Decode, Clone, Debug, TypeInfo, MaxEncodedLen, PartialEq, Eq, )]
pub struct OldKitty {
    pub dna: [u8; 16],
    pub name: [u8; 4],
}

pub fn migrate<T: Config>() -> Weight {
    let on_chain_version = Pallet::<T>::on_chain_storage_version();
    let current_version = Pallet::<T>::current_storage_version();

    if on_chain_version != 0 {
        return Weight::zero();
    }

    if current_version != 1 {
        return Weight::zero();
    }

    let module = Kitties::<T>::module_prefix();
    let item = Kitties::<T>::storage_prefix();
    
    for (index, kitty) in storage_key_iter::<KittyId, OldKitty, Blake2_128Concat>(module, item).drain() {
        let n = kitty.name;
        let mut new_name: [u8; 8] = [0; 8];
        new_name[0] = n[0];
        new_name[1] = n[1];
        new_name[2] = n[2];
        new_name[3] = n[3];

        let new_kitty = Kitty {
            dna: kitty.dna,
            name: new_name,
        };

        Kitties::<T>::insert(index, &new_kitty);
    }
    Weight::zero()
}