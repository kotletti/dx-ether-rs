/** @format */

function wrap(
  method = '',
  success = false,
  data = undefined,
  error = {}
) {
  if (!method && !method.length) {
    method = 'unknown';
  }

  if (!Object.keys(error).length) {
    error = undefined;
  }

  return {
    method,
    success,
    data,
    error,
  };
}

async function eth_accounts() {
  const method = 'eth_accounts';

  try {
    const accounts = await window.ethereum.request({
      method,
    });

    return wrap(method, true, accounts, undefined);
  } catch (error) {
    return wrap(method, false, undefined, error);
  }
}

async function eth_requestAccounts() {
  const method = 'eth_requestAccounts';

  try {
    const accounts = await window.ethereum.request({
      method,
    });

    return wrap(method, true, accounts, undefined);
  } catch (error) {
    return wrap(method, false, undefined, error);
  }
}

async function eth_gasPrice() {
  const method = 'eth_gasPrice';

  try {
    const gasPrice = await window.ethereum.request({
      method,
    });

    return wrap(method, true, gasPrice, undefined);
  } catch (error) {
    return wrap(method, false, undefined, error);
  }
}

async function eth_blockNumber() {
  const method = 'eth_blockNumber';

  try {
    const blockNumber = await window.ethereum.request({
      method,
    });

    return wrap(method, true, blockNumber, undefined);
  } catch (error) {
    return wrap(method, false, undefined, error);
  }
}

async function eth_chainId() {
  const method = 'eth_chainId';

  try {
    const chainId = await window.ethereum.request({
      method,
    });

    return wrap(method, true, chainId, undefined);
  } catch (error) {
    return wrap(method, false, undefined, error);
  }
}

async function eth_getBalance(
  address = '',
  block = 'latest'
) {
  const method = 'eth_getBalance';

  try {
    const balance = await window.ethereum.request({
      method,
      params: [address, block],
    });

    return wrap(method, true, balance, undefined);
  } catch (error) {
    return wrap(method, false, undefined, error);
  }
}

async function eth_call(
  to = '',
  data = '',
  block = 'latest'
) {
  const method = 'eth_call';

  try {
    const calldataReturn = await window.ethereum.request({
      method,
      params: [{ to, data }, block],
    });

    return wrap(method, true, calldataReturn, undefined);
  } catch (error) {
    return wrap(method, false, undefined, error);
  }
}

async function eth_getTransactionReceipt(
  transactionHash = ''
) {
  const method = 'eth_getTransactionReceipt';

  try {
    const transactionReceipt =
      await window.ethereum.request({
        method,
        params: [transactionHash],
      });

    return wrap(
      method,
      true,
      transactionReceipt,
      undefined
    );
  } catch (error) {
    return wrap(method, false, undefined, error);
  }
}

async function eth_sendTransaction(
  from = '',
  to = '',
  value = '',
  calldata = '',
  gas = '',
  gas_price = ''
) {
  const method = 'eth_sendTransaction';

  try {
    const transactionHash = await window.ethereum.request({
      method,
      params: [
        {
          from,
          to,
          ...(value && { value }),
          ...(calldata && { data: calldata }),
          ...(gas && { gas }),
          ...(gas_price && { gas_price }),
        },
      ],
    });

    return wrap(method, true, transactionHash, undefined);
  } catch (error) {
    return wrap(method, false, undefined, error);
  }
}

async function personal_sign(message = '', address = '') {
  const method = 'personal_sign';

  try {
    const signature = await window.ethereum.request({
      method,
      params: [message, address],
    });

    return wrap(method, true, signature, undefined);
  } catch (error) {
    return wrap(method, false, undefined, error);
  }
}

async function wallet_switchEthereumChain(chainId = '') {
  const method = 'wallet_switchEthereumChain';

  try {
    await window.ethereum.request({
      method,
      params: [{ chainId }],
    });

    return wrap(method, true, undefined, undefined);
  } catch (error) {
    return wrap(method, false, undefined, error);
  }
}

window['dxEvalProvider'] = {
  wrap,
  eth_accounts,
  eth_requestAccounts,
  eth_gasPrice,
  eth_blockNumber,
  eth_chainId,
  eth_getBalance,
  eth_call,
  eth_getTransactionReceipt,
  eth_sendTransaction,
  personal_sign,
  wallet_switchEthereumChain,
};
