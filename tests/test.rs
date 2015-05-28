extern crate libmarisa as marisa;

#[test]
fn test_keyset() {
    let mut keyset = marisa::KeySet::new();
    let keys = vec!("hello", "world");
    let keys_len = keys.len();

    for key in keys {
        keyset.push(key, 0);
    }
    let len = keyset.num_keys();
    assert!(len == keys_len);
}
