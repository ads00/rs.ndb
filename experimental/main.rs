#![feature(custom_attribute)]


use ndb;

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



fn main()
{
    print!("EXP MAIN{}", answer2());

    ndb::test();

    ndb::query();
}