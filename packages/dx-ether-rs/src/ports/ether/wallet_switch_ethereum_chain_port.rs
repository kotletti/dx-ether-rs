pub trait WalletSwitchEthereumChainPort: Send + Sync {
    type Output;
    type Error;

    fn wallet_switch_ethereum_chain(
        &self,
        chain_id: &str,
    ) -> impl Future<Output = Result<Self::Output, Self::Error>>;
}
