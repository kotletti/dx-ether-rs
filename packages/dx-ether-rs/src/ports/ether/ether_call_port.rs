pub trait EtherCallPort: Send + Sync {
    type Output;
    type Error;

    fn eth_call(
        &self,
        address: &str,
        calldata: &str,
        block: Option<String>,
    ) -> impl Future<Output = Result<Self::Output, Self::Error>>;
}
