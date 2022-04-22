use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;
//usage:
//cargo test -p poe 

#[test]
fn create_claim_works(){ //定义一个测试用例
    new_test_ext().execute_with(|| { //使用mock.rs里定义的帮助方法，帮助执行测试代码
        let claim = vec![0, 1];  //构造create_claim所需参数（存证的内容）
        assert_ok!(PoeModule::create_claim(Origin::signed(1),claim.clone())); //断言create_claim会返回一个ok
        assert_eq!(Proofs::<Test>::get(&claim),(1, frame_system::Module::<Test>::block_number())); //断言create_claim修改后的链上状态
    })

}

#[test]
fn revoke_claim_works() { 
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());//先创建一个存证

        assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
    })
}


#[test]
fn change_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());//先创建一个存证

        assert_ok!(PoeModule::change_owner_claim(Origin::signed(1),claim.clone(),2_u64));

        assert_eq!(Proofs::<Test>::get(&claim),(2, frame_system::Module::<Test>::block_number())); //断言create_claim修改后的链上状态
    })
}

