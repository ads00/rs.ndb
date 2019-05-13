#![feature(custom_attribute)]


use ndb;

#[concept(storable copyable)]
struct Struct
{
    zeta:i64,
}

#[zetatest]
struct Struct2;





fn main()
{
    //use std::vec;
    print!("EXP MAIN{}", ndb::test2());

    let a : Vec<usize> =  Vec::new();



    ndb::test();

    ndb::query();
}