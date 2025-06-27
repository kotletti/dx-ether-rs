pub trait EtherAccountsPort: Send + Sync {
    type Output;
    type Error;

    fn eth_accounts(&self) -> impl Future<Output = Result<Self::Output, Self::Error>>;
}
