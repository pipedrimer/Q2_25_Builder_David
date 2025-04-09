import wallet from "../dev-wallet.json";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
  createMetadataAccountV3,
  CreateMetadataAccountV3InstructionAccounts,
  CreateMetadataAccountV3InstructionArgs,
  DataV2Args,
  MPL_TOKEN_METADATA_PROGRAM_ID,
} from "@metaplex-foundation/mpl-token-metadata";
import {
  createSignerFromKeypair,
  signerIdentity,
  publicKey,
} from "@metaplex-foundation/umi";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { PublicKey } from "@solana/web3.js";

// Define our Mint address
const mint = publicKey("43CTRD6rXrQBPPPFct1YupvusPSLJethzdXGh71aJskG");

//const metadata_seeds = [Buffer.from("metadata"), mint.toBuffer()];

//const [metadata_pda, _bump] = PublicKey.findProgramAddressSync(
 // metadata_seeds,
  //MPL_TOKEN_METADATA_PROGRAM_ID
//);

// Create a UMI connection
const umi = createUmi("https://api.devnet.solana.com");
const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(signer));

(async () => {
  try {
    // Start here
    let accounts: CreateMetadataAccountV3InstructionAccounts = {
      mint,
      mintAuthority: signer,
    };

    let data: DataV2Args = {
      name: "CR7 TOKEN",
      symbol: "CR7",
      uri: "https://raw.githubusercontent.com/pipedrimer/Metadata/refs/heads/main/metadata.json",
      sellerFeeBasisPoints: 0,
      creators: null,
      collection: null,
      uses: null,
    };

    let args: CreateMetadataAccountV3InstructionArgs = {
      data,
      isMutable: true,
      collectionDetails: null
    };

    let tx = createMetadataAccountV3(umi, {
      ...accounts,
      ...args,
    });

    let result = await tx.sendAndConfirm(umi);
    console.log(bs58.encode(result.signature));
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();
