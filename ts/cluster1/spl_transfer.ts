import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, sendAndConfirmTransaction } from "@solana/web3.js"
import wallet from "../wba-wallet.json"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "processed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("8cNVHhZ3vrqNRYJ1UDVFVUXNHCC1fTzQs5UXjVHFTX5s");

// Recipient address
const to = new PublicKey("BvhV49WPYBbzPu8Fpy8YnPnwhNWLbm9Vmdj2T5bNSotS");




(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it
        const from_ata = await getOrCreateAssociatedTokenAccount(connection, keypair, mint , keypair.publicKey )

        // Get the token account of the toWallet address, and if it does not exist, create it

        const to_ata = await getOrCreateAssociatedTokenAccount(connection, keypair, mint , to )

        // Transfer the new token to the "toTokenAccount" we just created
        await transfer( connection, keypair, from_ata.address , to_ata.address, keypair.publicKey, 1)
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();