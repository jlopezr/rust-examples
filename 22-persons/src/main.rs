//https://dev.to/buntine/aha-understanding-lifetimes-in-rust

// Our Person struct consists of a name and an optional reference to another Person.
#[derive(Debug, Eq, PartialEq)]
struct Person<'a> {
    name: &'static str,
    parent: Option<&'a Person<'a>>,
}

impl<'a> Person<'a> {
    fn new(name: &'static str, parent: Option<&'a Person<'a>>) -> Person<'a> {
        Person{name: name, parent: parent}
    }

    // Rust lets us elide the lifetime of &self here.
    fn parents_name(&self) -> Option<&'static str> {
        self.parent.and_then(|p| Some(p.name))
    }
}

fn main() {
    let jane = Person::new("Jane", None);
    let tom = Person::new("Tom", Some(&jane));

    println!("{:?}", jane);
    println!("{:?}", tom);

    assert_eq!(tom.parent.unwrap().name, "Jane");
    assert_eq!(tom.parents_name(), Some("Jane"));
    assert_eq!(jane.parent, None);
    assert_eq!(jane.parents_name(), None);
}
