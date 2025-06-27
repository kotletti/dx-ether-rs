use dioxus::prelude::*;

use crate::{
    adapters::ether_adapter::{EtherAdapter, EtherTransactionReceipt},
    ports::ether::ether_get_transaction_receipt_port::EtherGetTransactionReceiptPort,
};

pub fn use_ether_get_transaction_receipt(
    transaction_hash: &str,
) -> (
    Signal<EtherTransactionReceipt>,
    Signal<String>,
    Signal<bool>,
) {
    let mut data = use_signal(EtherTransactionReceipt::default);
    let mut error = use_signal(|| String::new());
    let mut loading = use_signal(|| false);

    let transaction_hash = transaction_hash.to_string();

    use_effect(move || {
        let transaction_hash = transaction_hash.to_string();

        spawn(async move {
            match EtherAdapter::new()
                .eth_get_transaction_receipt(&transaction_hash)
                .await
            {
                Ok(r) => {
                    loading.set(false);
                    data.set(r);
                }
                Err(e) => {
                    error.set(e.to_string());
                    loading.set(false);
                }
            };
        });
    });

    (data, error, loading)
}
