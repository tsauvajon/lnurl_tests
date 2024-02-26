use std::{convert::Infallible, str::FromStr};

use image::Luma;
use lightning_invoice::{Currency, InvoiceBuilder};
use qrcode::{
    render::{svg, unicode},
    types::QrError,
    EcLevel, QrCode, Version,
};
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use serde::Deserialize;

use crate::recipient::{LightningRecipient, ParseRecipientError};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct WalletResponse {
    #[serde(rename = "callback")]
    pub callback_url: String,
    pub max_sendable: u64,
    pub min_sendable: u64,
    // pub metadata: String,
    // pub comment_allowed: i64,
    // pub tag: String,
    // pub allows_nostr: bool,
    // pub nostr_pubkey: String,
}

#[derive(Deserialize)]
struct CallbackResponse {
    #[serde(flatten)]
    invoice: Invoice,
    #[serde(rename = "routes")]
    _routes: Vec<()>,
}

pub(crate) enum Error {
    Reqwest(reqwest::Error),
    ParseRecipient(ParseRecipientError),
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reqwest(arg0) => f.debug_tuple("Reqwest").field(arg0).finish(),
            Self::ParseRecipient(arg0) => f.debug_tuple("ParseRecipient").field(arg0).finish(),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Reqwest(err)
    }
}

impl From<ParseRecipientError> for Error {
    fn from(err: ParseRecipientError) -> Self {
        Error::ParseRecipient(err)
    }
}

#[derive(Deserialize)]
pub(crate) struct Invoice {
    #[serde(rename = "pr")]
    data: String,
}

impl Invoice {
    pub(crate) async fn with_amount(recipient: &str, sats: u64) -> Result<Self, Error> {
        let recipient = LightningRecipient::from_str(recipient)?;
        let url = recipient.decode_url();
        let wallet_response: WalletResponse = reqwest::get(url).await?.json().await?;

        let amount = sats * 1000;
        if amount < wallet_response.min_sendable || amount > wallet_response.max_sendable {
            panic!("Amount out of bonds");
        }

        let url = format!("{}?amount={amount}", wallet_response.callback_url);
        let callback_response: CallbackResponse = reqwest::get(url).await?.json().await?;
        Ok(callback_response.invoice)
    }

    pub(crate) fn _print_qr_code(&self) {
        let code = QrCode::new(&self.data).unwrap();
        let image = code
            .render::<unicode::Dense1x2>()
            .dark_color(unicode::Dense1x2::Light)
            .light_color(unicode::Dense1x2::Dark)
            .build();
        println!("{image}");
    }

    pub(crate) fn _save_qr_code(&self) {
        let code = QrCode::new(&self.data).unwrap();
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

    pub(crate) fn qr_code(&self) -> Result<String, QrError> {
        // Try to fit the data until we get a match

        for size in 13..40 {
            match QrCode::with_version(&self.data, Version::Normal(size), EcLevel::M) {
                Ok(qr_code) => {
                    return Ok(qr_code
                        .render()
                        .quiet_zone(true)
                        .dark_color(svg::Color("#2b3252")) // 16ACEA
                        .light_color(svg::Color("#ef5455")) // FAD744
                        .build());
                }
                Err(QrError::DataTooLong) => {
                    println!("Too much data for size {size}");
                    continue;
                }
                Err(err) => return Err(err),
            }
        }

        Err(QrError::DataTooLong)
    }

    pub(crate) fn data(&self) -> String {
        self.data.clone()
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
    println!("{}", private_key.display_secret());
    let pub_key =
        PublicKey::from_str("03e7156ae33b0a208d0744199163177e909e80176e55d97a2f221ede0f934dd9ad")
            .unwrap();

    let invoice = InvoiceBuilder::new(Currency::Bitcoin)
        .description("Coins pls!".into())
        .amount_milli_satoshis(100)
        .payee_pub_key(pub_key)
        .current_timestamp()
        .min_final_cltv_expiry_delta(144)
        .build_raw()
        .unwrap()
        .sign(|hash| {
            Ok::<secp256k1::ecdsa::RecoverableSignature, Infallible>(
                Secp256k1::new().sign_ecdsa_recoverable(hash, &private_key),
            )
        })
        .unwrap();

    println!("{invoice}");

    assert!(invoice.to_string().starts_with("lnbc1"));
}
