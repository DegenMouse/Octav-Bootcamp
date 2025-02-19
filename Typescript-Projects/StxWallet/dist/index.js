"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const auth_1 = require("./auth");
const helper_1 = require("./helper");
const myWalletAccountInfo = {
    stxAdressTestnet: "",
    stxAdressMainnet: "",
    stxPrivateKey: "",
};
function printLogInMenu() {
    console.log("\n=== Welcome to the Stacks Wallet === ");
    console.log("[1] Authenticate secret key");
    console.log("[2] Create new wallet");
    console.log("[0] Exit");
}
function printLoggedInMenu() {
    console.log("\n=== Your wallet 💳 === ");
    console.log("[1] Get account balance");
    console.log("[2] Make a transfer");
    console.log("[0] Log out");
}
async function menu() {
    while (true) {
        var logger = auth_1.authenticated;
        if (!logger) {
            printLogInMenu();
            const option = await (0, helper_1.question)("Option: ");
            switch (option) {
                case '1':
                    {
                        const result = await (0, auth_1.loginToWallet)();
                        if (result != undefined) {
                            myWalletAccountInfo.stxAdressTestnet = result.testnetAdress;
                            myWalletAccountInfo.stxAdressMainnet = result.mainnetAdress;
                            myWalletAccountInfo.stxPrivateKey = result.stxPrivateKey;
                        }
                    }
                    break;
                case '2':
                    {
                        const result = await (0, auth_1.createWallet)();
                        myWalletAccountInfo.stxAdressTestnet = result.testnetAdress;
                        myWalletAccountInfo.stxAdressMainnet = result.mainnetAdress;
                        // myWalletAccountInfo.stxPrivateKey = result.stxPrivateKey;
                    }
                    break;
                case '0':
                    console.log("Goodbye!");
                    helper_1.rl.close();
                    return;
                default:
                    console.log("Invalid option. Please try again.");
            }
        }
        else {
            printLoggedInMenu();
            const option = await (0, helper_1.question)("Option: ");
            switch (option) {
                case '1':
                    await (0, auth_1.getBalance)(myWalletAccountInfo.stxAdressMainnet);
                    break;
                case '2':
                    await (0, auth_1.stxTransfer)(myWalletAccountInfo.stxPrivateKey);
                    break;
                case '0':
                    console.log("You were broke anyway...");
                    (0, auth_1.logout)();
                    logger = false;
                    break;
                default:
                    console.log("Invalid option. Please try again.");
            }
        }
    }
}
menu();
