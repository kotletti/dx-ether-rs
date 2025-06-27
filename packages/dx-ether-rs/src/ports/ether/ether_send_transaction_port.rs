pub trait EtherSendTransactionPort {
    type Output;
    type Error;

    fn eth_send_transaction(
        &self,
        from: &str,
        to: &str,
        value: Option<String>,
        calldata: Option<String>,
        gas: Option<String>,
        gas_price: Option<String>,
    ) -> impl Future<Output = Result<Self::Output, Self::Error>>;
}
