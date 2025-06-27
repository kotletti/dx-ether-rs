mod abi;
mod providers;

use std::str::FromStr;

use crate::{
    abi::abi::WETH_ABI,
    providers::{
        ether_account_provider::{EtherAccountContext, EtherAccountProvider},
        ether_chain_id_provider::{EtherChainIdContext, EtherChainIdProvider},
    },
};

use alloy_core::{
    hex,
    primitives::{Address, U256},
};
use alloy_sol_types::SolCall;
use dioxus::{logger::tracing, prelude::*};
use dx_ether_rs::{
    adapters::ether_hash_adapter::EtherHashAdapter,
    constants::scripts_constants::DX_EVAL_PROVIDER_SCRIPT,
    hooks::{
        use_ether_accounts::use_ether_accounts,
        use_ether_block_number::use_ether_block_number,
        use_ether_call::use_ether_call,
        use_ether_gas_price::use_ether_gas_price,
        use_ether_get_balance::use_ether_get_balance,
        use_ether_get_transaction_receipt::use_ether_get_transaction_receipt,
        use_ether_initialize::use_ether_initialize,
        use_ether_personal_sign::use_ether_personal_sign,
        use_ether_request_accounts::use_ether_request_accounts,
        use_ether_send_transaction::{UseEtherSendTransaction, use_ether_send_transaction},
        use_ether_switch_chain_id::use_ether_switch_chain_id,
    },
    ports::ether_hash_port::EtherHashPort,
};

#[component]
fn SendSmartContractTransaction() -> Element {
    let weth_erc20_address = "0x7b79995e5f793a07bc00c21412e50ecae098e7f9";

    let account = use_context::<Signal<EtherAccountContext>>();

    let mut activated = use_signal(|| false);
    let mut amount = use_signal(|| String::from("0.00001"));

    let address = Address::from_str(&account().0()).unwrap();

    let amount_wei = U256::from(amount().parse::<f64>().unwrap() * 10_f64.powi(18));

    let approve_calldata = WETH_ABI::approveCall {
        guy: address,
        wad: amount_wei,
    }
    .abi_encode();

    let approve_calldata_hex = EtherHashAdapter::new()
        .encode_to_string(&approve_calldata)
        .unwrap();

    let (trx_hash, trx_error, trx_loading) = use_ether_send_transaction(&UseEtherSendTransaction {
        from: account().0(),
        to: weth_erc20_address.to_string(),
        value: None,
        calldata: Some(approve_calldata_hex),
        gas: None,
        gas_price: None,
        activated: Some(activated),
    });

    use_effect(move || {
        let trx_error = trx_error();
        let trx_loading = trx_loading();

        if !trx_error.is_empty() && !trx_loading {
            tracing::error!(trx_error);
        }
    });

    let send_transaction_callback = use_callback(move |_: ()| {
        activated.set(!activated());
    });

    rsx! {
        div {
            "Wrap ether: "
            div {
                "Hash: "
                span { {trx_hash} }
            }
            div {
                input { value: amount(), oninput: move |e| amount.set(e.value()) }
            }
            div {
                button { onclick: move |_| send_transaction_callback(()), "Wrap" }
            }
        }
    }
}

#[component]
fn SendTransferTransaction() -> Element {
    let account = use_context::<Signal<EtherAccountContext>>();

    let mut activated = use_signal(|| false);
    let mut recipient = use_signal(|| String::from("0x..."));

    let (trx_hash, trx_error, trx_loading) = use_ether_send_transaction(&UseEtherSendTransaction {
        from: account().0(),
        to: recipient(),
        value: Some("100000000000000".to_string()), // 0.0001 ETH;
        calldata: None,
        gas: None,
        gas_price: None,
        activated: Some(activated),
    });

    use_effect(move || {
        let trx_error = trx_error();
        let trx_loading = trx_loading();

        if !trx_error.is_empty() && !trx_loading {
            tracing::error!(trx_error);
        }
    });

    let send_transaction_callback = use_callback(move |_: ()| {
        activated.set(!activated());
    });

    rsx! {
        div {
            "Send transfer transaction"
            div {
                "Hash: "
                span { {trx_hash} }
            }
            div {
                input {
                    value: recipient(),
                    oninput: move |e| recipient.set(e.value()),
                }
            }
            div {
                button { onclick: move |_| send_transaction_callback(()), "Send" }
            }
        }
    }
}

#[component]
fn TransactionReceiptView() -> Element {
    // WETH ERC20 deposit transaction hash Sepolia;
    let transaction_hash = "0x98610b3065f20be137e2d4b8ba33b1ceb8ae02689f08ff2c5ffd7bac74e9c434";

    let (receipt, receipt_error, receipt_loading) =
        use_ether_get_transaction_receipt(transaction_hash);

    use_effect(move || {
        let receipt_error = receipt_error();
        let receipt_loading = receipt_loading();

        if !receipt_error.is_empty() && !receipt_loading {
            tracing::error!(receipt_error);
        }
    });

    if receipt_loading() {
        return rsx! {
            div { "Transaction receipt: LOADING..." }
        };
    }

    let receipt = receipt.read();

    rsx! {
        div {
            "Transaction receipt: "
            ul {
                li { {format!("Blob gas price: {:?}", receipt.blob_gas_price)} }
                li { {format!("Blob gas used: {:?}", receipt.blob_gas_used)} }
                li { {format!("Block hash: {}", receipt.block_hash)} }
                li { {format!("Block number: {}", receipt.block_number)} }
                li { {format!("Contract address: {:?}", receipt.contract_address)} }
                li { {format!("Cumulative gas used: {}", receipt.cumulative_gas_used)} }
                li { {format!("Effective gas price: {}", receipt.effective_gas_price)} }
                li { {format!("From: {}", receipt.from)} }
                li { {format!("To: {}", receipt.to)} }
                li { {format!("Gas used: {}", receipt.gas_used)} }
                li { {format!("Logs bloom: {}", receipt.logs_bloom)} }
                li { {format!("Status: {}", receipt.status)} }
                li { {format!("Hash: {}", receipt.transaction_hash)} }
                li { {format!("Index: {}", receipt.transaction_index)} }
            }
        }
    }
}

#[component]
fn InteractContract() -> Element {
    // WETH ERC20 Sepolia;
    let weth_erc20_address = "0x7b79995e5f793a07bc00c21412e50ecae098e7f9";
    let calldata = WETH_ABI::nameCall {}.abi_encode();
    let hash_calldata = format!("0x{}", hex::encode(calldata));

    let mut name = use_signal(String::new);

    let (call, call_error, call_loading) = use_ether_call(
        weth_erc20_address,
        &hash_calldata,
        None,
        Some(Signal::new(true)),
    );

    use_effect(move || {
        let call = call();
        let call_error = call_error();
        let call_loading = call_loading();

        if !call_error.is_empty() && !call_loading {
            tracing::error!(call_error);
        }

        if !call.is_empty() {
            let name_bytes = EtherHashAdapter::new()
                .decode_to_string(&call)
                .unwrap_or(String::new());

            if name_bytes.is_empty() {
                return;
            }

            let decoded = WETH_ABI::nameCall::abi_decode_returns(&name_bytes.as_bytes())
                .unwrap_or(String::new());

            if decoded.is_empty() {
                return;
            }

            name.set(decoded);
        }
    });

    rsx! {
        div {
            "ERC20 call name(): "
            {
                match name().is_empty() {
                    true => rsx! {
                        span { "name is empty" }
                    },
                    false => rsx! {
                        span { {name} }
                    },
                }
            }
        }
    }
}

#[component]
fn GasPrice() -> Element {
    let (gas_price, gas_price_error, gas_price_loading) = use_ether_gas_price();

    use_effect(move || {
        let gas_price_error = gas_price_error();
        let gas_price_loading = gas_price_loading();

        if !gas_price_error.is_empty() && !gas_price_loading {
            tracing::error!(gas_price_error);
        }
    });

    match gas_price().is_empty() {
        true => rsx! {},
        false => rsx! {
            div {
                "Gas Price: "
                span { {gas_price} }
            }
        },
    }
}

#[component]
fn BlockNumber() -> Element {
    let (block_number, block_number_error, block_number_loading) = use_ether_block_number();

    use_effect(move || {
        let block_number_error = block_number_error();
        let block_number_loading = block_number_loading();

        if !block_number_error.is_empty() && !block_number_loading {
            tracing::error!(block_number_error);
        }
    });

    match block_number().is_empty() {
        true => rsx! {},
        false => rsx! {
            div {
                "Block number: "
                span { {block_number} }
            }
        },
    }
}

#[component]
fn Balance() -> Element {
    let account = use_context::<Signal<EtherAccountContext>>();

    let (balance, balance_error, balance_loading) =
        use_ether_get_balance(&account().0(), None, Some(Signal::new(true)));

    use_effect(move || {
        let balance_error = balance_error();
        let balance_loading = balance_loading();

        if !balance_error.is_empty() && !balance_loading {
            tracing::error!(balance_error);
        }
    });

    match balance().is_empty() {
        true => rsx! {},
        false => rsx! {
            div {
                "Balance: "
                span { {balance} }
            }
        },
    }
}

#[component]
fn SwitchWalletChainId() -> Element {
    let chain_id = use_context::<Signal<EtherChainIdContext>>();

    let mut activated = use_signal(|| false);
    let mut input_chain_id = use_signal(|| String::from("11155111"));

    let (chain_id_error, chain_id_loading) =
        use_ether_switch_chain_id(&input_chain_id(), Some(activated));

    use_effect(move || {
        let chain_id_error = chain_id_error();
        let chain_id_loading = chain_id_loading();

        if !chain_id_error.is_empty() && !chain_id_loading {
            tracing::error!(chain_id_error);
        }
    });

    let switch = use_callback(move |_: ()| {
        activated.set(!activated());
        chain_id().0.set(input_chain_id());
    });

    rsx! {
        div {
            "Switch wallet chain id"
            div {
                input {
                    value: input_chain_id(),
                    oninput: move |e| {
                        input_chain_id.set(e.value());
                    },
                }
            }
            div {
                button { onclick: move |_| switch(()), "Switch" }
            }
        }
    }
}

#[component]
fn ChainId() -> Element {
    let chain_id = use_context::<Signal<EtherChainIdContext>>();
    let chain_id = chain_id().0();

    rsx! {
        div {
            "Chain id: "
            span { {chain_id} }
        }
    }
}

#[component]
fn Accounts() -> Element {
    let (accounts, accounts_error, accounts_loading) = use_ether_accounts();

    use_effect(move || {
        let accounts_error = accounts_error();
        let accounts_loading = accounts_loading();

        if !accounts_error.is_empty() && !accounts_loading {
            tracing::error!(accounts_error);
        }
    });

    let items = accounts();

    match items.is_empty() {
        true => rsx! {},
        false => rsx! {
            div {
                "Accounts: "
                {items.iter().map(|i| rsx! {
                    span { {i.clone()} }
                })}
            }
        },
    }
}

#[component]
fn PersonalSign() -> Element {
    let mut activated = use_signal(|| false);
    let (signature, signature_error, signature_loading) = use_ether_personal_sign(
        "Hello",
        "0x57ca7f8e277696f79f913d853E809B18805807Cc",
        Some(activated),
    );

    use_effect(move || {
        let signature_error = signature_error();
        let signature_loading = signature_loading();

        if !signature_error.is_empty() && !signature_loading {
            tracing::error!(signature_error);
        }
    });

    let request = use_callback(move |_: ()| {
        activated.set(!activated());
    });

    rsx! {
        div {
            button { onclick: move |_| request(()), "Personal Sign" }
            div {
                "Signature: "
                span { {signature} }
            }
        }
    }
}

#[component]
fn ConnectWallet() -> Element {
    let mut activated = use_signal(|| false);
    let (_, accounts_error, accounts_loading) = use_ether_request_accounts(Some(activated));

    use_effect(move || {
        let accounts_error = accounts_error();
        let accounts_loading = accounts_loading();

        if !accounts_error.is_empty() && !accounts_loading {
            tracing::error!(accounts_error);
        }
    });

    let connect_wallet = use_callback(move |_: ()| {
        activated.set(!activated());
    });

    rsx! {
        div {
            button { onclick: move |_| connect_wallet(()), "Connect Wallet" }
        }
    }
}

#[component]
fn Main() -> Element {
    let (ether_error, ether_loading) = use_ether_initialize();

    use_effect(move || {
        let err = ether_error.read();

        if !err.is_empty() {
            tracing::error!("Ether initialize error: {}", err.to_string());
        }
    });

    if ether_loading() {
        return rsx! {
            h1 { "Loading ..." }
        };
    }

    rsx! {
        div {
            ConnectWallet {}
            PersonalSign {}
            Accounts {}
            EtherAccountProvider { Balance {} }
            InteractContract {}
            EtherChainIdProvider {
                ChainId {}
                SwitchWalletChainId {}
            }
            BlockNumber {}
            GasPrice {}
            TransactionReceiptView {}
            EtherAccountProvider {
                SendTransferTransaction {}
                SendSmartContractTransaction {}
            }
        }
    }
}

#[component]
fn App() -> Element {
    rsx! {
        document::Script { src: DX_EVAL_PROVIDER_SCRIPT }

        Main {}
    }
}

fn main() {
    dioxus::launch(App);
}
