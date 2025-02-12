import { authenticated, createWallet, getBalance, loginToWallet, logout, stxTransfer } from "./auth";
import { question, rl } from "./helper";

interface walletAccountInfo {
    stxAdressTestnet: string,
    stxAdressMainnet: string,
    stxPrivateKey: string
}

const myWalletAccountInfo : walletAccountInfo = {
    stxAdressTestnet: "",
    stxAdressMainnet: "",
    stxPrivateKey: "",
};

function printLogInMenu() {
    console.log("\n=== Welcome to the Stacks Wallet === ");
    console.log("[1] Authenticate secret key")
    console.log("[2] Create new wallet");
    console.log("[0] Exit")
}

function printLoggedInMenu() {
    console.log("\n=== Your wallet 💳 === ");
    console.log("[1] Get account balance")
    console.log("[2] Make a transfer")
    console.log("[0] Log out")
}

async function menu(){
    while(true){
        var logger = authenticated;
        if(!logger) {
            printLogInMenu();
            const option = await question("Option: ");
            switch (option) {
                case '1':
                    {
                        const result = await loginToWallet();
                        if (result!= undefined)
                        {
                            myWalletAccountInfo.stxAdressTestnet = result.testnetAdress;
                            myWalletAccountInfo.stxAdressMainnet = result.mainnetAdress;
                            myWalletAccountInfo.stxPrivateKey = result.stxPrivateKey;
                        }
                    }
                    break;
                case '2':
                    {
                        const result = await createWallet();
                        myWalletAccountInfo.stxAdressTestnet = result.testnetAdress;
                        myWalletAccountInfo.stxAdressMainnet = result.mainnetAdress;
                        // myWalletAccountInfo.stxPrivateKey = result.stxPrivateKey;
                    }
                    break;
                case '0':
                    console.log("Goodbye!");
                    rl.close();
                    return;
                default:
                    console.log("Invalid option. Please try again.");
            }
        }
        else {
            printLoggedInMenu();
            const option = await question("Option: ");
            switch (option) {
                case '1':
                    await getBalance(myWalletAccountInfo.stxAdressMainnet);
                    break;
                case '2':
                    await stxTransfer(myWalletAccountInfo.stxPrivateKey);
                    break;
                case '0':
                    console.log("You were broke anyway...");
                    logout();
                    logger = false;
                    break;
                default:
                    console.log("Invalid option. Please try again.");
            }
        }
    }
}

menu()
