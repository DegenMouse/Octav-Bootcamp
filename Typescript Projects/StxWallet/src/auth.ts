import { broadcastTransaction, getAddressFromPrivateKey, makeSTXTokenTransfer } from "@stacks/transactions";
import { randomSeedPhrase, generateWallet } from "@stacks/wallet-sdk";
import { question } from "./helper";
import * as bip39 from "bip39";

export var authenticated = false;

export async function createWallet(): Promise<{testnetAdress: string, mainnetAdress: string}> {
    const seed = randomSeedPhrase();
    const wallet = await generateWallet({
        secretKey: seed,
        password: 'secret',
      });
    const testnetAdress = getAddressFromPrivateKey(wallet.accounts[0].stxPrivateKey,'testnet');
    const mainnetAdress = getAddressFromPrivateKey(wallet.accounts[0].stxPrivateKey,'mainnet');
    console.log('Testnet adress: ',testnetAdress);
    console.log('Mainnet adress: ',mainnetAdress);

    authenticated = true;
    return {testnetAdress, mainnetAdress};
}

export async function loginToWallet(): Promise<{testnetAdress: string, mainnetAdress: string, stxPrivateKey: string} | undefined> {
    const seed = await question("Seed: ");
    const valid = bip39.validateMnemonic(seed)
    if (valid) {
      const wallet = await generateWallet({
        secretKey: seed,
        password: 'secret',
      });
      const testnetAdress = getAddressFromPrivateKey(wallet.accounts[0].stxPrivateKey,'testnet');
      const mainnetAdress = getAddressFromPrivateKey(wallet.accounts[0].stxPrivateKey,'mainnet');
      console.log('Testnet adress: ',testnetAdress);
      console.log('Mainnet adress: ',mainnetAdress); 
      
      authenticated = true;
      return {
        testnetAdress, 
        mainnetAdress, 
        stxPrivateKey: wallet.accounts[0].stxPrivateKey
      };
    }
    else {
      console.log("Invalid seed phrase");
      return undefined;
    }
}

export function logout() {
    authenticated = false;
}

export async function getBalance(adress: string) {
  const response = await fetch('https://api.hiro.so/extended/v1/address/' + adress + '/stx', {
    method: "GET"
  });
  const responseJson = await response.json();
  console.log(parseInt(responseJson.balance)/1000000, 'stx');
}

export async function stxTransfer(senderKey: string) {
  console.log("[1] Mainnet");
  console.log("[2] Testnet");
  const option = parseInt(await question("Option: "),10);
  const recipient = await question("Recipient address: ");
  const amount = parseInt(await question("Amount (in STX): ")) * 1000000;
  
  if (option == 1){
    const txOptions = {
      recipient: recipient,
      amount: amount,
      memo: 'test memo',
      senderKey: senderKey,
      network: 'mainnet' as const,
    };

    const transaction = await makeSTXTokenTransfer(txOptions);

    // to see the raw serialized tx
    const serializedTx = transaction.serialize(); // hex string

    // broadcast to the network
    const response = await broadcastTransaction({ transaction, network: 'mainnet' });
    console.log(response.txid);
  }
  
}

