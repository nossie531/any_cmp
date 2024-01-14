any_cmp
===

Support dynamic type comparisons.

*The author of this crate is not good at English.*  
*Forgive me if the document is hard to read.*

## What is this?

This crate provides traits that are comparable and can be dynamic types.

Mainly includes the following items.

| name            | summary                      |
| -               | -                            |
| `AnyEq`         | Like `std::cmp::Eq`.         |
| `AnyHash`       | Like `std::hash::Hash`.      |
| `AnyOrd`        | Like `std::cmp::Ord`.        |
| `AnyPartialEq`  | Like `std::cmp::PartialEq`.  |
| `AnyPartialOrd` | Like `std::cmp::PartialOrd`. |
| `ObjHash`       | Trait that combines `AnyEq` and `AnyHash`. |

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

## What's New

v0.4.1
* Minor refactoring.

v0.4.0
* Fix forgotten implementation.
* Add box upcasting methods (ex: `as_any_eq_box`).

v0.3.0

* `must_use` annotations are added at several locations.

v0.2.0

* Module `upcast` specs have slightly changed.
