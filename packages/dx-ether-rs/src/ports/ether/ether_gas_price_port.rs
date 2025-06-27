pub trait EtherGasPricePort: Send + Sync {
    type Output;
    type Error;

    fn eth_gas_price(&self) -> impl Future<Output = Result<Self::Output, Self::Error>>;
}
