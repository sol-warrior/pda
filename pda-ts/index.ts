//-----TS Script to find PDA-----
import { PublicKey } from "@solana/web3.js";

const programId = new PublicKey("11111111111111111111111111111111");
const seed = "solwarrior";
const vault_add = new PublicKey("BQuvWWJmjhS2X4jc6G9T2meEHdyzY6RsooTHLMABKeah");

//findProgramAddressSync
//looping

// for (let bump = 255; bump >= 253; bump--) {
//   try {
//     const PDA = PublicKey.createProgramAddressSync(
//       [Buffer.from(seed), Buffer.from([bump])],
//       programId
//     );

//     console.log("Bump : ", bump, "PDA : ", PDA.toBase58());
//   } catch (error) {
//     console.log("Bump : ", error);
//   }
// }

const [pda, bump] = PublicKey.findProgramAddressSync(
  [Buffer.from(seed), vault_add.toBuffer()],
  programId
);

console.log("Pda : ", pda.toBase58()); //4NajcZCzkipVs8SrvtCdN5pVM5KLLNd2aRQVb9mrdXHr
console.log("Bump : ", bump); //255
