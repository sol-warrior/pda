//-----TS Solana kit to find PDA-----
import {
  type Address,
  getAddressEncoder,
  getProgramDerivedAddress,
} from "@solana/kit";

const programId = "11111111111111111111111111111111" as Address;

const getEncoder = getAddressEncoder();

const user_add = getEncoder.encode(
  "BQuvWWJmjhS2X4jc6G9T2meEHdyzY6RsooTHLMABKeah" as Address
);
const seeds = ["solwarrior", user_add];

const [pda, bump] = await getProgramDerivedAddress({
  programAddress: programId,
  seeds,
});

console.log("PDA : ", pda); //4NajcZCzkipVs8SrvtCdN5pVM5KLLNd2aRQVb9mrdXHr
console.log("Seeds : ", seeds);
console.log("Bump : ", bump);

// BQuvWWJmjhS2X4jc6G9T2meEHdyzY6RsooTHLMABKeah
