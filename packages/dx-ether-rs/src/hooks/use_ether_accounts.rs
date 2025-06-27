use dioxus::prelude::*;

use crate::{
    adapters::ether_adapter::EtherAdapter, ports::ether::ether_accounts_port::EtherAccountsPort,
};

pub fn use_ether_accounts() -> (Signal<Vec<String>>, Signal<String>, Signal<bool>) {
    let mut data = use_signal(|| Vec::<String>::new());
    let mut error = use_signal(|| String::new());
    let mut loading = use_signal(|| false);

    use_future(move || async move {
        loading.set(true);

        match EtherAdapter::new().eth_accounts().await {
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
