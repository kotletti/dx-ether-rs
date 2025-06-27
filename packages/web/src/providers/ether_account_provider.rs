use dioxus::{logger::tracing, prelude::*};
use dx_ether_rs::hooks::use_ether_accounts::use_ether_accounts;

#[derive(Clone)]
pub struct EtherAccountContext(pub Signal<String>);

#[component]
pub fn EtherAccountProvider(children: Element) -> Element {
    let (accounts, accounts_error, accounts_loading) = use_ether_accounts();

    use_effect(move || {
        let accounts_error = accounts_error();
        let accounts_loading = accounts_loading();

        if !accounts_error.is_empty() && !accounts_loading {
            tracing::error!(accounts_error);
        }
    });

    if accounts_loading() {
        return rsx! {
            h1 { "Loading..." }
        };
    }

    let account = match accounts().first() {
        Some(r) => r.to_string(),
        _ => {
            return rsx! {
                h1 { "Ether account not found." }
            };
        }
    };

    use_context_provider(|| Signal::new(EtherAccountContext(Signal::new(account))));

    rsx! {
        {children}
    }
}
