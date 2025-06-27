use dioxus::prelude::*;

use crate::{
    adapters::ether_adapter::EtherAdapter,
    ports::ether::ether_send_transaction_port::EtherSendTransactionPort,
};

#[derive(Clone)]
pub struct UseEtherSendTransaction {
    pub from: String,
    pub to: String,
    pub value: Option<String>,
    pub calldata: Option<String>,
    pub gas: Option<String>,
    pub gas_price: Option<String>,
    pub activated: Option<Signal<bool>>,
}

pub fn use_ether_send_transaction(
    payload: &UseEtherSendTransaction,
) -> (Signal<String>, Signal<String>, Signal<bool>) {
    let mut data = use_signal(String::new);
    let mut error = use_signal(|| String::new());
    let mut loading = use_signal(|| false);

    let payload = payload.clone();
    let activated = payload.activated.unwrap_or(Signal::new(false));

    use_effect(move || {
        let activated = activated();

        if activated {
            loading.set(true);

            let payload = payload.clone();

            spawn(async move {
                match EtherAdapter::new()
                    .eth_send_transaction(
                        &payload.from,
                        &payload.to,
                        payload.value,
                        payload.calldata,
                        payload.gas,
                        payload.gas_price,
                    )
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
        }
    });

    (data, error, loading)
}
