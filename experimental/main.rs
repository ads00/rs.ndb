#![feature(custom_attribute)]

extern crate ndb_macro;
use ndb_macro::storable;
use ndb_macro::zetatest;



#[concept(storable copyable)]
struct Struct
{
    zeta:i64,
}

#[zetatest]
struct Struct2
{
    zeta:i64,
}



fn main() {
    print!("{}", answer2());
}