use dioxus::{logger::tracing, prelude::*};

use crate::adapters::ether_adapter::EtherAdapter;

pub fn use_ether_initialize() -> (Signal<String>, Signal<bool>) {
    let mut error = use_signal(String::new);
    let mut loading = use_signal(|| false);

    use_future(move || async move {
        loading.set(true);

        match EtherAdapter::new().wait_js_scripts().await {
            Ok(_) => {
                loading.set(false);

                tracing::info!("Ether has initialized.");
            }
            Err(e) => {
                error.set(e.to_string());
                loading.set(false);

                tracing::error!("Ether has been failed: {}", e.to_string());
            }
        };
    });

    (error, loading)
}
