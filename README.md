# json_minimal

A minimal json crate conforming to https://www.ecma-international.org/publications/files/ECMA-ST/ECMA-404.pdf .

## Tutorial (creating jsons)

In order to create a valid (i.e. generally accepted) json you should always start with:
```rust
    use json_minimal::*;

    let mut json = Json::new();
    // which is equivalent to
    let mut json = Json::JSON(Vec::new());
    // ...
```

To add an object, simply do this:
```rust
    // ...
    let greeting = 
        Json::OBJECT {
            name: String::from("Greeting"),

            value: Box::new(
                Json::STRING( String::from("Hello, world!") )
            )
        }
    ;

    json.add(greeting);
    // ...
```
or alternatively:
```rust
    // ...
    json.add(
        Json::OBJECT {
            name: String::from("Greeting"),

            value: Box::new(
                Json::STRING( String::from("Hello, world!") )
            )
        }
    );
    // ...
```

As you can see, whilst the crate is minimal (in my opinion) it may not be the quickest to work with. This becomes clearer when adding an array to an object:
```rust
    // ...

    let mut days_in_the_week =
        Json::OBJECT {
            name: String::from("Days of the week"),

            value: Box::new(
                Json::JSON(Vec::new())
            )
        }
    ;

    let mut days = Json::ARRAY(Vec::new());

    days
        .add(
            Json::STRING( String::from("Monday") )
        )
        .add(
            Json::STRING( String::from("Tuesday") )
        )
        .add(
            Json::STRING( String::from("Wednesday") )
        )
        .add(
            Json::STRING( String::from("Thursday") )
        )
        .add(
            Json::STRING( String::from("Friday") )
        )
        .add(
            Json::STRING( String::from("Saturday") )
        )
        .add(
            Json::STRING( String::from("Sunday") )
        )
    ;

    days_in_the_week
        .add(
            Json::OBJECT {
                name: String::from("Total number of days"),

                value: Box::new(
                    Json::NUMBER(7.0) // Accepts `f64`
                )
            }
        )
        .add(
            Json::OBJECT {
                name: String::from("They are called"),

                value: Box::new(
                    days
                )
            }
        )
    ;

    json.add(days_in_the_week);
    // ...
```

In conclusion:
```rust
    // ...

    let mut conclusion =
        Json::OBJECT {
            name: String::from("Conclusion"),

            value: Box::new(
                Json::JSON(Vec::new())
            )
        }
    ;

    conclusion
        .add(
            Json::OBJECT {
                name: String::from("Minimal in my opinion"),

                value: Box::new(
                    Json::BOOL(true)
                )
            }
        )
        .add(
            Json::OBJECT {
                name: String::from("How much I care about your opinion"),

                value: Box::new(
                    Json::NULL
                )
            }
        )
        .add(
            Json::OBJECT {
                name: String::from("Comment"),

                value: Box::new(
                    Json::STRING( String::from(";)") )
                )
            }
        )
    ;

    json.add(conclusion);
    // ...
```

Calling:
```rust
    // ...
    let resulting_json = json.print();
```
will result in a `String` containing:
`{"Greeting":"Hello, world!","Days of the week":{"Total number of days":7,"They are called":["Monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"]},"Conclusion":{"Minimal in my opinion":true,"How much I care about your opinion":null,"Comment":";)"}}`

## Tutorial (parsing and working with jsons)

Parsing a json value from bytes is even more minimal - at the cost of being more cumbersome. Let's see how we can parse the json we generated above. You must be sure that the json you are about to parse is in the same format as this here (no things like '\n', '\r', ' ' etc. inbetween):
```rust
    use json_minimal::*;

    let mut json = Vec::new();

    match Json::parse(b"{\"Greeting\":\"Hello, world!\",\"Days of the week\":{\"Total number of days\":7,\"They are called\":[\"Monday\",\"Tuesday\",\"Wednesday\",\"Thursday\",\"Friday\",\"Saturday\",\"Sunday\"]},\"Conclusion\":{\"Minimal in my opinion\":true,\"How much I care about your opinion\":null,\"Comment\":\";)\"}}") {
        Ok(parsed_json) => {
            match parsed_json {
                Json::JSON(parsed_values) => {
                    json = parsed_values;
                },
                _ => {
                    panic!("Oh no! What happened?");
                }
            }
        },
        Err( (position,message) ) => {
            panic!("`{}` at position `{}`!!!");
        }
    }
    // ...
```

Let's first talk about what information is given for a parsing error. As you might expect it is minimal. `position` above is the position were everything went wrong and the `message` will be something like`"Error parsing array."` if, for example, a closing `]` is missing somewhere. Continuing where we left off:
```rust
    // ...
    let json = Json::JSON( json ); // It's called tautology, I think

    match json.get("Greeting") {
        Some(json) => {
            match json {
                Json::OBJECT { name: _, value } => {
                    match value.unbox() {
                        Json::STRING(val) => {
                            assert_eq!("Hello, world!",val);
                        },
                        json => {
                            panic!("Expected Json::STRING but found {:?}",json);
                        }
                    }
                },
                json => {
                    panic!("Expected Json::JSON but found {:?}!!!",json)
                }
            }
        },
        None => {
            panic!("Couln't find Greeting. How rude!");
        }
    }
    // ...
```
Unfortunately all of this was necessary because, even though we were able to confirm that `"Greeting"` exists, we had no way of knowing what it really is. It could have been a standalone string value, after all (although nobody does that, i think). It's not over:
```rust 
    // ...
    match json.get("Days of the week") { // Hint: You can also use `get_mut` to aid in editing/creating jsons...
        Some(json) => {
            match json {
                Json::OBJECT { name: _, value } => {
                    match value.unbox() {
                        Json::JSON(values) => {
                            assert_eq!(values.len(),2);

                            match &values[0] {
                                Json::OBJECT { name, value: _ } => {
                                    assert_eq!("Total number of days",name);
                                },
                                json => {
                                    panic!("Expected Json::OBJECT but found {:?}!!!",json);
                                }
                            }

                            match &values[1] {
                                Json::OBJECT { name, value: _ } => {
                                    assert_eq!("They are called",name);
                                },
                                json => {
                                    panic!("Expected Json::OBJECT but found {:?}!!!",json);
                                }
                            }

                        },
                        json => {
                            panic!("Expected Json::JSON but found {:?}!!!",json);
                        }
                    }
                },
                json => {
                    panic!("Expected Json::OBJECT but found {:?}!!!",json);
                }
            }
        },
        None => {
            panic!("Days of the week not found!");
        }
    }
    // You get the idea.
```
The function `Json::parse(...)` can also parse 'standalone values'. Example:

```rust
    match Json::parse("\"What's up?\"") {
        Ok(json) => {
            match json {
                Json::STRING(val) => {
                    assert_eq!("What's up?",val);
                },
                json => {
                    panic!("Expected Json::STRING but found {:?}!!!",json);
                }
            }
        },
        Err( (position,message) ) => {
            panic!("`{}` at position `{}`.");
        }
    }

    // Another example:

    match Json::parse("[1,2,3,\"four\"]") {
        Ok(json) => {
            match json {
                Json::ARRAY(val) => {
                    assert_eq!(val.len(),4);
                },
                json => {
                    panic!("Expected Json::ARRAY but found {:?}!!!",json);
                }
            }
        },
        Err( (position,message) ) => {
            panic!("`{}` at position `{}`.");
        }
    }
```
Please let me know if you experience any issues.