use any_cmp::ObjHash;
use std::collections::HashMap;

#[test]
fn test() {
    let mut map = HashMap::<Box<dyn ObjHash>, String>::new();
    map.insert(Box::new(false), "bool".to_string());
    map.insert(Box::new(0), "int".to_string());
    map.insert(Box::new(""), "string".to_string());

    assert_eq!(map[&false as &dyn ObjHash], "bool");
    assert_eq!(map[&0 as &dyn ObjHash], "int");
    assert_eq!(map[&"" as &dyn ObjHash], "string");
}
