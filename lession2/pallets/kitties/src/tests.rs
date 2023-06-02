use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_test() {
    assert_eq!(1, 1);
}

#[test]
fn it_works_for_create() {
    new_test_ext().execute_with(|| {
        let kitty_id = 0;
        let account_id = 1;

        assert_eq!(KittiesModule::next_kitty_id(), kitty_id);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));

        assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 1);
        assert_eq!(KittiesModule::kitties(kitty_id).is_some(), true);
        assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
        assert_eq!(KittiesModule::kitty_parents(kitty_id), None);

        crate::NextKittyId::<Test>::set(crate::KittyId::max_value());
        assert_noop!(
            KittiesModule::create(RuntimeOrigin::signed(account_id)),
            Error::<Test>::InvalidKittyId
        );
		
        let event_record: frame_system::EventRecord<_, _> = System::events().pop().unwrap();
        let pallet_event: Event<Test> = event_record.event.try_into().unwrap();

        let (tmp_who, tmp_kitty_id, _) = match pallet_event {
            Event::KittyCreated{who, kitty_id, kitty} => (who, kitty_id, kitty),
            _ => panic!("unexpected error")
        };
        assert_eq!(tmp_kitty_id, kitty_id);
        assert_eq!(tmp_who, account_id);
    })
}

#[test]
fn it_works_for_breed() {
    new_test_ext().execute_with(|| {
        let kitty_id = 0;
        let account_id = 1;

        // crate::NextKittyId::<Test>::set(crate::KittyId::max_value());
        assert_noop!(
            KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id),
            Error::<Test>::SameKittyId
        );
        assert_noop!(
            KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id + 1),
            Error::<Test>::InvalidKittyId
        );

        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));

        assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 2);

        assert_ok!(KittiesModule::breed(
            RuntimeOrigin::signed(account_id), 
            kitty_id,
            kitty_id + 1
        ));

        let breed_kitty_id = 2;
        assert_eq!(KittiesModule::next_kitty_id(), breed_kitty_id + 1);
        assert_eq!(KittiesModule::kitties(breed_kitty_id).is_some(),true);
        assert_eq!(KittiesModule::kitty_owner(breed_kitty_id), Some(account_id));

        assert_eq!(
            KittiesModule::kitty_parents(breed_kitty_id),
            Some((kitty_id, kitty_id + 1))
        );
        
        let event_record: frame_system::EventRecord<_, _> = System::events().pop().unwrap();
        let pallet_event: Event<Test> = event_record.event.try_into().unwrap();

        let (tmp_who, tmp_kitty_id, _) = match pallet_event {
            Event::KittyBreed{who, kitty_id, kitty} => (who, kitty_id, kitty),
            _ => panic!("unexpected error")
        };
        assert_eq!(tmp_kitty_id, breed_kitty_id);
        assert_eq!(tmp_who, account_id);
    })
}

#[test]
fn it_works_for_transfer() {
    new_test_ext().execute_with(|| {
        let kitty_id = 0;
        let account_id = 1;
        let recipient = 2;

        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
        assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));

        assert_noop!(KittiesModule::transfer(
            RuntimeOrigin::signed(recipient),
            kitty_id,
            account_id
        ), Error::<Test>::NotOwner);

        assert_ok!(KittiesModule::transfer(
            RuntimeOrigin::signed(account_id),
            kitty_id,
            recipient
        ));

        System::assert_last_event(Event::KittyTransferred { who: account_id, recipient: recipient, kitty_id: kitty_id }.into());

        assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(recipient));

        assert_ok!(KittiesModule::transfer(
            RuntimeOrigin::signed(recipient),
            kitty_id,
            account_id
        ));
        assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
    });
}