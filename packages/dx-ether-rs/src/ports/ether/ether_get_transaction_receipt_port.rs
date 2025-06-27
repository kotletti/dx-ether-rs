pub trait EtherGetTransactionReceiptPort: Send + Sync {
    type Output;
    type Error;

    fn eth_get_transaction_receipt(
        &self,
        transaction_hash: &str,
    ) -> impl Future<Output = Result<Self::Output, Self::Error>>;
}
