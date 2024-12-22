use elrond_wasm::storage::*;

#[elrond_wasm::module]
pub trait Storage {
    #[storage_mapper("issued_tokens")]
    fn issued_tokens(&self, issuer: &ManagedAddress) -> SingleValueMapper<u64>;

    #[storage_mapper("token_balances")]
    fn token_balances(&self, token_id: &TokenIdentifier) -> SingleValueMapper<u64>;
}