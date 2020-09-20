#[derive(Debug)]
pub enum Json {
    OBJECT {
        name: String,
        value: Box<Json>,
    },
    JSON(Vec<Json>),
    ARRAY(Vec<Json>),
    STRING(String),
    NUMBER(f64),
    BOOL(bool),
    NULL,
}

impl Json {
    /// Construct a new `Json::JSON`
    /// ## Example
    /// ```
    /// use json_minimal::*;
    /// 
    /// let mut json = Json::new();
    /// ```
    pub fn new() -> Json {
        Json::JSON(Vec::new())
    }

    /// Add any `Json` variant to a `Json` variant of type `Json::JSON`, `Json::ARRAY`
    /// or a `Json::OBJECT` (holding a `Json::JSON`,`Json::ARRAY`,`Json::OBJECT` (holding a `Json::JSON`,`Json::`...)).
    /// ## Panics!
    /// Will panic if the conditions stated above are not met OR if an attempt is made to add a `Json::JSON` to a `Json::JSON`
    /// without wrapping it in a `Json::OBJECT` first.
    /// ## Example
    /// ```
    ///     use json_minimal::*;
    ///     
    ///     let mut json = Json::new();
    /// 
    ///     json
    ///         .add(
    ///             Json::OBJECT {
    ///                 name: String::from("Greeting"),
    /// 
    ///                 value: Box::new(
    ///                     Json::STRING( String::from("Hello, world!") )
    ///                 )
    ///             }
    ///         )
    ///     ;
    /// ```
    /// See the <a href="https://github.com/36den/json_minimal-rs/">tutorial</a> on github for more.
    pub fn add(&mut self, value: Json) -> &mut Json {
        match self {
            Json::JSON(values) => {
                match value {
                    Json::OBJECT { name, value } => {
                        values.push( Json::OBJECT { name, value } );
                    },
                    Json::JSON(_) => {
                        panic!("A `Json::JSON` may not be added to a `Json::JSON` if it is not within a `Json::OBJECT`.");
                    },
                    Json::ARRAY(vals) => {
                        values.push( Json::ARRAY(vals) );
                    },
                    Json::STRING(val) => {
                        values.push( Json::STRING(val) );
                    },
                    Json::NUMBER(val) => {
                        values.push( Json::NUMBER(val) );
                    },
                    Json::BOOL(val) => {
                        values.push( Json::BOOL(val) );
                    },
                    Json::NULL => {
                        values.push( Json::NULL );
                    }
                }
            },
            Json::OBJECT{ name: _, value: obj_val} => {
                match obj_val.unbox_mut() {
                    Json::JSON(values) => {
                        match value {
                            Json::OBJECT { name, value } => {
                                values.push( Json::OBJECT { name, value } );
                            },
                            Json::JSON(_) => {
                                panic!("A `Json::JSON` may not be added to a `Json::JSON` if it is not within a `Json::OBJECT`.");
                            },
                            Json::ARRAY(vals) => {
                                values.push( Json::ARRAY(vals) );
                            },
                            Json::STRING(val) => {
                                values.push( Json::STRING(val) );
                            },
                            Json::NUMBER(val) => {
                                values.push( Json::NUMBER(val) );
                            },
                            Json::BOOL(val) => {
                                values.push( Json::BOOL(val) );
                            },
                            Json::NULL => {
                                values.push( Json::NULL );
                            }
                        }
                    },
                    Json::ARRAY(values) => {
                        match value {
                            Json::OBJECT { name, value } => {
                                values.push( Json::OBJECT { name, value } );
                            },
                            Json::JSON(vals) => {
                                values.push( Json::JSON(vals) );
                            },
                            Json::ARRAY(vals) => {
                                values.push( Json::ARRAY(vals) );
                            },
                            Json::STRING(val) => {
                                values.push( Json::STRING(val) );
                            },
                            Json::NUMBER(val) => {
                                values.push( Json::NUMBER(val) );
                            },
                            Json::BOOL(val) => {
                                values.push( Json::BOOL(val) );
                            },
                            Json::NULL => {
                                values.push( Json::NULL );
                            }
                        }
                    },
                    json => {
                        panic!("The function `add(`&mut self`,`name: String`,`value: Json`)` may only be called on a `Json::JSON`, `Json::ARRAY` or `Json::OBJECT` holding a `Json::JSON` or `Json::ARRAY`. It was called on: {:?}",json);
                    }
                }
            },
            Json::ARRAY(values) => {
                match value {
                    Json::OBJECT { name, value } => {
                        values.push( Json::OBJECT { name, value } );
                    },
                    Json::JSON(vals) => {
                        values.push( Json::JSON(vals) );
                    },
                    Json::ARRAY(vals) => {
                        values.push( Json::ARRAY(vals) );
                    },
                    Json::STRING(val) => {
                        values.push( Json::STRING(val) );
                    },
                    Json::NUMBER(val) => {
                        values.push( Json::NUMBER(val) );
                    },
                    Json::BOOL(val) => {
                        values.push( Json::BOOL(val) );
                    },
                    Json::NULL => {
                        values.push( Json::NULL );
                    }
                }
            },
            json => {
                panic!("The function `add(`&mut self`,`name: String`,`value: Json`)` may only be called on a `Json::JSON`, `Json::ARRAY` or `Json::OBJECT` holding a `Json::JSON` or `Json::ARRAY`. It was called on: {:?}",json);
            }
        }

        self
    }

    /// Get the `Json` with the requested name if it exists.
    /// ## Panics
    /// This function will panic if called on a `Json` variant other than `Json::JSON` or `Json::OBJECT`,
    /// as only these two variants may hold `Json::OBJECT` (which has a `name` field).
    /// ## Example
    /// ```
    /// use json_minimal::*;
    /// 
    /// let mut json = Json::new();
    /// 
    /// json
    ///     .add(
    ///         Json::OBJECT {
    ///             name: String::from("Greeting"),
    /// 
    ///             value: Box::new(
    ///                 Json::STRING( String::from("Hello, world!") )
    ///             )
    ///         }
    ///     )
    /// ;
    /// 
    /// match json.get("Greeting") {
    ///     Some(json) => {
    ///         match json {
    ///             Json::OBJECT { name, value } => {
    ///                 match value.unbox() { // See `unbox()` below
    ///                     Json::STRING(val) => {
    ///                         assert_eq!("Hello, world!",val);
    ///                     },
    ///                     _ => {
    ///                         panic!("I expected this to be a `Json::STRING`!!!");
    ///                     }
    ///                 }   
    ///             },
    ///             _ => {
    ///                 panic!("This shouldn't happen!!!");
    ///             }
    ///         }
    ///     },
    ///     None => {
    ///         panic!("Not found!!!");
    ///     }
    /// }
    /// ```
    pub fn get(&self,search: &str) -> Option<&Json> {
        match self {
            Json::JSON(values) => {
                for n in 0..values.len() {
                    match &values[n] {
                        Json::OBJECT { name, value: _ } => {
                            match name == search {
                                true => {
                                    return Some(&values[n]);
                                },
                                false => {},
                            }
                        },
                        _ => {}
                    }
                }

                return None;
            },
            Json::OBJECT { name: _, value } => {
                match value.unbox() {
                    Json::JSON(values) => {
                        for n in 0..values.len() {
                            match &values[n] {
                                Json::OBJECT { name, value: _ } => {
                                    match name == search {
                                        true => {
                                            return Some(&values[n]);
                                        },
                                        false => {},
                                    }
                                },
                                _ => {}
                            }
                        }
        
                        return None;
                    },
                    json => {
                        panic!("The function `get(`&self`,`search: &str`)` may only be called on a `Json::JSON` or a `Json::OBJECT` holding a `Json::JSON`. I was called on: {:?}",json);
                    },
                }
            }
            json => {
                panic!("The function `get(`&self`,`search: &str`)` may only be called on a `Json::JSON`. I was called on: {:?}",json);
            }
        }
    }

    /// Same as `get` above, but the references are mutable. Use `unbox_mut()` (see below) with this one.
    /// ## Panics
    /// This function will panic if called on a `Json` variant other than `Json::JSON` or `Json::OBJECT`,
    /// as only these two variants may hold `Json::OBJECT` which has a `name` field.
    pub fn get_mut(&mut self, search: &str) -> Option<&mut Json> {
        match self {
            Json::JSON(values) => {
                for n in 0..values.len() {
                    match &values[n] {
                        Json::OBJECT { name, value: _ } => {
                            match name == search {
                                true => {
                                    return Some(&mut values[n]);
                                },
                                false => {},
                            }
                        },
                        _ => {}
                    }
                }

                return None;
            },
            Json::OBJECT { name: _, value } => {
                match value.unbox_mut() {
                    Json::JSON(values) => {
                        for n in 0..values.len() {
                            match &values[n] {
                                Json::OBJECT { name, value: _ } => {
                                    match name == search {
                                        true => {
                                            return Some(&mut values[n]);
                                        },
                                        false => {},
                                    }
                                },
                                _ => {}
                            }
                        }
        
                        return None;
                    },
                    json => {
                        panic!("The function `get_mut(`&self`,`search: &str`)` may only be called on a `Json::JSON` or a `Json::OBJECT` holding a `Json::JSON`. I was called on: {:?}",json);
                    },
                }
            }
            json => {
                panic!("The function `get_mut(`&self`,`search: &str`)` may only be called on a `Json::JSON` or a `Json::OBJECT` holding a `Json::JSON`. I was called on: {:?}",json);
            }
        }
    }

    /// Enables matching the contents of a `Box`.
    pub fn unbox(&self) -> &Json {
        self
    }

    /// Idem.
    pub fn unbox_mut(&mut self) -> &mut Json {
        self
    }

    /// Returns a `String` of the form: `{"Json":"Value",...}` but can also be called on 'standalone objects'
    /// which could result in `"Object":{"Stuff":...}` or `"Json":true`.
    pub fn print(&self) -> String {
        let mut result = String::new();

        match self {
            Json::OBJECT { name, value } => {
                result.push_str(&format!("\"{}\":{}",name,value.print()));
            },
            Json::JSON(values) => {
                result.push('{');

                for n in 0..values.len() {
                    result.push_str(&values[n].print());
                    result.push(',');
                }

                result.pop();

                result.push('}');

            },
            Json::ARRAY(values) => {

                result.push('[');

                for n in 0..values.len() {
                    result.push_str(&values[n].print());
                    result.push(',');
                }

                result.pop();

                result.push(']');

            },
            Json::STRING(val) => {
                result.push_str(&format!("\"{}\"",val));
            },
            Json::NUMBER(val) => {
                result.push_str(&format!("{}",val));
            },
            Json::BOOL(val) => {
                match val {
                    true => {
                        result.push_str("true");
                    },
                    false => {
                        result.push_str("false")
                    },
                }
            },
            Json::NULL => {
                result.push_str("null");
            },
        }

        result
    }

    /// Parses the given bytes if a json structure is found. It even works with `\"Hello\":\"World\"`
    /// (doesn't have to be like `{...}`), i.e. it can return any of the variants in the `Json` enum.
    /// The error is returned in the for `(last position,what was the problem)`. Unfortunately the error
    /// description are minimal (basically "Error parsing ...type...").
    /// ## Example
    /// ```
    /// use json_minimal::*;
    /// 
    /// match Json::parse(b"{\"Greeting\":\"Hello, world!\"}") {
    ///     Ok(json) => {
    ///         
    ///         match json.get("Greeting") {
    ///             Some(json) => {
    ///                 match json {
    ///                     Json::OBJECT { name, value } => {
    ///                         match value.unbox() {
    ///                             Json::STRING(val) => {
    ///                                 assert_eq!(val,"Hello, world!");
    ///                             },
    ///                             json => {
    ///                                 panic!("Expected Json::STRING but found {:?}!!!",json);
    ///                             }
    ///                         }
    ///                     }
    ///                     json => {
    ///                         panic!("Expected Json::OBJECT but found {:?}!!!",json);
    ///                     }
    ///                 }
    ///             },
    ///             None => {
    ///                 panic!("Greeting was not found!!!");
    ///             }
    ///         }
    ///     },
    ///     Err( (pos,msg) ) => {
    ///         panic!("`{}` at position `{}`!!!",msg,pos);
    ///     }
    /// }
    /// ```
    /// See the <a href="https://github.com/36den/json_minimal-rs/">tutorial</a> on github for more.
    pub fn parse(input: &[u8]) -> Result<Json,(usize,&'static str)> {
        let mut incr: usize = 0;

        match input[incr] as char {
            '{' => {
                return Self::parse_json(input,&mut incr);
            },
            '\"' => {
                return Self::parse_string(input,&mut incr);
            },
            '[' => {
                return Self::parse_array(input,&mut incr);
            },
            't' => {
                return Self::parse_bool(input,&mut incr);
            },
            'f' => {
                return Self::parse_bool(input,&mut incr);
            },
            'n' => {
                return Self::parse_null(input,&mut incr);
            },
            '0'..='9' => {
                return Self::parse_number(input,&mut incr);
            },
            _ => {
                return Err( (incr,"Not a valid json format") );
            }
        }
    }

    // This must exclusively be used by `parse_string` to make any sense.
    fn parse_object(input: &[u8],incr: &mut usize,name: String) -> Result<Json,(usize,&'static str)> {

        match input[*incr] as char {
            ':' => {

            },
            _ => {
                return Err( (*incr,"Error parsing object.") );
            }
        }

        *incr += 1;

        match *incr < input.len() {
            true => {}
            false => {
                return Err( (*incr,"Error parsing object.") );
            }
        }

        match input[*incr]  as char {
            '{' => {
                match Self::parse_json(input,incr) {
                    Ok(json) => {
                        return Ok(
                            Json::OBJECT {
                                name,

                                value: Box::new( json )
                            }
                        )
                    },
                    Err(e) => {
                        return Err(e);
                    }
                }
            },
            '[' => {
                match Self::parse_array(input,incr) {
                    Ok(json) => {
                        return Ok(
                            Json::OBJECT {
                                name,

                                value: Box::new( json )
                            }
                        )
                    },
                    Err(e) => {
                        return Err(e);
                    }
                }
            },
            '\"' => {
                match Self::parse_string(input,incr) {
                    Ok(json) => {
                        return Ok(
                            Json::OBJECT {
                                name,

                                value: Box::new( json )
                            }
                        )
                    },
                    Err(e) => {
                        return Err(e);
                    }
                }
            },
            't' => {
                match Self::parse_bool(input,incr) {
                    Ok(json) => {
                        return Ok(
                            Json::OBJECT {
                                name,

                                value: Box::new( json )
                            }
                        )
                    },
                    Err(e) => {
                        return Err(e);
                    }
                }
            },
            'f' => {
                match Self::parse_bool(input,incr) {
                    Ok(json) => {
                        return Ok(
                            Json::OBJECT {
                                name,

                                value: Box::new( json )
                            }
                        )
                    },
                    Err(e) => {
                        return Err(e);
                    }
                }
            },
            'n' => {
                match Self::parse_null(input,incr) {
                    Ok(json) => {
                        return Ok(
                            Json::OBJECT {
                                name,

                                value: Box::new( json )
                            }
                        )
                    },
                    Err(e) => {
                        return Err(e);
                    }
                }
            },
            '0'..='9' => {
                match Self::parse_number(input,incr) {
                    Ok(json) => {
                        return Ok(
                            Json::OBJECT {
                                name,

                                value: Box::new( json )
                            }
                        )
                    },
                    Err(e) => {
                        return Err(e);
                    }
                }
            },
            _ => {
                return Err( (*incr,"Error parsing object.") );
            }
        }
    }

    // Parse if you thik it's something like `{...}`
    fn parse_json(input: &[u8], incr: &mut usize) -> Result<Json,(usize,&'static str)> {
        let mut result: Vec<Json> = Vec::new();

        match input[*incr] as char {
            '{' => {}
            _ => {
                return Err( (*incr,"Error parsing json.") );
            }
        }
    
        *incr += 1;
    
        match *incr < input.len() {
            true => {}
            false => {
                return Err( (*incr,"Error parsing json.") );
            }
        }

        loop {
            match input[*incr] as char {
                ',' => {
                    *incr += 1;
                },
                '\"' => {
                    match Self::parse_string(input,incr) {
                        Ok(json) => {
                            result.push( json );
                        },
                        Err(e) => {
                            return Err(e);
                        }
                    }
                },
                '[' => {
                    match Self::parse_array(input,incr) {
                        Ok(json) => {
                            result.push( json );
                        },
                        Err(e) => {
                            return Err(e);
                        }
                    }
                },
                't' => {
                    match Self::parse_bool(input,incr) {
                        Ok(json) => {
                            result.push( json );
                        },
                        Err(e) => {
                            return Err(e);
                        }
                    }
                },
                'f' => {
                    match Self::parse_bool(input,incr) {
                        Ok(json) => {
                            result.push( json );
                        },
                        Err(e) => {
                            return Err(e);
                        }
                    }
                },
                'n' => {
                    match Self::parse_null(input,incr) {
                        Ok(json) => {
                            result.push( json );
                        },
                        Err(e) => {
                            return Err(e);
                        }
                    }
                },
                '0'..='9' => {
                    match Self::parse_number(input,incr) {
                        Ok(json) => {
                            result.push( json );
                        },
                        Err(e) => {
                            return Err(e);
                        }
                    }
                },
                '}' => {
                    *incr += 1;

                    return Ok( Json::JSON( result ) );
                },
                '{' => {
                    match Self::parse_json(input,incr) {
                        Ok(json) => {
                            result.push( json );
                        },
                        Err(e) => {
                            return Err(e);
                        }
                    }
                },
                _ => {
                    return Err( (*incr,"Error parsing json.") );  
                }
            }
        }
    }

    // Parse a &str if you're sure it resembles `[...`
    fn parse_array(input: &[u8], incr: &mut usize) -> Result<Json,(usize,&'static str)> {
    let mut result: Vec<Json> = Vec::new();
    
        match input[*incr] as char {
            '[' => {}
            _ => {
                return Err( (*incr,"Error parsing array.") );
            }
        }
    
        *incr += 1;
    
        match *incr < input.len() {
            true => {}
            false => {
                return Err( (*incr,"Error parsing array.") );
            }
        }
    
        loop {
            match input[*incr] as char {
                ',' => {
                    *incr += 1;
                },
                '\"' => {
                    match Self::parse_string(input,incr) {
                        Ok(json) => {
                            result.push( json );
                        },
                        Err(e) => {
                            return Err(e);
                        }
                    }
                },
                '[' => {
                    match Self::parse_array(input,incr) {
                        Ok(json) => {
                            result.push( json );
                        },
                        Err(e) => {
                            return Err(e);
                        }
                    }
                },
                '{' => {
                    match Self::parse_json(input,incr) {
                        Ok(json) => {
                            result.push( json );
                        },
                        Err(e) => {
                            return Err(e);
                        }
                    }
                },
                't' => {
                    match Self::parse_bool(input,incr) {
                        Ok(json) => {
                            result.push( json );
                        },
                        Err(e) => {
                            return Err(e);
                        }
                    }
                },
                'f' => {
                    match Self::parse_bool(input,incr) {
                        Ok(json) => {
                            result.push( json );
                        },
                        Err(e) => {
                            return Err(e);
                        }
                    }
                },
                'n' => {
                    match Self::parse_null(input,incr) {
                        Ok(json) => {
                            result.push( json );
                        },
                        Err(e) => {
                            return Err(e);
                        }
                    }
                },
                '0'..='9' => {
                    match Self::parse_number(input,incr) {
                        Ok(json) => {
                            result.push( json );
                        },
                        Err(e) => {
                            return Err(e);
                        }
                    }
                },
                ']' => {
                    *incr += 1;

                    return Ok( Json::ARRAY( result ) );
                }
                _ => {
                    return Err( (*incr,"Error parsing array.") );  
                }
            }
        }
    
    }

    // Parse a &str if you know that it corresponds to/starts with a json String.
    fn parse_string(input: &[u8], incr: &mut usize) -> Result<Json,(usize,&'static str)> {
        let mut result = String::new();
    
        match input[*incr] as char {
            '\"' => {}
            _ => {
                return Err( (*incr,"Error parsing string.") );
            }
        }

        *incr += 1;

        match *incr < input.len() {
            true => {}
            false => {
                return Err( (*incr,"Error parsing string.") );
            }
        }

        loop {
            match input[*incr] as char {
                '\"' => {
                    *incr += 1;

                    match *incr < input.len() {
                        true => {
                            match input[*incr] as char {
                                ':' => {
                                    return Self::parse_object(input,incr,result);
                                },
                                _ => {
                                    return Ok( Json::STRING( result ) );
                                }
                            }
                        },
                        false => {
                            return Ok( Json::STRING( result ) );
                        }
                    }
                },
                c => {
                    result.push(c);

                    *incr += 1;

                    match *incr < input.len() {
                        true => {}
                        false => {
                            return Err( (*incr,"Error parsing string.") );
                        }
                    }
                }
            }
        }

    }

    fn parse_number(input: &[u8], incr: &mut usize) -> Result<Json,(usize,&'static str)> {
        let mut result = String::new();

        loop {
            match input[*incr] as char {
                '}' => {
                    break;
                },
                ']' => {
                    break;
                },
                ',' => {
                    break;
                },
                c => {
                    result.push(c);

                    *incr += 1;

                    match *incr < input.len() {
                        true => {
                        },
                        false => {
                            match result.parse::<f64>() {
                                Ok(num) => {
                                    return Ok( Json::NUMBER( num ) );
                                },
                                Err(_) => {
                                    return Err( (*incr,"Error parsing number.") );
                                }
                            }
                        }
                    }
                }
            }
        }

        match result.parse::<f64>() {
            Ok(num) => {
                return Ok( Json::NUMBER( num ) );
            },
            Err(_) => {
                return Err( (*incr,"Error parsing number.") );
            }
        }

    }

    fn parse_bool(input: &[u8], incr: &mut usize) -> Result<Json,(usize,&'static str)> {
        let mut result = String::new();

        loop {
            match input[*incr] as char {
                ',' => {
                    break;
                },
                ']' => {
                    break;
                },
                '}' => {
                    break;
                },
                c => {
                    result.push(c);

                    *incr += 1;

                    match *incr < input.len() {
                        true => {}
                        false => {
                            match result == "true" {
                                true => {
                                    return Ok( Json::BOOL( true ) );
                                },
                                false => {}
                            }
                    
                            match result == "false" {
                                true => {
                                    return Ok( Json::BOOL( false ) );
                                },
                                false => {}
                            }
                    
                            return Err( (*incr,"Error parsing bool.") );
                        }
                    }
                }
            }
        }

        match result == "true" {
            true => {
                return Ok( Json::BOOL( true ) );
            },
            false => {}
        }

        match result == "false" {
            true => {
                return Ok( Json::BOOL( false ) );
            },
            false => {}
        }

        return Err( (*incr,"Error parsing bool.") );
    }

    fn parse_null(input: &[u8], incr: &mut usize) -> Result<Json,(usize,&'static str)> {
        let mut result = String::new();

        loop {

            match input[*incr] as char {
                ',' => {
                    break;
                },
                ']' => {
                    break;
                },
                '}' => {
                    break;
                },
                c => {
                    result.push(c);

                    *incr += 1;

                    match *incr < input.len() {
                        true => {}
                        false => {
                            match result == "null" {
                                true => {
                                    return Ok( Json::NULL );
                                },
                                false => {
                                    return Err( (*incr,"Error parsing null.") );
                                }
                            } 
                        }
                    }
                }
            }
        }

        match result == "null" {
            true => {
                return Ok( Json::NULL );
            },
            false => {
                return Err( (*incr,"Error parsing null.") );
            }
        } 
    }

}

#[cfg(test)]
mod tests;