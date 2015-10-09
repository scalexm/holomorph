use crypto::digest::Digest;
use crypto::md5::Md5;
use crypto::aes;
use crypto::buffer::{RefReadBuffer, RefWriteBuffer, BufferResult, WriteBuffer, ReadBuffer};
use crypto::symmetriccipher::{Encryptor, SymmetricCipherError};
use crypto::blockmodes::NoPadding;

pub fn md5(s: &str) -> String {
    let mut md5 = Md5::new();
    md5.input_str(&s);
    md5.result_str()
}

pub fn aes_256(key: &[u8], iv: &[u8], input: &[u8]) -> Result<Vec<u8>, SymmetricCipherError> {
    let mut result = Vec::new();
    let mut cbc = aes::cbc_encryptor(aes::KeySize::KeySize256, key, iv, NoPadding);
    let mut output = [0; 32];
    let mut read_buffer = RefReadBuffer::new(&input);
    let mut write_buffer = RefWriteBuffer::new(&mut output);

    loop {
        let res = try!(cbc.encrypt(&mut read_buffer, &mut write_buffer, true));

        result.extend(write_buffer.take_read_buffer()
                                  .take_remaining()
                                  .iter()
                                  .map(|&i| i));

        if let BufferResult::BufferUnderflow = res {
            break;
        }
    }

    Ok(result)
}
