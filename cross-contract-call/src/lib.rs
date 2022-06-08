use near_sdk::near_abi;

#[near_abi]
mod abi {
    use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
    use near_sdk::near_bindgen;

    near_sdk_abi::near_abi_ext!("../target/near/adder/abi.json");

    #[near_bindgen]
    #[derive(Default, BorshDeserialize, BorshSerialize)]
    pub struct Delegator {}

    #[near_bindgen]
    impl Delegator {
        pub fn delegate(
            &self,
            a: u32,
            b: u32,
            c: u32,
            d: u32,
            adder_account_id: near_sdk::AccountId,
        ) -> near_sdk::Promise {
            ext_adder::ext(adder_account_id).add(vec![a.into(), b.into()], vec![c.into(), d.into()])
        }
    }
}
