#![no_std]
const TIME_LIMIT: u64 = 120 * 60; // 120 min

mod claims;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::contract]
pub trait DevNetFaucet {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

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
            .execute_on_dest_context::<()>();
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
            self.last_faucet(&caller).get() <= current_timestamp - TIME_LIMIT,
            "Cannot claim tokens so early after last claim"
        );
        self.last_faucet(&caller).set(current_timestamp);
        self.send().direct_esdt(
            &caller,
            &reward_token,
            0u64,
            &(BigUint::from(1000u64) * BigUint::from(10u64).pow(18u32)),
        );
        self.claims_proxy(self.claims_address().get())
            .add_claim(&caller, claims::ClaimType::Reward)
            .with_esdt_transfer(EsdtTokenPayment::new(
                reward_token.clone(),
                0u64,
                BigUint::from(2u64) * BigUint::from(10u64).pow(18u32),
            ))
            .execute_on_dest_context::<()>();
        self.claims_proxy(self.claims_address().get())
            .add_claim(&caller, claims::ClaimType::Airdrop)
            .with_esdt_transfer(EsdtTokenPayment::new(
                reward_token.clone(),
                0u64,
                BigUint::from(2u64) * BigUint::from(10u64).pow(18u32),
            ))
            .execute_on_dest_context::<()>();
        self.claims_proxy(self.claims_address().get())
            .add_claim(&caller, claims::ClaimType::Allocation)
            .with_esdt_transfer(EsdtTokenPayment::new(
                reward_token.clone(),
                0u64,
                BigUint::from(2u64) * BigUint::from(10u64).pow(18u32),
            ))
            .execute_on_dest_context::<()>();

        self.claims_proxy(self.claims_address().get())
            .add_claim(&caller, claims::ClaimType::Royalties)
            .with_esdt_transfer(EsdtTokenPayment::new(
                reward_token.clone(),
                0u64,
                BigUint::from(2u64) * BigUint::from(10u64).pow(18u32),
            ))
            .execute_on_dest_context::<()>();
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
