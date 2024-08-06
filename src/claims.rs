multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Clone, Debug, TypeAbi)]
pub enum ClaimType {
    Reward,
    Airdrop,
    Allocation,
    Royalties
}

#[multiversx_sc::proxy]
pub trait ClaimsContract {
    #[payable("*")]
    #[endpoint(addClaim)]
    fn add_claim(&self, address: &ManagedAddress, claim_type: ClaimType);
}
