any_cmp
===

Support dynamic type comparisons.

*The author of this crate is not good at English.*  
*Forgive me if the document is hard to read.*

## What is this?

This crate provides traits that are comparable and can be dynamic types.

Mainly includes the following items.

* AnyEq - Like `std::cmp::Eq`, but this trait can be dynamic.
* AnyHash - Like `std::hash::Hash`, but this trait can be dynamic.
* AnyOrd - Like `std::cmp::Ord`, but this trait can be dynamic.
* AnyPartialEq - Like `std::cmp::PartialEq`, but this trait can be dynamic.
* AnyPartialOrd - Like `std::cmp::PartialOrd`, but this trait can be dynamic.
* ObjHash - Trait that combines `AnyEq` and `AnyHash`.

## Examples

Here is an example simple but useless.

```rust
let x = &42 as &dyn AnyEq;
let y = &42 as &dyn AnyEq;
let z = &"42" as &dyn AnyEq;
assert!(x == y);
assert!(x != z);
```

Here is an example of using dynamic types for HashMap keys.

```rust
let mut map = HashMap::<Box<dyn ObjHash>, String>::new();
map.insert(Box::new(false), "bool".to_string());
map.insert(Box::new(0), "int".to_string());
map.insert(Box::new(""), "string".to_string());

assert_eq!(map[&false as &dyn ObjHash], "bool");
assert_eq!(map[&0 as &dyn ObjHash], "int");
assert_eq!(map[&"" as &dyn ObjHash], "string");
```
