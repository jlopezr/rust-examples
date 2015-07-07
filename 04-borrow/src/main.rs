struct Person {  
    first: String,
    last: String
}

fn hello() {  
    let yehuda = Person {
        first: "Yehuda".to_string(),
        last: "Katz".to_string()
    };

    // `yehuda` is transferred to `name_size`, so it cannot be
    // used anymore in this function, and it will not be destroyed
    // when this function returns. It is up to `name_size`,
    // or possibly a future owner, to destroy it.
    let _size = name_size(yehuda);

    let _tom = Person {
        first: "Tom".to_string(),
        last: "Dale".to_string()
    };

    // `tom` wasn't transferred, so it will be
    // destroyed when this function returns.
}

fn name_size(person: Person) -> usize {  
    let Person { first, last } = person;
    first.len() + last.len()

    // this function owns Person, so the Person is destroyed when `name_size` returns
}

fn main() {
    hello();
}
