// SPDX-License-Identifier: MIT
// Copyright 2026 IROX Contributors
//

use crate::scanner::{LineEnding, QuotedChars, ReadToken, Scanner, Token};
use irox_bits::Error;
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Sep {
    Equal,
    LineEnding(LineEnding),
    Export,
    Comment,
}
pub fn load_dotenv_from_cwd() -> Result<BTreeMap<String, String>, Error> {
    let Ok(cwd) = std::env::current_dir() else {
        println!("no cwd");
        return Ok(Default::default());
    };
    let p = cwd.join(".env");
    load_dotenv(p)
}
fn check_and_replace(haystack: String, needles: &BTreeMap<String, String>) -> String {
    for (k, v) in needles {
        let search = format!("${{{k}}}");
        if haystack.contains(&search) {
            return haystack.replace(&search, v);
        }
    }
    haystack
}
pub fn load_dotenv<P: AsRef<Path>>(path: P) -> Result<BTreeMap<String, String>, Error> {
    let p = path.as_ref();
    let data = std::fs::read(p)?;
    let wrap = data.as_slice();

    let mut parser = Scanner::new(
        wrap,
        &[
            Token::new("=", Sep::Equal).with_quote_char(QuotedChars::SingleOrDoubleQuotes),
            Token::new("\r", Sep::LineEnding(LineEnding::CarriageReturn))
                .with_quote_char(QuotedChars::SingleOrDoubleQuotes),
            Token::new("\n", Sep::LineEnding(LineEnding::LineFeed))
                .with_quote_char(QuotedChars::SingleOrDoubleQuotes),
            Token::new("\r\n", Sep::LineEnding(LineEnding::CarriageReturnLineFeed))
                .with_quote_char(QuotedChars::SingleOrDoubleQuotes),
            Token::new("#", Sep::Comment).with_quote_char(QuotedChars::SingleOrDoubleQuotes),
            Token::new("export ", Sep::Export)
        ],
    );
    let mut varbls = BTreeMap::<String, String>::new();
    let mut key = Option::<String>::None;

    macro_rules! check_add {
        ($key:ident,$data:ident,$varbls:expr) => {
            if let Some(key) = $key.take() {
                let va = String::from_utf8_lossy($data.as_slice()).to_string();
                let va = va
                    .trim_start_matches('"')
                    .trim_start_matches('\'')
                    .trim_end_matches('"')
                    .trim_end_matches('\'')
                    .trim()
                    .to_string();
                let va = check_and_replace(va, &$varbls);
                $varbls.insert(key, va);
            }
        };
    }

    while let Ok(result) = parser.read_next() {
        match result {
            ReadToken::Found { data, token } => match token.get_response() {
                Sep::Equal => {
                    key = Some(String::from_utf8_lossy(data.as_slice()).trim().to_string());
                }
                Sep::Comment => {
                    check_add!(key, data, varbls);
                    key = None;
                }
                Sep::LineEnding(_) => {
                    check_add!(key, data, varbls);
                }
                Sep::Export => {
                    key = None;
                }
            },
            ReadToken::NotFound => {
                break;
            }
            ReadToken::EndOfData { data } => {
                check_add!(key, data, varbls);
                break;
            }
        }
    }
    Ok(varbls)
}

pub fn set_envs(varbls: &BTreeMap<String, String>) -> Result<(), Error> {
    for (key, value) in varbls {
        std::env::set_var(key, value);
    }
    Ok(())
}

pub fn load_dotenv_and_set<P: AsRef<Path>>(path: P) -> Result<(), Error> {
    let varbls = load_dotenv(path)?;
    set_envs(&varbls)
}
pub fn load_dotenv_from_cwd_and_set() -> Result<(), Error> {
    let varbls = load_dotenv_from_cwd()?;
    set_envs(&varbls)
}

#[cfg(test)]
mod tests {
    use crate::dotenv::load_dotenv;
    use irox_bits::Error;

    macro_rules! check {
        ($key:literal, $val:literal,$iter:ident) => {
            assert_eq!(Some((&$key.to_string(), &$val.to_string())), $iter.next());
        };
    }
    #[test]
    pub fn test() -> Result<(), Error> {
        let varbls = load_dotenv("tests/.env")?;
        println!("{varbls:#?}");
        assert_eq!(varbls.len(), 23);
        let mut iter = varbls.iter();

        check!("BASE", "base", iter);
        check!("BASE_URL", "https://api.example.com", iter);
        check!("DATABASE_POOL_SIZE", "10", iter);
        check!("DATABASE_URL", "postgres://user:pass@localhost:5432/mydb", iter);
        check!("DOLLAR", "base-suffix", iter);
        check!("DOUBLE", "double value", iter);
        check!("DUP", "second", iter);
        check!("EMPTY", "", iter);
        check!("ENABLE_CACHE", "true", iter);
        check!("EXPORTED", "exported", iter);
        check!("FULL_URL", "https://api.example.com/v1/users", iter);
        check!("HASH_JOINED", "alpha", iter);
        check!("HASH_QUOTED", "alpha # beta", iter);
        check!("HASH_SPACED", "alpha", iter);
        check!("MULTI", "line one\r\nline two", iter);
        check!("NODE_ENV", "development", iter);
        check!("PORT", "3000", iter);
        check!("PRIVATE_KEY", "-----BEGIN RSA KEY-----\r\nMIIBogIBAAJBALRi...\r\n-----END RSA KEY-----", iter);
        check!("SENDGRID_API_KEY", "SG.xxxx", iter);
        check!("SINGLE", "single value", iter);
        check!("SPACE", "spaced value", iter);
        check!("STRIPE_SECRET_KEY", "sk_test_abc123", iter);
        check!("VALID", "after", iter);
        assert_eq!(iter.next(), None);

        Ok(())
    }
}
