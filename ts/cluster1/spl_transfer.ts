import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import wallet from "/Users/lmqrd/.config/solana/id.json"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("DpZ3H6iCty6GQxT5zC84jxBSN7KnFH5oDVHcVWyEisEo");

// Recipient address
const to = new PublicKey("8RYEWyZGhmUDqwuQNqdFEdti3U6AU1ggo3t8TurjHQsw");


const decimals = 9n;
const token_decimals = 10n ** decimals;
const amount = 10n *token_decimals; // I want to transfer 10 tokens

(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create 
        const fromAta = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            keypair.publicKey
        )

        // Get the token account of the toWallet address, and if it does not exist, create it
        const toAta = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            to
        )

        // Transfer the new token to the "toTokenAccount" we just created
        const transferTx = await transfer(
            connection,
            keypair,
            fromAta.address,
            toAta.address,
            keypair,
            amount
        )
        console.log("transfer txid:", transferTx);
        console.log("from ata:", fromAta.address.toBase58());
        console.log("to ata:", toAta.address.toBase58());
        console.log("amount:", amount);
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();