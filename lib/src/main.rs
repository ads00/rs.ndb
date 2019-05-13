#![feature(custom_attribute)]


pub use ndb_macro::*;

    pub fn test()
    {
        print!("test");
    }

    pub fn query()
    {
        let connection = sqlite::open("test.db").unwrap();

        connection
            .execute(
                "
                CREATE TABLE IF NOT EXISTS users (name TEXT, age INTEGER);
                INSERT INTO users (name, age) VALUES ('Alice', 42);
                INSERT INTO users (name, age) VALUES ('Bob', 69);
                ",
            )
            .unwrap();
    }




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
