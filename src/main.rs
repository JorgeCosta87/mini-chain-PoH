use crate::keys::AccountKey;

mod keys;

fn main() {
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
}
