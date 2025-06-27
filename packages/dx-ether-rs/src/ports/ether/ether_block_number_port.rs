pub trait EtherBlockNumberPort: Send + Sync {
    type Output;
    type Error;

    fn eth_block_number(&self) -> impl Future<Output = Result<Self::Output, Self::Error>>;
}
