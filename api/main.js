const path = require('path');
const { DirectSecp256k1HdWallet } = require("@cosmjs/proto-signing");
const { stringToPath } = require("@cosmjs/crypto");
const cosmwasm = require('@cosmjs/cosmwasm-stargate');
const { GasPrice } = require('@cosmjs/cosmwasm-stargate/node_modules/@cosmjs/stargate/build');
require('dotenv').config({ path: path.resolve(__dirname, process.env.NODE_ENV ? `.env.${process.env.NODE_ENV}` : ".env") })
var express = require('express');
const bodyParser = require('body-parser');
const cors = require('cors');

const network = {
    rpc: process.env.NETWORK_RPC || "https://testnet-rpc.orai.io",
    prefix: "orai",
}
// config
const customerContractAddr = process.env.CUSTOMER_CONTRACT_ADDRESS;
const expertContractAddr = process.env.EXPERT_CONTRACT_ADDRESS;
const wallet = process.env.MNEMONIC;
const feeAmount = [{ denom: "orai", amount: "1000" }]
let finalFeeAmount = feeAmount.filter(fee => fee.amount !== '0');
if (finalFeeAmount.length === 0) finalFeeAmount = undefined;
// end config
const collectWallet = async (mnemonic) => {
    const wallet = await DirectSecp256k1HdWallet.fromMnemonic(
        mnemonic,
        {
            hdPaths: [stringToPath("m/44'/118'/0'/0/0")],
            prefix: network.prefix,
        }
    );
    return wallet;
}

const execute = async ({ mnemonic, address, handleMsg, memo, amount, gasData = undefined }) => {
    try {
        const wallet = await collectWallet(mnemonic);
        const [firstAccount] = await wallet.getAccounts();
        const client = await cosmwasm.SigningCosmWasmClient.connectWithSigner(network.rpc, wallet, { gasPrice: gasData ? GasPrice.fromString(`${gasData.gasAmount}${gasData.denom}`) : undefined, prefix: network.prefix, gasLimits: { exec: 20000000 } });
        const input = JSON.parse(handleMsg);
        console.log(input)
        const result = await client.execute(firstAccount.address, address, input, memo, amount);
        return result.transactionHash;
    } catch (error) {
        console.log("error in executing contract: ", error);
        throw error;
    }
}

const customer = async (data) => {
    const input = JSON.stringify({
        add_customer_request: data
    })
    // store the merkle root on-chain
    const txHash = await execute({ mnemonic: wallet, address: customerContractAddr, handleMsg: input, gasData: { gasAmount: "0", denom: "orai" }, amount: finalFeeAmount });
    console.log("execute result: ", "https://testnet.scan.orai.io/txs/" + txHash);
    return txHash;

}

const expert = async (data) => {
    const input = JSON.stringify({
        add_expert_response: data
    })
    // store the merkle root on-chain
    const txHash = await execute({ mnemonic: wallet, address: expertContractAddr, handleMsg: input, gasData: { gasAmount: "0", denom: "orai" }, amount: finalFeeAmount });
    console.log("execute result: ", "https://testnet.scan.orai.io/txs/" + txHash);
    return txHash;
}

const app = express();
const port = 3000;

app.use(cors());

app.use(bodyParser.json());


app.post('/addCustomerRequest', async (req, res) => {
    const data = req.body;
    const txhash = await customer(data)
    res.send({ txHash: txhash });
});

app.post('/addExpertResponse', async (req, res) => {
    const data = req.body;
    const txhash = await expert(data)
    res.send({ txHash: txhash });
});

app.listen(port, () => console.log(`Hello world app listening on port ${port}!`));