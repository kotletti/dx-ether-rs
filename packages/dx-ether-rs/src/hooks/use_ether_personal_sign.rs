use dioxus::prelude::*;

use crate::{
    adapters::ether_adapter::EtherAdapter, ports::ether::personal_sign_port::PersonalSignPort,
};

pub fn use_ether_personal_sign(
    message: &str,
    address: &str,
    activated: Option<Signal<bool>>,
) -> (Signal<String>, Signal<String>, Signal<bool>) {
    let mut data = use_signal(String::new);
    let mut error = use_signal(|| String::new());
    let mut loading = use_signal(|| false);

    let message = message.to_string();
    let address = address.to_string();
    let activated = activated.unwrap_or(Signal::new(false));

    use_effect(move || {
        let activated = activated();
        let message = message.to_string();
        let address = address.to_string();

        spawn(async move {
            if activated {
                loading.set(true);

                match EtherAdapter::new().personal_sign(&message, &address).await {
                    Ok(r) => {
                        loading.set(false);
                        data.set(r);
                    }
                    Err(e) => {
                        error.set(e.to_string());
                        loading.set(false);
                    }
                };
            }
        });
    });

    (data, error, loading)
}
