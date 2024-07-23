use std::{fs, path::Path, io};
use bip39::Mnemonic;
use solana_sdk::signature::Signer;
use eth_keystore::encrypt_key;
use rand::rngs::OsRng;
use rand::RngCore;
use uuid::Uuid;

// generate wallet keypairs from mnemonic(seed phrase)
pub fn gen_sol_wallet(mnemonic: &Mnemonic) -> (
    Vec<solana_sdk::pubkey::Pubkey>,
    Vec<solana_sdk::signer::keypair::Keypair>,
) {

    //new seed using bip 39 and mnemonic
    let seed = mnemonic.to_seed_normalized("1233"); //128 hex chars = 512 bits = 64 bytes
    let seed_bytes = &seed;

    let mut keypairs = vec![];
    let mut public_keys = vec![];

    for i in 0..10 {
        // we can also fix the derivation path before entering the loop
        let derivation_path = solana_sdk::derivation_path::DerivationPath::new_bip44(Some(i), Some(0));
        let keypair = solana_sdk::signer::keypair::keypair_from_seed_and_derivation_path(
            seed_bytes,
            Some(derivation_path),
        )
        .unwrap();
        let pubk = keypair.pubkey();

        keypairs.push(keypair);
        public_keys.push(pubk);
    }

    (public_keys, keypairs)
}


// generate a new pnemonic(for a new sol wallet)
pub fn generate_and_save_mnemonic() -> (Mnemonic, String){
    let mut entropy = [0u8; 16]; // 16 bytes (128 bits) for example
    let mut rng = OsRng; //retrieving entropy from the kernel
    rng.fill_bytes(&mut entropy);
    let mnemonic = Mnemonic::from_entropy(&entropy).unwrap();
    //generate a strong password
    let uuid = encrypt_keystore_file(&mnemonic, "235542");
    (mnemonic, uuid)
}


// import keypair from private key and save it inside a file
pub fn import_and_save_private_key(private_key: &str) ->String{
    let dir = Path::new("./keys");
    let mut rng = rand::thread_rng();
    let private_key_bytes = bs58::decode(private_key).into_vec().unwrap();
    let mut x: [u8; 16] = [0; 16];
    x.copy_from_slice(&private_key_bytes[..16]);
    let uuid = encrypt_key(dir, &mut rng, private_key_bytes, "424554", Some(&Uuid::from_bytes(x).to_string())).expect("Failed to encrypt and save private key");
    uuid
}

pub fn encrypt_keystore_file(mnemonic: &Mnemonic, password: &str) -> String {
    let dir = Path::new("./keys");
    let mut rng = rand::thread_rng();
    // save the entropy, not the seed into keystore
    let entropy = mnemonic.to_entropy();
    remove_dir_contents(dir).unwrap(); //clean out existing keys
    // generate a new random string for the name
    let name_bytes = mnemonic.to_entropy_array().0;
    let mut x: [u8; 16] = [0; 16];
    x.copy_from_slice(&name_bytes[..16]);
    let uuid = encrypt_key(&dir, &mut rng, entropy, password, Some(&Uuid::from_bytes(x).to_string())).unwrap();
    // println!("{}", uuid);

    uuid
}

pub fn remove_dir_contents<P: AsRef<Path>>(path: P) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        fs::remove_file(entry?.path())?;
    }
    Ok(())
}