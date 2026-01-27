import { Keypair, Connection, Commitment } from "@solana/web3.js";
import { createMint } from '@solana/spl-token';
import wallet from "/Users/lmqrd/.config/solana/id.json"



// Load keypair from Solana CLI configuration
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
console.log('Public key: ', keypair.publicKey.toBase58());

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

(async () => {
    try {
        // Start here
        const mint= await createMint(
            connection,
            keypair,
            keypair.publicKey,
            null,
            9
        )
        console.log(`Mint created: ${mint.toBase58()}`)
    } catch(error) {
        console.log(`Oops, something went wrong: ${error}`)
    }
})()
