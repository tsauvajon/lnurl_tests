use std::convert::Infallible;
use std::str::FromStr;

use bech32::FromBase32;
use image::Luma;
use lightning_invoice::{Currency, InvoiceBuilder};
use qrcode::render::svg;
use qrcode::{render::unicode, QrCode};
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use serde::Deserialize;

const LN_URL: &str = "lnurl1dp68gurn8ghj7ampd3kx2ar0veekzar0wd5xjtnrdakj7tnhv4kxctttdehhwm30d3h82unvwqhhg6tdv438yctwvscrxhnh4nf";


#[tokio::main]
async fn main() {
    let recipient = LightningRecipient::from_str(LN_URL).unwrap();
    recipient.decode_url();

    let recipient = LightningRecipient::from_str("timebrand03@walletofsatoshi.com").unwrap();
    let url = recipient.decode_url();
    let wallet_response: WalletResponse = reqwest::get(url).await.unwrap().json().await.unwrap();

    let amount_sats = 1_000;
    let amount = amount_sats * 1000;
    if amount < wallet_response.min_sendable || amount > wallet_response.max_sendable {
        panic!("Amount out of bonds");
    }

    let url = format!("{}?amount={amount}", wallet_response.callback_url);
    let callback_response: CallbackResponse =
        reqwest::get(url).await.unwrap().json().await.unwrap();

    println!("{}", callback_response.invoice);

    callback_response.print_qr_code();
    callback_response.save_qr_code();
}

#[derive(Debug, Eq, PartialEq)]
enum LightningRecipient {
    LightningAddress { domain: String, username: String },
    LnUrl(String),
}

impl FromStr for LightningRecipient {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("lnurl") {
            return Ok(Self::LnUrl(s.to_string()));
        }

        if s.contains('@') {
            let mut parts = s.split('@');
            if parts.clone().count() != 2 {
                return Err(());
            }

            return Ok(Self::LightningAddress {
                username: parts.next().unwrap().to_string(),
                domain: parts.next().unwrap().to_string(),
            });
        }

        Err(())
    }
}

#[test]
fn lr_from_str() {
    assert_eq!(
        Ok(LightningRecipient::LightningAddress {
            domain: "world.com".to_string(),
            username: "hello".to_string()
        }),
        LightningRecipient::from_str("hello@world.com")
    );

    assert_eq!(
        Ok(LightningRecipient::LnUrl("lnurlabcdefgh".to_string())),
        LightningRecipient::from_str("lnurlabcdefgh")
    );

    assert_eq!(Err(()), LightningRecipient::from_str("hello@world@com"));
    assert_eq!(Err(()), LightningRecipient::from_str("helloworld.com"));
}

impl LightningRecipient {
    fn decode_url(&self) -> String {
        match self {
            LightningRecipient::LnUrl(encoded) => {
                let (_hrp, data, _variant) = bech32::decode(&encoded).unwrap();
                let decoded = Vec::<u8>::from_base32(&data).unwrap();
                String::from_utf8(decoded).unwrap()
            }
            LightningRecipient::LightningAddress { domain, username } => {
                let url = format!("https://{domain}/.well-known/lnurlp/{username}");
                println!("Lightning Address URL: {url}");
                url
            }
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct WalletResponse {
    #[serde(rename = "callback")]
    pub callback_url: String,
    pub max_sendable: i64,
    pub min_sendable: i64,
    // pub metadata: String,
    // pub comment_allowed: i64,
    // pub tag: String,
    // pub allows_nostr: bool,
    // pub nostr_pubkey: String,
}

#[derive(Deserialize)]
struct CallbackResponse {
    #[serde(rename = "pr")]
    invoice: String,
    #[serde(rename = "routes")]
    _routes: Vec<()>,
}

impl CallbackResponse {
    fn print_qr_code(&self) {
        let code = QrCode::new(&self.invoice).unwrap();
        let image = code
            .render::<unicode::Dense1x2>()
            .dark_color(unicode::Dense1x2::Light)
            .light_color(unicode::Dense1x2::Dark)
            .build();
        println!("{image}");
    }

    fn save_qr_code(&self) {
        let code = QrCode::new(&self.invoice).unwrap();
        let image = code.render::<Luma<u8>>().build();
        image.save("/Users/thomas/dev/astron/qrcode.png").unwrap();

        let image = code
            .render()
            .min_dimensions(200, 200)
            .dark_color(svg::Color("#000000"))
            .light_color(svg::Color("#ffffff"))
            .build();
        std::fs::write("/Users/thomas/dev/astron/qrcode.svg", image).unwrap();
    }
}

fn _create_qr_invoice() {
    let _private_key = SecretKey::from_slice(
        &[
            0xe1, 0x26, 0xf6, 0x8f, 0x7e, 0xaf, 0xcc, 0x8b, 0x74, 0xf5, 0x4d, 0x26, 0x9f, 0xe2,
            0x06, 0xbe, 0x71, 0x50, 0x00, 0xf9, 0x4d, 0xac, 0x06, 0x7d, 0x1c, 0x04, 0xa8, 0xca,
            0x3b, 0x2d, 0xb7, 0x34,
        ][..],
    )
    .unwrap();
    let private_key =
        SecretKey::from_str("e126f68f7eafcc8b74f54d269fe206be715000f94dac067d1c04a8ca3b2db734")
            .unwrap();
    //e126f68f7eafcc8b74f54d269fe206be715000f94dac067d1c04a8ca3b2db734
    //4b79476f6a58644a7a784a46517152686e3870504264745a3673325143326b61343456534171364763747967717773347a544872
    println!("{}", private_key.display_secret());
    let pub_key =
        PublicKey::from_str("03e7156ae33b0a208d0744199163177e909e80176e55d97a2f221ede0f934dd9ad")
            .unwrap();

    // let payment_hash = sha256::Hash::from_slice(&[0; 32][..]).unwrap();
    // let payment_secret = PaymentSecret([42u8; 32]);

    let invoice = InvoiceBuilder::new(Currency::Bitcoin)
        .description("Coins pls!".into())
        .amount_milli_satoshis(100)
        .payee_pub_key(pub_key)
        // .payment_hash(payment_hash)
        // .payment_secret(payment_secret)
        .current_timestamp()
        .min_final_cltv_expiry_delta(144)
        .build_raw()
        .unwrap()
        .sign(|hash| {
            Ok::<secp256k1::ecdsa::RecoverableSignature, Infallible>(
                Secp256k1::new().sign_ecdsa_recoverable(hash, &private_key),
            )
        })
        // .build_signed(|hash| Secp256k1::new().sign_ecdsa_recoverable(hash, &private_key))
        .unwrap();

    println!("{invoice}");

    assert!(invoice.to_string().starts_with("lnbc1"));
}
