use dioxus::document;
use gloo_timers::future::sleep;
use std::time::Duration;

use crate::{
    adapters::ether_hash_adapter::EtherHashAdapter,
    errors::{error::Error, ether_error::EtherError},
    ports::{
        ether::{
            ether_accounts_port::EtherAccountsPort, ether_block_number_port::EtherBlockNumberPort,
            ether_call_port::EtherCallPort, ether_chain_id_port::EtherChainIdPort,
            ether_error_port::EtherErrorPort, ether_gas_price_port::EtherGasPricePort,
            ether_get_balance_port::EtherGetBalancerPort,
            ether_get_transaction_receipt_port::EtherGetTransactionReceiptPort,
            ether_request_accounts_port::EtherRequestAccountsPort,
            ether_send_transaction_port::EtherSendTransactionPort,
            personal_sign_port::PersonalSignPort,
            wallet_switch_ethereum_chain_port::WalletSwitchEthereumChainPort,
        },
        ether_hash_port::EtherHashPort,
    },
};

#[derive(serde::Serialize, serde::Deserialize)]
struct EtherWrapper<D: Default, E = Error> {
    pub method: String,
    pub success: bool,
    pub data: Option<D>,
    pub error: Option<E>,
}

pub struct EtherAdapter {
    ether_hash: EtherHashAdapter,
    key: String,
}

impl EtherAdapter {
    pub fn new() -> Self {
        Self {
            ether_hash: EtherHashAdapter::new(),
            key: String::from("window.dxEvalProvider"),
        }
    }

    pub async fn wait_js_scripts(&self) -> Result<(), Error> {
        loop {
            let mut eval = document::eval(&format!("await dioxus.send(typeof {})", self.key));

            let recv = eval.recv::<String>().await.unwrap_or_default();

            if recv == "object" {
                break;
            }

            sleep(Duration::from_millis(25)).await;
        }

        Ok(())
    }

    fn unwrap<D: Default, E: EtherErrorPort>(
        &self,
        target: EtherWrapper<D, E>,
    ) -> Result<D, Error> {
        if !target.success {
            let error = match target.error {
                Some(r) => EtherError {
                    message: r.message(),
                    stack: r.stack(),
                },
                _ => EtherError::new_unknown(),
            };

            return Err(format!("[{}]: {}", target.method, error.message).into());
        }

        Ok(target.data.unwrap_or_default())
    }
}

/*
    Ethereum RPC method: eth_RequestAccounts -> Vec<String>;
*/
impl EtherRequestAccountsPort for EtherAdapter {
    type Output = Vec<String>;
    type Error = Error;

    async fn eth_request_accounts(&self) -> Result<Self::Output, Self::Error> {
        let mut eval = document::eval(&format!(
            "await dioxus.send({}.eth_requestAccounts())",
            self.key
        ));

        let recv = eval
            .recv::<EtherWrapper<Self::Output, EtherError>>()
            .await?;

        let data = self.unwrap(recv)?;

        Ok(data.iter().map(|i| i.to_lowercase().to_string()).collect())
    }
}

/*
    Ethereum RPC method: eth_accounts -> Vec<String>;
*/
impl EtherAccountsPort for EtherAdapter {
    type Output = Vec<String>;
    type Error = Error;

    async fn eth_accounts(&self) -> Result<Self::Output, Self::Error> {
        let mut eval = document::eval(&format!("await dioxus.send({}.eth_accounts())", self.key));

        let recv = eval
            .recv::<EtherWrapper<Self::Output, EtherError>>()
            .await?;

        let data = self.unwrap(recv)?;

        Ok(data.iter().map(|i| i.to_lowercase().to_string()).collect())
    }
}

/*
    Ethereum RPC method: eth_chainId -> String;
*/
impl EtherChainIdPort for EtherAdapter {
    type Output = String;

    type Error = Error;

    async fn eth_chain_id(&self) -> Result<Self::Output, Self::Error> {
        let mut eval = document::eval(&format!("await dioxus.send({}.eth_chainId())", self.key));

        let recv = eval
            .recv::<EtherWrapper<Self::Output, EtherError>>()
            .await?;

        let data = self.unwrap(recv)?;

        self.ether_hash.decode_to_number(&data)
    }
}

/*
    Metamask wallet method: wallet_switchEthereumChain -> ();
*/
impl WalletSwitchEthereumChainPort for EtherAdapter {
    type Output = ();

    type Error = Error;

    async fn wallet_switch_ethereum_chain(
        &self,
        chain_id: &str,
    ) -> Result<Self::Output, Self::Error> {
        let chain_id = self.ether_hash.encode_to_number(chain_id)?;

        let mut eval = document::eval(&format!(
            "await dioxus.send({}.wallet_switchEthereumChain('{}'))",
            self.key, chain_id
        ));

        let recv = eval
            .recv::<EtherWrapper<Self::Output, EtherError>>()
            .await?;

        self.unwrap(recv)
    }
}

/*
    Ethereum RPC method: eth_blockNumber -> String;
*/
impl EtherBlockNumberPort for EtherAdapter {
    type Output = String;

    type Error = Error;

    async fn eth_block_number(&self) -> Result<Self::Output, Self::Error> {
        let mut eval = document::eval(&format!(
            "await dioxus.send({}.eth_blockNumber())",
            self.key
        ));

        let recv = eval
            .recv::<EtherWrapper<Self::Output, EtherError>>()
            .await?;

        let data = self.unwrap(recv)?;

        self.ether_hash.decode_to_number(&data)
    }
}

/*
    Ethereum RPC method: eth_gasPrice -> String;
*/
impl EtherGasPricePort for EtherAdapter {
    type Output = String;

    type Error = Error;

    async fn eth_gas_price(&self) -> Result<Self::Output, Self::Error> {
        let mut eval = document::eval(&format!("await dioxus.send({}.eth_gasPrice())", self.key));

        let recv = eval
            .recv::<EtherWrapper<Self::Output, EtherError>>()
            .await?;

        let data = self.unwrap(recv)?;

        self.ether_hash.decode_to_number(&data)
    }
}

/*
    Ethereum RPC method: personal_sign -> String;
*/
impl PersonalSignPort for EtherAdapter {
    type Output = String;

    type Error = Error;

    async fn personal_sign(
        &self,
        message: &str,
        address: &str,
    ) -> Result<Self::Output, Self::Error> {
        let mut eval = document::eval(&format!(
            "await dioxus.send({}.personal_sign('{}', '{}'))",
            self.key, message, address
        ));

        let recv = eval
            .recv::<EtherWrapper<Self::Output, EtherError>>()
            .await?;

        self.unwrap(recv)
    }
}

/*
    Ethereum RPC method: eth_getBalance -> String;
*/
impl EtherGetBalancerPort for EtherAdapter {
    type Output = String;

    type Error = Error;

    async fn eth_get_balance(
        &self,
        address: &str,
        block: Option<String>,
    ) -> Result<Self::Output, Self::Error> {
        let block = block.unwrap_or(String::from("latest"));

        let mut eval = document::eval(&format!(
            "await dioxus.send({}.eth_getBalance('{}', '{}'))",
            self.key, address, block
        ));

        let recv = eval
            .recv::<EtherWrapper<Self::Output, EtherError>>()
            .await?;

        let data = self.unwrap(recv)?;

        self.ether_hash.decode_to_number(&data)
    }
}

/*
    Ethereum RPC method: eth_call -> String;
*/
impl EtherCallPort for EtherAdapter {
    type Output = String;

    type Error = Error;

    async fn eth_call(
        &self,
        address: &str,
        calldata: &str,
        block: Option<String>,
    ) -> Result<Self::Output, Self::Error> {
        let block = block.unwrap_or(String::from("latest"));

        let mut eval = document::eval(&format!(
            "await dioxus.send({}.eth_call('{address}', '{calldata}', '{block}'))",
            self.key
        ));

        let recv = eval
            .recv::<EtherWrapper<Self::Output, EtherError>>()
            .await?;

        self.unwrap(recv)
    }
}

/*
    Ethereum RPC method: eth_getTransactionReceipt -> Struct;
*/
#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct EtherTransactionReceipt {
    #[serde(rename(serialize = "block_hash", deserialize = "blockHash"))]
    pub block_hash: String,
    #[serde(rename(serialize = "block_number", deserialize = "blockNumber"))]
    pub block_number: String,
    #[serde(rename(serialize = "contract_address", deserialize = "contractAddress"))]
    pub contract_address: Option<String>,
    #[serde(rename(serialize = "cumulative_gas_used", deserialize = "cumulativeGasUsed"))]
    pub cumulative_gas_used: String,
    pub from: String,
    #[serde(rename(serialize = "gas_used", deserialize = "gasUsed"))]
    pub gas_used: String,
    #[serde(rename(serialize = "blob_gas_used", deserialize = "blobGasUsed"))]
    pub blob_gas_used: Option<String>,
    // pub logs: Vec<String>, // Coming soon;
    #[serde(rename(serialize = "effective_gas_price", deserialize = "effectiveGasPrice"))]
    pub effective_gas_price: String,
    #[serde(rename(serialize = "blob_gas_price", deserialize = "blobGasPrice"))]
    pub blob_gas_price: Option<String>,
    #[serde(rename(serialize = "logs_bloom", deserialize = "logsBloom"))]
    pub logs_bloom: String,
    pub status: String,
    pub to: String,
    #[serde(rename(serialize = "transaction_hash", deserialize = "transactionHash"))]
    pub transaction_hash: String,
    #[serde(rename(serialize = "transaction_index", deserialize = "transactionIndex"))]
    pub transaction_index: String,
}

impl EtherGetTransactionReceiptPort for EtherAdapter {
    type Output = EtherTransactionReceipt;

    type Error = Error;

    async fn eth_get_transaction_receipt(
        &self,
        transaction_hash: &str,
    ) -> Result<Self::Output, Self::Error> {
        let mut eval = document::eval(&format!(
            "await dioxus.send({}.eth_getTransactionReceipt('{transaction_hash}'))",
            self.key
        ));

        let recv = eval
            .recv::<EtherWrapper<Self::Output, EtherError>>()
            .await?;

        let receipt = self.unwrap(recv)?;

        Ok(receipt)
    }
}

/*
    Ethereum RPC method: eth_sendTransaction -> String;
*/
impl EtherSendTransactionPort for EtherAdapter {
    type Output = String;

    type Error = Error;

    async fn eth_send_transaction(
        &self,
        from: &str,
        to: &str,
        value: Option<String>,
        calldata: Option<String>,
        gas: Option<String>,
        gas_price: Option<String>,
    ) -> Result<Self::Output, Self::Error> {
        let value = value.unwrap_or(String::new());
        let calldata = calldata.unwrap_or(String::new());
        let gas = gas.unwrap_or(String::new());
        let gas_price = gas_price.unwrap_or(String::new());

        let mut eval = document::eval(&format!(
            "await dioxus.send({}.eth_sendTransaction('{from}', '{to}', '{value}', '{calldata}', '{gas}', '{gas_price}'))",
            self.key
        ));

        let recv = eval
            .recv::<EtherWrapper<Self::Output, EtherError>>()
            .await?;

        let r = self.unwrap(recv);

        Ok(r?)
    }
}
