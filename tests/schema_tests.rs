#[cfg(test)]
mod tests {
    use gradecoin::schema::*;
    use serde_json::error::Error;
    use serde_test::{assert_tokens, Token};
    use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

    #[test]
    fn claims_serialize_correctly() {
        let claims = Claims {
            tha: "hashed_string".to_owned(),
            iat: 0,
            exp: 100,
        };
        assert_tokens(
            &claims,
            &[
                Token::Struct{name: "Claims", len: 3},
                Token::String("tha"),
                Token::String("hashed_string"),
                Token::String("iat"),
                Token::U64(0),
                Token::String("exp"),
                Token::U64(100),
                Token::StructEnd,
            ]
        )
    }

    #[test]
    fn claims_deserialize_correctly() {
        let data = r#"{"tha":"hashed_string","iat":0,"exp":100}"#;
        let claims: Claims = serde_json::from_str(data).unwrap();
        let expected_claims = Claims {
            tha: "hashed_string".to_owned(),
            iat: 0,
            exp: 100,
        };
        assert_eq!(claims, expected_claims);
    }

    #[test]
    fn transaction_serialize_correctly() {
        let transaction = Transaction {
            by: "source".to_owned(),
            source: "source".to_owned(),
            target: "target".to_owned(),
            amount: 0,
            timestamp: NaiveDate::from_ymd(2021, 4, 2).and_hms(4, 2, 42),
        };

        assert_tokens(
            &transaction,
            &[
                Token::Struct{name: "Transaction", len: 5},
                Token::String("by"),
                Token::String("source"),
                Token::String("source"),
                Token::String("source"),
                Token::String("target"),
                Token::String("target"),
                Token::String("amount"),
                Token::I32(0),
                Token::String("timestamp"),
                Token::String("2021-04-02T04:02:42"),
                Token::StructEnd,
            ]
        )
    }

    #[test]
    fn transaction_deserialize_correctly() {
        let data = r#"{"by":"source","source":"source","target":"target","amount":0,"timestamp":"2021-04-02T04:02:42"}"#;
        let transaction: Transaction = serde_json::from_str(data).unwrap();
        let expected_transaction = Transaction {
            by: "source".to_owned(),
            source: "source".to_owned(),
            target: "target".to_owned(),
            amount: 0,
            timestamp: NaiveDate::from_ymd(2021, 4, 2).and_hms(4, 2, 42),
        };

        assert_eq!(transaction, expected_transaction);
    }

    #[test]
    fn block_serialize_correctly() {

    }

    #[test]
    fn block_deserialize_correctly() {

    }

    #[test]
    fn block_deserialize_when_vec_emptpy() {

    }

    #[test]
    fn naked_block_serialize_correctly() {

    }

    #[test]
    fn naked_block_deserialize_correctly() {

    }

    #[test]
    fn naked_block_deserialize_when_vec_emptpy() {

    }

    #[test]
    fn user_serialize_correctly() {

    }

    #[test]
    fn user_deserialize_correctly() {

    }

    #[test]
    fn metu_id_serialize_correctly() {

    }

    #[test]
    fn metu_id_deserialize_correctly() {

    }

    #[test]
    fn auth_request_serialize_correctly() {

    }

    #[test]
    fn auth_request_deserialize_correctly() {

    }














}