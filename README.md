# Dioxus web3 ethereum wallet integration

## ARCHIVED, MOVED TO GITLAB -> [https://gitlab.kotletti.com/kotletti/deps/dx-ether-rs](https://gitlab.kotletti.com/kotletti/deps/dx-ether-rs)

### How its work
Its bridge between JS layer and WASM.
`dioxus::document::eval` invoke JS functions and manage the responses, thats all.

### How to run demo app
1. `cargo build`
2. `dx serve -p web --platform web --port 8080`
3. goto http://localhost:8080

### Ethereum methods and hooks implemented
1. `eth_accounts` -> `use_ether_accounts`
2. `eth_blockNumber` -> `use_ether_block_number`
3. `eth_call` -> `use_ether_call`
4. `eth_chainId` -> `use_ether_chain_id`
5. `eth_gasPrice` -> `use_ether_gas_price`
6. `eth_getBalance` -> `use_ether_get_balance`
7. `eth_getTransactionReceipt` -> `use_ether_get_transaction_receipt`
8. `eth_requestAccounts` -> `use_ether_request_accounts`
9. `eth_sendTransaction` -> `use_ether_send_transaction`
10. `wallet_switchEthereumChain` -> `use_ether_switch_chain_id`
11. Non-ethereum method `use_ether_initialize` -- await loading JS scripts for eval.

### Examples

-> Connect wallet
```rust
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
```

-> Personal Sign
```rust
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
```

-> Get block number
```rust
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
```

### All examples contained in `packages/web/src/main.rs`

# Disclaimer
Its pre-alpha version, many things could be unstable but works in exampled view. Maybe later I will add the tests and more documentation.
