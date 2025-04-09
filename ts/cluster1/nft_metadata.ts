import wallet from "../wba-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        //https://gateway.irys.xyz/
        // Follow this JSON structure
        // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

         const image = "https://devnet.irys.xyz/BBRQ1SzRzyC8ZRAo75Uq62SaAjK8pFeo4TqnLRGEAgJn"
         const metadata = {
              name: "Jeff Wif Mat",
              symbol: "JWM",
              description: "Jeff having a chillax day on his special rug",
              image,
            attributes: [
                {trait_type: 'shades', value: 'dark'},
                {trait_type:'pose' , value:'switch gang sign'}
            ],
            properties: {
                files: [
                    {
                        type: "image/png",
                        uri: "https://devnet.irys.xyz/BBRQ1SzRzyC8ZRAo75Uq62SaAjK8pFeo4TqnLRGEAgJn"
                    },
                ]
            },
            creators: []
        };
        const myUri = await umi.uploader.uploadJson(metadata)
        console.log("Your metadata URI: ", myUri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
