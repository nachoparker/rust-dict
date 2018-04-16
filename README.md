# rust-dict
Exercise crate implementing real associative arrays, also known as dictionaries.

This is a toy example. In practice better use std::collections::HashMap

```
use dict::{ Dict, DictIface };

//create a dictionary of strings
let mut dict = Dict::<String>::new();
assert_eq!( dict.is_empty(), true );
assert_eq!( dict.len(), 0 );

// add an element "val" indexed by the key "key"
assert_eq!( dict.add( "key".to_string(), "val".to_string() ), true );
assert_eq!( dict.is_empty(), false );
assert_eq!( dict.len(), 1 );

// keys must be unique
assert_eq!( dict.add( "key".to_string()      , "other_val".to_string() ), false );
assert_eq!( dict.len(), 1 );
assert_eq!( dict.add( "other_key".to_string(), "other_val".to_string() ), true );
assert_eq!( dict.len(), 2 );

// we can iterate just like an array
for o in &dict {
    println!( "{} - {}", o.key, o.val );
}
dict.iter().for_each( |o| println!( "{} - {}", o.key, o.val ) );

// we can access the value by its key string with get()
assert_eq!( dict.get( "key" ).unwrap(), "val" );
assert_eq!( dict.contains_key( "key" ), true );
assert_eq!( dict.remove_key( "key" ).unwrap(), "val" );
assert_eq!( dict.contains_key( "key" ), false );
```

[ownyourbits.com](https://ownyourbits.com)

[crates.io](https://crates.io/crates/dict)
