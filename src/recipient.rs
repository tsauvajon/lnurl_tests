use std::str::FromStr;

use bech32::FromBase32;

const EXAMPLE_LN_URL: &str = "lnurl1dp68gurn8ghj7ampd3kx2ar0veekzar0wd5xjtnrdakj7tnhv4kxctttdehhwm30d3h82unvwqhhg6tdv438yctwvscrxhnh4nf";

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum LightningRecipient {
    LightningAddress { domain: String, username: String },
    LnUrl(String),
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum ParseRecipientError {
    TooManyAtSigns,
    NoRecipientFound,
}

impl FromStr for LightningRecipient {
    type Err = ParseRecipientError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("lnurl") {
            return Ok(Self::LnUrl(s.to_string()));
        }

        if s.contains('@') {
            let mut parts = s.split('@');
            if parts.clone().count() != 2 {
                return Err(ParseRecipientError::TooManyAtSigns);
            }

            return Ok(Self::LightningAddress {
                username: parts.next().unwrap().to_string(),
                domain: parts.next().unwrap().to_string(),
            });
        }

        Err(ParseRecipientError::NoRecipientFound)
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

    assert_eq!(
        Err(ParseRecipientError::TooManyAtSigns),
        LightningRecipient::from_str("hello@world@com")
    );
    assert_eq!(
        Err(ParseRecipientError::NoRecipientFound),
        LightningRecipient::from_str("helloworld.com")
    );
}

impl LightningRecipient {
    pub(crate) fn decode_url(&self) -> String {
        match self {
            LightningRecipient::LnUrl(encoded) => {
                let (_hrp, data, _variant) = bech32::decode(encoded).unwrap();
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

pub(crate) fn decode_recipient() -> String {
    let recipient = LightningRecipient::from_str(EXAMPLE_LN_URL).unwrap();
    recipient.decode_url()
}
