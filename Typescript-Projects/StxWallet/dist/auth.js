"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || (function () {
    var ownKeys = function(o) {
        ownKeys = Object.getOwnPropertyNames || function (o) {
            var ar = [];
            for (var k in o) if (Object.prototype.hasOwnProperty.call(o, k)) ar[ar.length] = k;
            return ar;
        };
        return ownKeys(o);
    };
    return function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k = ownKeys(mod), i = 0; i < k.length; i++) if (k[i] !== "default") __createBinding(result, mod, k[i]);
        __setModuleDefault(result, mod);
        return result;
    };
})();
Object.defineProperty(exports, "__esModule", { value: true });
exports.authenticated = void 0;
exports.createWallet = createWallet;
exports.loginToWallet = loginToWallet;
exports.logout = logout;
exports.getBalance = getBalance;
exports.stxTransfer = stxTransfer;
const transactions_1 = require("@stacks/transactions");
const wallet_sdk_1 = require("@stacks/wallet-sdk");
const helper_1 = require("./helper");
const bip39 = __importStar(require("bip39"));
exports.authenticated = false;
async function createWallet() {
    const seed = (0, wallet_sdk_1.randomSeedPhrase)();
    const wallet = await (0, wallet_sdk_1.generateWallet)({
        secretKey: seed,
        password: 'secret',
    });
    const testnetAdress = (0, transactions_1.getAddressFromPrivateKey)(wallet.accounts[0].stxPrivateKey, 'testnet');
    const mainnetAdress = (0, transactions_1.getAddressFromPrivateKey)(wallet.accounts[0].stxPrivateKey, 'mainnet');
    console.log('Testnet adress: ', testnetAdress);
    console.log('Mainnet adress: ', mainnetAdress);
    exports.authenticated = true;
    return { testnetAdress, mainnetAdress };
}
async function loginToWallet() {
    const seed = await (0, helper_1.question)("Seed: ");
    const valid = bip39.validateMnemonic(seed);
    if (valid) {
        const wallet = await (0, wallet_sdk_1.generateWallet)({
            secretKey: seed,
            password: 'secret',
        });
        const testnetAdress = (0, transactions_1.getAddressFromPrivateKey)(wallet.accounts[0].stxPrivateKey, 'testnet');
        const mainnetAdress = (0, transactions_1.getAddressFromPrivateKey)(wallet.accounts[0].stxPrivateKey, 'mainnet');
        console.log('Testnet adress: ', testnetAdress);
        console.log('Mainnet adress: ', mainnetAdress);
        exports.authenticated = true;
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
function logout() {
    exports.authenticated = false;
}
async function getBalance(adress) {
    const response = await fetch('https://api.hiro.so/extended/v1/address/' + adress + '/stx', {
        method: "GET"
    });
    const responseJson = await response.json();
    console.log(parseInt(responseJson.balance) / 1000000, 'stx');
}
async function stxTransfer(senderKey) {
    console.log("[1] Mainnet");
    console.log("[2] Testnet");
    const option = parseInt(await (0, helper_1.question)("Option: "), 10);
    const recipient = await (0, helper_1.question)("Recipient address: ");
    const amount = parseInt(await (0, helper_1.question)("Amount (in STX): ")) * 1000000;
    if (option == 1) {
        const txOptions = {
            recipient: recipient,
            amount: amount,
            memo: 'test memo',
            senderKey: senderKey,
            network: 'mainnet',
        };
        const transaction = await (0, transactions_1.makeSTXTokenTransfer)(txOptions);
        // to see the raw serialized tx
        const serializedTx = transaction.serialize(); // hex string
        // broadcast to the network
        const response = await (0, transactions_1.broadcastTransaction)({ transaction, network: 'mainnet' });
        console.log(response.txid);
    }
}
