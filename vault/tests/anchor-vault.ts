import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram } from "@solana/web3.js";
import { VaultAnchor } from "../target/types/vault_anchor";
import { BN } from "bn.js";

describe("vault-anchor", () => {
 
  anchor.setProvider(anchor.AnchorProvider.env()); 
  
  const provider = anchor.getProvider()
  const connection = provider.connection;
  

  const program = anchor.workspace.anchorVault as Program<VaultAnchor>;
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

  const vault = PublicKey.findProgramAddressSync([Buffer.from("vault"), vault_state.toBuffer()], programId)[0];

 const accounts= {
  signer:maker.publicKey,
  vault_state,
  vault,
  systemProgram:SystemProgram.programId,
 }
 
 before("airdrop account", async() =>{
     
  
     await connection.requestAirdrop(maker.publicKey, 10*LAMPORTS_PER_SOL).then(confirm)
 })

  it("Is initialized!", async () => {
    
     await program.methods
    .initialize()
    .accounts({...accounts})
    .signers([maker])
    .rpc()
    .then(confirm)
    .then(log)
    
  });

  it("Deposit", async () =>{

    await program.methods
    .deposit(new BN(1e9))
    .accounts({...accounts})
    .signers([maker])
    .rpc()
    .then(confirm)
    .then(log)
  });

  it("Withdraw", async () =>{

    await program.methods
    .withdraw(new BN(1e3))
    .accounts({...accounts})
    .signers([maker])
    .rpc()
    .then(confirm)
    .then(log)
  });

  it ("Close", async ()=>{
    await program.methods
    .close()
    .accounts({...accounts})
    .signers([maker])
    .rpc()
    .then(confirm)
    .then(log)
  })
});
