use crate::{gen_pass, get_reader, TextSignFormat};
use anyhow::Result;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use std::fs;
use std::io::Read;
use std::path::Path;

trait TextSign {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}
trait TextVerify {
    fn verify(&self, reader: impl Read, sign: &[u8]) -> Result<bool>;
    // 等价于上面的写法
    // 区别于sign的写法，这种是静态分派的写法，效率相对更高，但是打包会比较大
    // fn verify<R: Read>(&self, reader: R)->Result<(bool)>;
}

trait KeyLoader {
    fn load(path: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized; //返回的类型是有固定长度的数据类型
}

trait KeyGenerate {
    fn generate() -> Result<Vec<Vec<u8>>>;
}
struct Blake3 {
    key: [u8; 32],
}
struct ED25519Signer {
    key: SigningKey,
}

struct ED25519Verifier {
    key: VerifyingKey,
}

pub fn process_text_sign(input: &str, key: &str, format: TextSignFormat) -> Result<()> {
    let mut reader = get_reader(input)?;
    let signed = match format {
        TextSignFormat::Blake3 => {
            let singer = Blake3::load(key)?;
            singer.sign(&mut reader)?
        }
        TextSignFormat::ED25519 => {
            let singer = ED25519Signer::load(key)?;
            singer.sign(&mut reader)?
        }
    };
    let signed = URL_SAFE_NO_PAD.encode(&signed);
    println!("{}", signed);

    Ok(())
}

pub fn process_text_verify(
    input: &str,
    key: &str,
    sign: &str,
    format: TextSignFormat,
) -> Result<bool> {
    let mut reader = get_reader(input)?;
    let sign = URL_SAFE_NO_PAD.decode(sign)?;
    let verified = match format {
        TextSignFormat::Blake3 => {
            let singer = Blake3::load(key)?;
            singer.verify(&mut reader, &sign)?
        }
        TextSignFormat::ED25519 => {
            let singer = ED25519Verifier::load(key)?;
            singer.verify(&mut reader, &sign)?
        }
    };
    Ok(verified)
}

pub fn process_generate(format: &TextSignFormat) -> Result<Vec<Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::ED25519 => ED25519Signer::generate(),
    }
}
impl KeyGenerate for Blake3 {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let key = gen_pass(32, true, true, true, true)?;
        let key = key.as_bytes().to_vec();
        Ok(vec![key])
    }
}

impl KeyGenerate for ED25519Signer {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let mut csprng = rand::rngs::OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let sk = signing_key.as_bytes().to_vec();
        let pk = signing_key.verifying_key().to_bytes().to_vec();
        Ok(vec![sk, pk])
    }
}
impl KeyLoader for Blake3 {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        let loader = Self::try_new(&key)?;
        Ok(loader)
    }
}
impl KeyLoader for ED25519Signer {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        let loader = Self::try_new(&key)?;
        Ok(loader)
    }
}

impl KeyLoader for ED25519Verifier {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        let loader = Self::try_new(&key)?;
        Ok(loader)
    }
}

impl Blake3 {
    fn new(key: [u8; 32]) -> Self {
        Self { key }
    }
    fn try_new(key: &[u8]) -> Result<Self> {
        let key = &key[..32];
        let key = key.try_into()?;
        Ok(Self::new(key))
    }
}
impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, mut reader: impl Read, sign: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let binding = blake3::keyed_hash(&self.key, &buf);
        let hash = binding.as_bytes();
        Ok(hash == sign)
    }
}

impl TextSign for ED25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(self.key.sign(&buf).to_bytes().to_vec())
    }
}

impl TextVerify for ED25519Verifier {
    fn verify(&self, mut reader: impl Read, sign: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sign = Signature::from_bytes(sign.try_into()?);
        let res = self.key.verify(&buf, &sign).is_ok();
        Ok(res)
    }
}

impl ED25519Signer {
    fn new(key: SigningKey) -> Self {
        Self { key }
    }
    fn try_new(key: &[u8]) -> Result<Self> {
        let key = SigningKey::from_bytes(&key.try_into()?);
        Ok(Self::new(key))
    }
}

impl ED25519Verifier {
    fn new(key: VerifyingKey) -> Self {
        Self { key }
    }
    fn try_new(key: &[u8]) -> Result<Self> {
        let key = VerifyingKey::from_bytes(&key.try_into()?)?;
        Ok(Self::new(key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_blake3() -> Result<()> {
        let blake3 = Blake3::load("fixtures/blake3.txt")?;
        let data = b"hello world";
        let signed = blake3.sign(&mut data.as_ref())?;
        let verified = blake3.verify(&mut data.as_ref(), &signed)?;
        assert!(verified);
        Ok(())
    }
    #[test]
    fn test_ed25519() -> Result<()> {
        let sk = ED25519Signer::load("fixtures/ed25519.sk")?;
        let pk = ED25519Verifier::load("fixtures/ed25519.pk")?;
        let data = b"hello world";
        let signed = sk.sign(&mut data.as_ref())?;
        let verified = pk.verify(&mut data.as_ref(), &signed)?;
        assert!(verified);
        Ok(())
    }
}
