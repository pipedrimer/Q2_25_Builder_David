import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Keypair, PublicKey } from "@solana/web3.js";
import { AnchorVault } from "../target/types/vault_anchor";

describe("anchor-vault", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env()); 
  
  const provider= anchor.getProvider()
  const connection= provider.connection;
  

  const program = anchor.workspace.anchorVault as Program<AnchorVault>;
  const programId = program.programId;

  const confirm = async(signature: string): Promise<string> =>{
    const block = await connection.getLatestBlockhash();

    await connection.confirmTransaction({signature, ...block});
    return signature;
  }

  const log = async(signature:string): Promise<string> =>{
    console.log(
      `Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=${connection.rpcEndpoint}`
    );
    return signature;
  }
  const maker = Keypair.generate();

  const vault_state = PublicKey.findProgramAddressSync([Buffer.from("state"), maker.publicKey.toBuffer()], programId)[0];

  const vault = PublicKey.findProgramAddressSync([Buffer.from("vault"), vault_state.toBuffer()], programId)[0]
 

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
