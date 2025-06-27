pub trait EtherChainIdPort: Send + Sync {
    type Output;
    type Error;

    fn eth_chain_id(&self) -> impl Future<Output = Result<Self::Output, Self::Error>>;
}
