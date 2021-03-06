use std::io::Read;

#[cfg(feature = "internal")]
mod internal;

mod hash;

#[cfg(feature = "aes")]
pub mod aes;
#[cfg(feature = "cipher")]
pub mod cipher;
#[cfg(feature = "ed25519")]
pub mod ed25519;
#[cfg(feature = "elliptic")]
pub mod elliptic;
#[cfg(feature = "hmac")]
pub mod hmac;
#[cfg(feature = "md5")]
pub mod md5;
#[cfg(feature = "rand")]
pub mod rand;
#[cfg(feature = "rc4")]
pub mod rc4;
#[cfg(feature = "sha1")]
pub mod sha1;
#[cfg(feature = "sha256")]
pub mod sha256;
#[cfg(feature = "sha512")]
pub mod sha512;
#[cfg(feature = "subtle")]
pub mod subtle;

pub mod x;

pub use hash::*;

/// PrivateKey represents a private key using an unspecified algorithm
pub trait PrivateKey {}
/// PublicKey represents a public key using an unspecified algorithm
pub trait PublicKey {}

/// Signer is an interface for an opaque private key that can be used for signing operations. For
/// example, an RSA key kept in a hardware module
pub trait Signer {
    type PublicKey;

    /// Public returns the public key corresponding to the opaque, private key
    fn public(&self) -> Self::PublicKey;

    /// Sign signs digest with the private key, possibly using entropy from
    /// rand.
    ///
    /// @TODO: For an RSA key, the resulting signature should be either a
    /// PKCS#1 v1.5 or PSS signature (as indicated by opts). For an (EC)DSA
    /// key, it should be a DER-serialised, ASN.1 signature structure.
    ///
    /// @TODO: Hash implements the SignerOpts interface and, in most cases, one can
    /// simply pass in the hash function used as opts. Sign may also attempt
    /// to type assert opts to other types in order to obtain algorithm
    /// specific values. See the documentation in each package for details.
    ///
    /// @TODO: Note that when a signature of a hash of a larger message is needed,
    /// the caller is responsible for hashing the larger message and passing
    /// the hash (as digest) and the hash function (as opts) to Sign.
    /// @TODO: add the SignerOpts
    fn sign<T>(&self, rand: &mut T, digest: &[u8]) -> Result<Vec<u8>, String>
    where
        T: Read;
}
