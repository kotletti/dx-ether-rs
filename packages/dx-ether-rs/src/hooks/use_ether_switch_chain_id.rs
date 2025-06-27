use dioxus::prelude::*;

use crate::{
    adapters::ether_adapter::EtherAdapter,
    ports::ether::wallet_switch_ethereum_chain_port::WalletSwitchEthereumChainPort,
};

pub fn use_ether_switch_chain_id(
    chain_id: &str,
    activated: Option<Signal<bool>>,
) -> (Signal<String>, Signal<bool>) {
    let mut error = use_signal(|| String::new());
    let mut loading = use_signal(|| false);

    let chain_id = chain_id.to_string();
    let activated = activated.unwrap_or(Signal::new(false));

    use_effect(move || {
        let activated = activated();

        if activated {
            loading.set(true);

            let chain_id = chain_id.to_string();

            spawn(async move {
                match EtherAdapter::new()
                    .wallet_switch_ethereum_chain(&chain_id)
                    .await
                {
                    Ok(_) => {
                        loading.set(false);
                    }
                    Err(e) => {
                        error.set(e.to_string());
                        loading.set(false);
                    }
                };
            });
        }
    });

    (error, loading)
}
