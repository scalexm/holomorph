//! Command line tool helping with signatures within the Dofus conventions.
//! Implementation was straight-forwardly translated from Dofus sources:
//! * `com/ankamagames/jerakine/utils/crypto/SignatureKey.as`
//! * `com/ankamagames/jerakine/utils/crypto/Signature.as`

use docopt::Docopt;
use openssl::rsa::{Padding, Rsa};
use serde::Deserialize;

const USAGE: &'static str = "
Dofus signature utilities.

Usage:
  signature gen
  signature sign <priv> --hosts=<string>
  signature sign <priv> --file=<path>
  signature (-h | --help)
  signature --version

Options:
  -h --help        Show this screen.
  --hosts=<hosts>  Hosts string to sign.
  --file=<path>    Path to file to sign.
  --version        Show version.
";

#[derive(Deserialize)]
struct Args {
    cmd_gen: bool,
    cmd_sign: bool,
    arg_priv: Option<String>,
    flag_hosts: Option<String>,
    flag_file: Option<String>,
}

const KEY_HEADER: &'static str = "DofusPublicKey";
const SIGN_HEADER: &'static str = "AKSF";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if args.cmd_gen {
        // Generate an RSA key pair.
        //
        // Write the public key in Dofus public key format to `pub.bin` and
        // write the private key in PEM format to `priv.pem`.
        //
        // The public key should be replacing the `SIGNATURE_KEY_DATA` binary
        // resource in `DofusInvoker.swf`.
        //
        // The private key may be used to:
        // * sign the "connection.host" string in `config.xml`, the signature
        //   being written in "connection.host.signature"
        // * sign the client theme in e.g. `content/themes/darkStone`, the
        //   signature being written in a `signature.xmls` file in the theme
        //   directory

        let keypair = Rsa::generate(1024)?;
        let n = keypair.n().to_hex_str()?;
        let e = keypair.e().to_hex_str()?;

        let mut buf = vec![];
        buf.extend_from_slice(&(KEY_HEADER.len() as u16).to_be_bytes());
        buf.extend_from_slice(KEY_HEADER.as_bytes());
        buf.extend_from_slice(&(n.len() as u16).to_be_bytes());
        buf.extend_from_slice(n.as_ref());
        buf.extend_from_slice(&(e.len() as u16).to_be_bytes());
        buf.extend_from_slice(e.as_ref());

        std::fs::write("pub.bin", &buf)?;
        std::fs::write("priv.pem", &keypair.private_key_to_pem()?)?;
    } else if args.cmd_sign {
        let key = Rsa::private_key_from_pem(&std::fs::read(args.arg_priv.unwrap())?)?;

        let input = match args.flag_hosts.as_ref() {
            Some(hosts) => hosts.clone(),
            None => std::fs::read_to_string(args.flag_file.as_ref().unwrap())?,
        };

        let mut hash = vec![];
        let random = rand::random::<u8>();
        hash.push(random);
        hash.extend_from_slice(&(input.len() as u32).to_be_bytes());
        hash.extend_from_slice(format!("{:x}", md5::compute(input.as_bytes())).as_bytes());
        for i in 2..hash.len() {
            hash[i] ^= random;
        }

        let mut signed = vec![0; 256];
        let n = key.private_encrypt(&hash, &mut signed, Padding::PKCS1)?;
        signed.truncate(n);

        let mut out = vec![];
        out.extend_from_slice(&(SIGN_HEADER.len() as u16).to_be_bytes());
        out.extend_from_slice(SIGN_HEADER.as_bytes());
        out.extend_from_slice(&1u16.to_be_bytes());
        out.extend_from_slice(&(signed.len() as u32).to_be_bytes());
        out.extend_from_slice(&signed);

        if args.flag_hosts.is_some() {
            // If signing the hosts string, output the base64-encoded
            // signature.
            println!("{}", base64::encode(&out));
        } else {
            // If signing a file, append the data and write it in binary form
            // to `out.bin`.
            out.extend_from_slice(input.as_bytes());
            std::fs::write("out.bin", &out)?;
        }
    }

    Ok(())
}
