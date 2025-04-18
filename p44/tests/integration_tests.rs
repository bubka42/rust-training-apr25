#[test]
fn test_hash_map_macro() {
    let map = p44::hash_map! (
        "key1" => "value1",
        "key2" => "value2",
    );
    assert_eq!(map.get("key1"), Some(&"value1"));
    assert_eq!(map.get("key2"), Some(&"value2"));
}

#[test]
fn test_empty_hash_map_macro() {
    let map: std::collections::HashMap<&str, &str> = p44::hash_map! {};
    assert!(map.is_empty());
}
