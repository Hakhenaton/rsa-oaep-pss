# RSA-OAEP-PSS

[![Build Status](https://drone.rnzaou.me/api/badges/Hakhenaton/rsa-oaep-pss/status.svg)](https://drone.rnzaou.me/Hakhenaton/rsa-oaep-pss)

A pure Rust implementation of the RSA public key cryptosystem. 

The following features are available:

- Encryption using [Optimal Asymmetric Encryption Padding (OAEP)](https://fr.wikipedia.org/wiki/Optimal_Asymmetric_Encryption_Padding)
- Signature using [Probabilistic Signature Scheme (PSS)](https://en.wikipedia.org/wiki/Probabilistic_signature_scheme)

> :warning: **This crate has not been audited by any peer**: We encourage you to review the code carefully before using it.

## Useful links

- [crates.io](https://crates.io/crates/rsa_oaep_pss)
- [docs.rs](https://docs.rs/rsa-oaep-pss)
- [PKCS#1 RFC](https://www.rfc-editor.org/rfc/pdfrfc/rfc8017.txt.pdf)

## Installation

Add the following line to your `Cargo.toml` dependencies:

```toml
[dependencies]
rsa-oaep-pss = "1.0.1"
```

## How to use ?

### Keys generation

```rust
let (public_key, private_key) = rsa_oaep_pss::generate_rsa_keys(&mut rng, 2048)
    .expect("keys generation error");
```

### Encryption using OAEP scheme

```rust
let message = b"some secret";

let mut oaep = rsa_oaep_pss::RsaOaep::new(rand::rngs::OsRng, &sha2::Sha256::new());

let ciphertext = oaep
    .encrypt(&public_key, message)
    .expect("encryption error");

let recovered = oaep
    .decrypt(&private_key, &ciphertext)
    .expect("decryption error");

assert_eq!(recovered, message);
```

### Signature using PSS scheme

```rust
let message = b"message to sign";

let mut pss = rsa_oaep_pss::RsaPss::new(rand::rngs::OsRng, &sha2::Sha256::new());

let signature = pss.sign(&private_key, message).expect("signature error");

let verification = pss.verify(&public_key, message, &signature);

assert!(verification.is_ok());
```

## Run the examples

You can run examples contained in the `examples` folder by using the following command:

```sh
cargo run --example <filename>.rs --release 
```