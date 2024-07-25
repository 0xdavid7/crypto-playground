#[cfg(test)]
mod test {
    use ring::hmac;

    //RUST_LOG=debug cargo test -- --nocapture
    #[test]
    fn test_hmac() {
        let raw_key = b"cryptography";

        let key = hmac::Key::new(hmac::HMAC_SHA384, raw_key);

        let msg = "hello";

        let tag = hmac::sign(&key, msg.as_bytes());

        println!("{:?}", tag.as_ref());

        let output = "83d1c3d3774d8a32b8ea0460330c16d1b2e3e5c0ea86ccc2d70e603aa8c8151d675dfe339d83f3f495fab226795789d4";

        assert_eq!(output, hex::encode(tag.as_ref()));
    }
}
