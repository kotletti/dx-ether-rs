pub trait EtherGetBalancerPort: Send + Sync {
    type Output;
    type Error;

    fn eth_get_balance(
        &self,
        address: &str,
        block: Option<String>,
    ) -> impl Future<Output = Result<Self::Output, Self::Error>>;
}
