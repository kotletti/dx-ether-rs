use dioxus::prelude::*;

use crate::{
    adapters::ether_adapter::EtherAdapter,
    ports::ether::ether_get_balance_port::EtherGetBalancerPort,
};

pub fn use_ether_get_balance(
    address: &str,
    block: Option<String>,
    activated: Option<Signal<bool>>,
) -> (Signal<String>, Signal<String>, Signal<bool>) {
    let mut data = use_signal(String::new);
    let mut error = use_signal(|| String::new());
    let mut loading = use_signal(|| false);

    let address = address.to_string();
    let block = block.unwrap_or("latest".to_string());
    let activated = activated.unwrap_or(Signal::new(false));

    use_effect(move || {
        let activated = activated();

        if activated {
            loading.set(true);

            let address = address.to_string();
            let block = block.to_string();

            spawn(async move {
                match EtherAdapter::new()
                    .eth_get_balance(&address, Some(block))
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
