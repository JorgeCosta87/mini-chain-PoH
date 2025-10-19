use base58::{FromBase58, ToBase58};
use ed25519_dalek::{PUBLIC_KEY_LENGTH, Signature, Signer, SigningKey};
use rand::rngs::OsRng;

pub struct AccountKey {
    keypair: SigningKey,
}

// Based on https://docs.rs/ed25519-dalek/latest/ed25519_dalek/#example
impl AccountKey {
    pub fn new() -> Self {
        let mut csprng = OsRng;
        let keypair = SigningKey::generate(&mut csprng);

        Self { keypair }
    }

    pub fn public_key_string(&self) -> String {
        let verifying_key_bytes: [u8; PUBLIC_KEY_LENGTH] = self.keypair.verifying_key().to_bytes();

        verifying_key_bytes.to_base58()
    }

    pub fn sign(&self, message: &[u8]) -> String {
        let signature: Signature = self.keypair.sign(message);

        signature.to_bytes().to_base58()
    }

    pub fn verify(&self, message: &[u8], sig: String) -> bool {
        let sig_bytes = sig.from_base58().unwrap();
        let sig_array: [u8; 64] = sig_bytes.try_into().unwrap();
        let signature = Signature::from_bytes(&sig_array);

        match self.keypair.verify(message, &signature) {
            Ok(()) => return true,
            Err(e) => {
                println!("Error verifying message: {:#?}", e);

                false
            }
        }
    }
}

/*
Ok(v) => {
    if v.len() != 32
    {
        println!("Invalid public key length! Expected 32, got {}", v.len());
        return false
    }

    let mut pubkey_bytes_array = [0u8; 32];
    pubkey_bytes_array.copy_from_slice(&v);
    let pubkey = VerifyingKey::from_bytes(&pubkey_bytes_array);

},
Err(e)    => {
    println!("Error decoding pubkey: {:?}", e);
    return false;
}
};

*/
