#![no_std]
#![feature(generic_associated_types)]
const FIVE_MIN: u64 = 5 * 60;

mod claims;

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::contract]
pub trait DevNetFaucet {
    #[init]
    fn init(&self) {}

    #[only_owner]
    #[endpoint(setClaimsAddress)]
    fn set_claims_address(&self, address: ManagedAddress) {
        self.claims_address().set(&address);
    }

    #[only_owner]
    #[endpoint(changeClaimsOwnership)]
    fn change_claims_ownership(&self) {
        let new_owner = self.blockchain().get_owner_address();
        self.send()
            .change_owner_address(self.claims_address().get(), &new_owner)
            .execute_on_dest_context();
    }

    #[only_owner]
    #[endpoint(setRewardToken)]
    fn set_reward_token(&self, token_id: TokenIdentifier) {
        self.reward_token().set(&token_id);
    }

    #[endpoint(activateFaucet)]
    fn activate_faucet(&self) {
        let caller = self.blockchain().get_caller();
        let reward_token = self.reward_token().get();
        let current_timestamp = self.blockchain().get_block_timestamp();
        require!(
            self.last_faucet(&caller).get() <= current_timestamp - FIVE_MIN,
            "Cannot claim tokens so early after last claim"
        );
        self.last_faucet(&caller).set(current_timestamp);
        self.send().direct(
            &caller,
            &reward_token,
            0u64,
            &(BigUint::from(10u64) * BigUint::from(10u64).pow(18u32)),
            &[],
        );
        self.claims_proxy(self.claims_address().get())
            .add_claim(&caller, claims::ClaimType::Reward)
            .add_token_transfer(
                reward_token,
                0,
                BigUint::from(10u64) * BigUint::from(10u64).pow(18u32),
            )
            .execute_on_dest_context();
    }

    #[storage_mapper("rewardToken")]
    fn reward_token(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getLastFaucet)]
    #[storage_mapper("lastFaucet")]
    fn last_faucet(&self, address: &ManagedAddress) -> SingleValueMapper<u64>;

    #[storage_mapper("claimsAddress")]
    fn claims_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[proxy]
    fn claims_proxy(&self, sc_address: ManagedAddress) -> claims::Proxy<Self::Api>;
}
