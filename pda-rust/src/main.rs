//-----Rust Script to find PDA-----

use solana_sdk::{program_memory, pubkey::Pubkey};
use std::str::FromStr;

#[tokio::main]

async fn main() -> anyhow::Result<()> {
    let program_id = Pubkey::from_str("11111111111111111111111111111111")?;
    let seed = b"solwarrior";
    let vault_seed = Pubkey::from_str("BQuvWWJmjhS2X4jc6G9T2meEHdyzY6RsooTHLMABKeah")?;
    // for bump in (0..255).rev() {
    //     match Pubkey::create_program_address(&[seed.as_ref(), &[bump]], &programId) {
    //         Ok(pda) => println!("Bump : {} , PDA : {}", bump, pda),
    //         Err(err) => println!("bump : {} , Error : {}", bump, err),
    //     }
    // }

    let (pda, bump) = Pubkey::find_program_address(&[seed, vault_seed.as_ref()], &program_id);

    println!("bump : {} , Pda : {}", bump, pda);
    Ok(())
}

// PDA = BQuvWWJmjhS2X4jc6G9T2meEHdyzY6RsooTHLMABKeah , bump : 254

// BQuvWWJmjhS2X4jc6G9T2meEHdyzY6RsooTHLMABKeah

// BQuvWWJmjhS2X4jc6G9T2meEHdyzY6RsooTHLMABKeah

// BQuvWWJmjhS2X4jc6G9T2meEHdyzY6RsooTHLMABKeah
//BQuvWWJmjhS2X4jc6G9T2meEHdyzY6RsooTHLMABKeah
