use dioxus::prelude::*;

use crate::{
    adapters::ether_adapter::EtherAdapter,
    ports::ether::ether_request_accounts_port::EtherRequestAccountsPort,
};

pub fn use_ether_request_accounts(
    activated: Option<Signal<bool>>,
) -> (Signal<Vec<String>>, Signal<String>, Signal<bool>) {
    let mut data = use_signal(|| Vec::<String>::new());
    let mut error = use_signal(|| String::new());
    let mut loading = use_signal(|| false);

    let activated = activated.unwrap_or(Signal::new(false));

    use_effect(move || {
        if activated() {
            loading.set(true);

            spawn(async move {
                match EtherAdapter::new().eth_request_accounts().await {
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
        }
    });

    (data, error, loading)
}
