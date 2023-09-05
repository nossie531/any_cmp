use any_cmp::AnyOrd;
use std::collections::BTreeMap;

#[test]
fn test() {
    let mut map = BTreeMap::<Box<dyn AnyOrd>, String>::new();
    map.insert(Box::new(false), "bool".to_string());
    map.insert(Box::new(0), "int".to_string());
    map.insert(Box::new(""), "string".to_string());

    assert_eq!(map[&false as &dyn AnyOrd], "bool");
    assert_eq!(map[&0 as &dyn AnyOrd], "int");
    assert_eq!(map[&"" as &dyn AnyOrd], "string");
}
