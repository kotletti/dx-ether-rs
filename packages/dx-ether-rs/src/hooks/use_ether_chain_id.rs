use dioxus::prelude::*;

use crate::{
    adapters::ether_adapter::EtherAdapter, ports::ether::ether_chain_id_port::EtherChainIdPort,
};

pub fn use_ether_chain_id() -> (Signal<String>, Signal<String>, Signal<bool>) {
    let mut data = use_signal(String::new);
    let mut error = use_signal(|| String::new());
    let mut loading = use_signal(|| false);

    use_future(move || async move {
        loading.set(true);

        match EtherAdapter::new().eth_chain_id().await {
            Ok(r) => {
                data.set(r);
                loading.set(false);
            }
            Err(e) => {
                error.set(e.to_string());
                loading.set(false);
            }
        };
    });

    (data, error, loading)
}
