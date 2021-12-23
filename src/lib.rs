//!
//! Crate implementing real associative arrays, also known as dictionaries.
//!
//! Associative arrays behave just like indexed arrays, but use unique strings as indexes instead
//! of integers.
//!
//! This associative array implementation is built as a Trait implementation over std::vec::Vec, so
//! all [Vec methods](https://doc.rust-lang.org/std/vec/struct.Vec.html) are also available 
//! for a Dict object.
//!
//! Insert time is O(n²), and retrieval time is O(log n) based on key hashing. This means that it
//! is far more efficient to query values than to insert them. If we need frequent inserts in big
//! sets, it can be more efficient to implement a solution based on linked lists or binary heaps.
//!
//! This crate is created as an exercise in extending Vec. It is more efficient to use
//! [std::collections::HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
//!
//! # Examples
//!
//! ```
//! use dict::{ Dict, DictIface };
//!
//! //create a dictionary of strings
//! let mut dict = Dict::<String>::new();
//! assert_eq!( dict.is_empty(), true );
//! assert_eq!( dict.len(), 0 );
//!
//! // add an element "val" indexed by the key "key"
//! assert_eq!( dict.add( "key".to_string(), "val".to_string() ), true );
//! assert_eq!( dict.is_empty(), false );
//! assert_eq!( dict.len(), 1 );
//!
//! // keys must be unique
//! assert_eq!( dict.add( "key".to_string()      , "other_val".to_string() ), false );
//! assert_eq!( dict.len(), 1 );
//! assert_eq!( dict.add( "other_key".to_string(), "other_val".to_string() ), true );
//! assert_eq!( dict.len(), 2 );
//!
//! // we can iterate just like an array
//!  for o in &dict {
//!      println!( "{} - {}", o.key, o.val );
//!  }
//!  dict.iter().for_each( |o| println!( "{} - {}", o.key, o.val ) );
//!
//!
//! // we can access the value by its key string with get()
//! assert_eq!( dict.get( "key" ).unwrap(), "val" );
//! assert_eq!( dict.contains_key( "key" ), true );
//! assert_eq!( dict.remove_key( "key" ).unwrap(), "val" );
//! assert_eq!( dict.contains_key( "key" ), false );
//! assert_eq!( dict.len(), 1 );
//! ```
//!
//! # More information
//!
//! Copyleft 2018 by Ignacio Nunez Hernanz - nacho _at_ ownyourbits _dot_ com
//!
//! GPL licensed
//!
//! More at [OwnYouBits](https://ownyourbits.com)
//!

use std::fmt;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub struct DictEntry<T> { hash : u64, pub key : String, pub val : T }

impl<T> fmt::Debug for DictEntry<T>
where T: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{\"{}\": {:?}}}", self.key, self.val)
    }
}

pub type Dict<T> = Vec<DictEntry<T>>;

pub trait DictIface<T> {
    fn add( &mut self, key : String, val : T ) -> bool;
    fn get( &self, key : &str ) -> Option<&T>;
    fn contains_key( &self, key : &str ) -> bool;
    fn remove_key( &mut self, key : &str ) -> Option<T>;
}

impl<T> DictIface<T> for Dict<T> {
    /// Add an element _val_ of type T, indexed by the string _key_. Returns false if the key
    /// exists or there is a hash collision
    fn add( &mut self, key : String, val : T ) -> bool {
        match self.binary_search_by_key( &hash_f(&key), |o| o.hash ) {
            Ok (  _  ) => return false,   // key exists or hash collision
            Err( pos ) => self.insert( pos, DictEntry{ hash: hash_f( &key ) , key, val } ),
        }
        true
    }
    /// Remove the element identified by the key _key_ and return it, if exists.
    fn remove_key( &mut self, key : &str ) -> Option<T> {
        if let Ok( pos ) = self.binary_search_by_key( &hash_f(key), |o| o.hash ) {
            let entry = self.remove( pos );
            Some( entry.val )
        } else { None }
    }
    /// Return a reference to the value identified by the key _key_, if exists.
    fn get( &self, key : &str ) -> Option<&T> {
        if let Ok( pos ) = self.binary_search_by_key( &hash_f(key), |o| o.hash ) {
            Some( &self[pos].val )
        } else { None }
    }
    /// Return true if an element identified by the key _key_ exists.
    fn contains_key( &self, key : &str ) -> bool {
        self.get( key ).is_some()
    }
}

impl<T> Hash for DictEntry<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}

fn hash_f<T>(obj: T) -> u64
where
    T: Hash,
{
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish()
}

// License
//
// This script is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// This script is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this script; if not, write to the
// Free Software Foundation, Inc., 59 Temple Place, Suite 330,
