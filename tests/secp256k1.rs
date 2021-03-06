#[cfg(all(test, feature = "secp256k1"))]
mod test {
    use cryptographer::rand::Rand;
    use cryptographer::{x::secp256k1, Signer};

    #[test]
    fn sign_verify() {
        let mut reader = Rand::new();

        let priv_key = secp256k1::generate_key(&mut reader).unwrap();

        let msg = [123u8; 32];

        let sig = secp256k1::sign(&priv_key, &msg[..]).unwrap();

        let pub_key = priv_key.public();
        secp256k1::verify(&pub_key, &msg[..], &sig).unwrap();
    }
}
