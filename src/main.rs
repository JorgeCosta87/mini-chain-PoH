use crate::keys::AccountKey;
use poh::{PoH, Hash};

mod keys;
mod poh;

const SEED: Hash = *b"ssssssssssssssssssssssssssssssss";
const HASHES_PER_TICK: u64 = 12_500;
const TICKS_PER_SLOT: u64 = 64;

fn main() {

    //Test Account keys
    let acc = AccountKey::new();
    let acc1 = AccountKey::new();

    println!("pub key: {:?}", acc.public_key_string());

    let message = String::from("HELLO SOLANA");
    let signed_msg = acc.sign(&message.as_bytes());

    println!("signed msg: {}", signed_msg);

    let res = acc.verify(&message.as_bytes(), signed_msg.clone());
    println!("Verify result: {:?}", res);

    let res = acc1.verify(&message.as_bytes(), signed_msg);
    println!("Verify result: {:?}", res);

    let mut poh = PoH::new(
        SEED, 
        HASHES_PER_TICK,
        TICKS_PER_SLOT
    );

    //Test PoH
    println!("poh state 1: {:#?}", poh);
    poh.compute_tick_hashes();
    println!("poh state 2: {:#?}", poh);
    poh.mixin_data(SEED);
    println!("poh state 3: {:#?}", poh);
    poh.compute_tick_hashes();
    println!("poh state 4: {:#?}", poh);

}
