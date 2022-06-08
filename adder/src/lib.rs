use near_sdk::near_abi;

#[near_abi]
mod abi {
    use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
    use near_sdk::near_bindgen;
    use near_sdk::schemars::JsonSchema;
    use near_sdk::serde::{Deserialize, Serialize};

    #[derive(JsonSchema, Serialize, Deserialize)]
    pub struct Pair(u32, u32);

    #[near_bindgen]
    #[derive(Default, BorshDeserialize, BorshSerialize)]
    pub struct Adder {}

    #[near_bindgen]
    impl Adder {
        pub fn add(&self, a: Pair, b: Pair) -> Pair {
            Pair(a.0 + b.0, a.1 + b.1)
        }
    }
}
