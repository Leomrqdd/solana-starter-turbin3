import { Keypair, PublicKey, Connection, Commitment } from "@solana/web3.js";
import { getOrCreateAssociatedTokenAccount, mintTo } from '@solana/spl-token';
import wallet from "/Users/lmqrd/.config/solana/id.json"


// Load keypair
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
console.log('Public key: ', keypair.publicKey.toBase58());

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

const decimals = 9n;
const token_decimals = 10n ** decimals;
const amount = 100n *token_decimals; // I want to mint 100 tokens

// Mint address
const mint = new PublicKey("DpZ3H6iCty6GQxT5zC84jxBSN7KnFH5oDVHcVWyEisEo");

(async () => {
    try {
        // Create an ATA
        const ata  = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            keypair.publicKey,
        )
        console.log(`Your ata is: ${ata.address.toBase58()}`);

        // Mint to ATA
        const mintTx = await mintTo(
            connection,
            keypair,
            mint,
            ata.address,
            keypair,
            amount,   
        )

        console.log(`Your mint txid: ${mintTx}`);
    } catch(error) {
        console.log(`Oops, something went wrong: ${error}`)
    }
})()

