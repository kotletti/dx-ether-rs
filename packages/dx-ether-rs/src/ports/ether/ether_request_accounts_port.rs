pub trait EtherRequestAccountsPort: Send + Sync {
    type Output;
    type Error;

    fn eth_request_accounts(&self) -> impl Future<Output = Result<Self::Output, Self::Error>>;
}
