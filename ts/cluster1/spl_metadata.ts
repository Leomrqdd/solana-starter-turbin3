import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { 
    createMetadataAccountV3, 
    CreateMetadataAccountV3InstructionAccounts, 
    CreateMetadataAccountV3InstructionArgs,
    DataV2Args
} from "@metaplex-foundation/mpl-token-metadata";
import { createSignerFromKeypair, signerIdentity, publicKey } from "@metaplex-foundation/umi";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import wallet from "/Users/lmqrd/.config/solana/id.json"


// Define our Mint address
const mint = publicKey("DpZ3H6iCty6GQxT5zC84jxBSN7KnFH5oDVHcVWyEisEo")

// Create a UMI connection
const umi = createUmi('https://api.devnet.solana.com');
const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(createSignerFromKeypair(umi, keypair)));

(async () => {
    try {
        // accounts
        let accounts: CreateMetadataAccountV3InstructionAccounts = {
            mint: mint,
            mintAuthority: signer

         }

        //data
        let data: DataV2Args = {
            name: "leo_kevred",
            symbol:"LKE",
            uri: "https://gray-familiar-wombat-195.mypinata.cloud/ipfs/bafkreiftaydfqgjhtgchpmdm6tzznp577wsgftzsjyttky5qt7mag5zmcy",
            sellerFeeBasisPoints: 0,
            creators: null,
            collection: null,
            uses: null,
        }

        //args to create metadata account
        let args: CreateMetadataAccountV3InstructionArgs = {
            data:data,
            isMutable:true,
            collectionDetails:null,
        }

        //tx to create metadata account
        let tx = createMetadataAccountV3(
            umi,
            {
                ...accounts,
                ...args
            }
        )

        //send and confirm tx
        let result = await tx.sendAndConfirm(umi);
        const txInfo = await umi.rpc.getTransaction(result.signature)
        console.log(bs58.encode(result.signature));
        console.log(txInfo);
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();
