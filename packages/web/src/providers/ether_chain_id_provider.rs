use dioxus::{logger::tracing, prelude::*};
use dx_ether_rs::hooks::use_ether_chain_id::use_ether_chain_id;

#[derive(Clone)]
pub struct EtherChainIdContext(pub Signal<String>);

#[component]
pub fn EtherChainIdProvider(children: Element) -> Element {
    let (chain_id, chain_id_error, chain_id_loading) = use_ether_chain_id();

    use_effect(move || {
        let chain_id_error = chain_id_error();
        let chain_id_loading = chain_id_loading();

        if !chain_id_error.is_empty() && !chain_id_loading {
            tracing::error!(chain_id_error);
        }
    });

    if chain_id_loading() {
        return rsx! {
            h1 { "Loading..." }
        };
    }

    if chain_id().is_empty() {
        return rsx! {
            h1 { "Chain id is empty." }
        };
    }

    use_context_provider(|| Signal::new(EtherChainIdContext(chain_id)));

    rsx! {
        {children}
    }
}
