#![allow(unused)]
#![feature(collections)]
#![feature(core)]
#![feature(io)]
use std::path::Path;
use std::old_io as io;

mod grammar;

#[derive(PartialEq, Show)]
pub enum Identifyer{
    Url(String),
    Relative(String),
    NoID
}
#[derive(PartialEq, Show)]
enum Statement{
    Directive(Directive),
    Triple(Triple)
}
#[derive(PartialEq, Show)]
pub struct Triple{
    subject: Identifyer,
    predicate: Identifyer,
    object: Identifyer
}
#[derive(PartialEq, Show)]
enum Literal{
    RDF(String),
    Number(f64),
    Boolean(bool)
}
#[derive(PartialEq, Show)]
enum Object{
    Literal(Literal),
    URI(String),
    Collection,
    BlankNodePropertyList,
    Blank
}

#[derive(PartialEq, Show)]
enum Directive{
    PrefixID(String, String),
    Base(String),
    SparqlPrefix(String),
    SparqlBase(String)
}

///the internal representation of a trutle document
#[derive(PartialEq, Show)]
pub struct TurtleDoc{
    statements: Vec<Statement>
}

impl TurtleDoc{
    pub fn new(s: &str) -> Result<TurtleDoc, String>{
        grammar::turtleDoc(s)
    }
    ///read a turtle document form a file, the file will be opend as read only
    pub fn from_file(path: &Path) -> io::IoResult<TurtleDoc>{
        let mut f = try!(io::File::open(path));
        let r = try!(f.read_to_string());
        Ok(grammar::turtleDoc(r.as_slice()).unwrap())
    }

    pub fn set(&mut self, subject: String, predicate: String, object: String){
        unimplemented!()
    }

    pub fn query(&self, query: Query){
        unimplemented!()
    }

    pub fn is_trible(&self, subject: String, predicate: String, object: String) -> bool{
        false
    }
}

///Type representing a query for a turtle document
pub enum Query{
    ///get an entry by(id)
    ById(String),
    ///get many ids
    ByIds(Vec<String>),
    ///fetch the object with (subject, predicate)
    GetObject(Box<Query>, Box<Query>),
    ///fetch data where (subject, predicate, object)
    GetWhere(Box<Query>, Box<Query>, Box<Query>),
    ///a value
    Number(f64),
    String(String)
}

//TODO: make better error handling

#[test]
fn test_parser_methods() {
    let fp = &Path::new("examples/ex1.ttl");
    let r = TurtleDoc::from_file(fp).unwrap();
    println!("{:?}", r);
}

#[test]
fn test_directive() {
    let e = grammar::directive("@base <http://example.org/> .");
    assert_eq!(e, Ok(
        Directive::Base("http://example.org/".to_string())
    ));
    let e = grammar::directive("@prefix ex: <http://example.org/>.");
    assert_eq!(e, Ok(
        Directive::PrefixID("ex".to_string(),"http://example.org/".to_string())
    ));
}

#[test]
fn test_base() {
    let e = grammar::base("@base <http://example.org/> .");
    assert_eq!(e, Ok(
        "http://example.org/".to_string()
    ));
}

#[test]
fn test_triples() {
    let e = grammar::triples("<http://example.org/> <http://example.org/> <http://example.org/> .");
    assert_eq!(e, Ok(
        Triple{
            subject: Identifyer::Url("http://example.org".to_string()),
            predicate: Identifyer::Url("http://example.org".to_string()),
            object: Identifyer::Url("http://example.org".to_string()),
        }
    ));
}

#[test]
fn test_prefix() {
    let e = grammar::prefixID("@prefix ex: <http://example.org/> .");
    assert_eq!(e, Ok(
        ("ex".to_string(), "http://example.org/".to_string())
        ));
    }

#[test]
fn test_iref(){
    let e = grammar::IRIREF("<http://example.org/>");
    assert_eq!(e, Ok(
        "http://example.org/".to_string()
    ));
}
