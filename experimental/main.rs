#![feature(custom_attribute)]

use ndb;
use std::string::ToString;


struct Field<T>
{
    f_01: T
}

#[concept(storable)]
struct Movie
{
    test: Field<i32>,
    id:i32,
    name:String,
}

impl Movie
{
    fn save(&self)
    {
        println!("save {}", self.name);
    }
}


#[derive(Default)]
struct Query
{
    data: String,
}

struct Get;
const get_ : Get = Get{};

fn get() -> Get { get_ }

impl Query
{
    fn test(&mut self, expr: Get) -> &mut Self
    {
        self.data.push_str(&String::from("test()"));

        self
    }

    fn get(&mut self) -> &mut Self
    {
        self.data.push_str(&String::from("get()"));

        self
    }

    fn source(&mut self) -> &mut Self
    {
        self.data.push_str(&String::from("source()"));

        self
    }

    fn output(&mut self) -> &String
    {
        &self.data
    }
}




struct void_;
impl void_ { fn eval(&self){} }

struct add_;
impl add_ { fn eval(&self){} }

struct value_<T> { v: T }
impl<T> value_<T> { fn eval(&self){} }

struct expr_<E, T = void_, U = void_>(E, T, U);

impl<E, T, U> expr_<E, T, U>
{
    fn eval(&self)
    {
        self.1.eval();
        self.2.eval();
    }
}

fn add<T, U>(t: T, u: U) -> expr_<add_, T, U>
{
     expr_(add_{}, t, u)
}

fn main()
{
    let t = Field{ f_01 : 0i32 };
    let m = Movie{ test: t, id: 0i32, name: "name".to_string() };

    let mut q = Query::default();
    let mut q2 = Query::default();

    let mut q = Query::default();


    let v1 = expr_(value_{ v: 0 }, void_{}, void_{});
    let v2 = expr_(value_{ v: 0 }, void_{}, void_{});

    add(v1, v2);

    //f(add_{}, 5, 2);


    //println!("G1 {}", q.output());

    //println!("G2 {}", q.output());

    //ndb::query().get(m.name).source(m).filter( equal(m.name, value) );

    m.save();
}