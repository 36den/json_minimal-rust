#[derive(Debug)]
pub enum Json {
    OBJECT { name: String, value: Box<Json> },
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
            Json::JSON(values) => match value {
                Json::OBJECT { name, value } => {
                    values.push(Json::OBJECT { name, value });
                }
                Json::JSON(_) => {
                    panic!("A `Json::JSON` may not be added to a `Json::JSON` if it is not within a `Json::OBJECT`.");
                }
                Json::ARRAY(vals) => {
                    values.push(Json::ARRAY(vals));
                }
                Json::STRING(val) => {
                    values.push(Json::STRING(val));
                }
                Json::NUMBER(val) => {
                    values.push(Json::NUMBER(val));
                }
                Json::BOOL(val) => {
                    values.push(Json::BOOL(val));
                }
                Json::NULL => {
                    values.push(Json::NULL);
                }
            },
            Json::OBJECT {
                name: _,
                value: obj_val,
            } => match obj_val.unbox_mut() {
                Json::JSON(values) => match value {
                    Json::OBJECT { name, value } => {
                        values.push(Json::OBJECT { name, value });
                    }
                    Json::JSON(_) => {
                        panic!("A `Json::JSON` may not be added to a `Json::JSON` if it is not within a `Json::OBJECT`.");
                    }
                    Json::ARRAY(vals) => {
                        values.push(Json::ARRAY(vals));
                    }
                    Json::STRING(val) => {
                        values.push(Json::STRING(val));
                    }
                    Json::NUMBER(val) => {
                        values.push(Json::NUMBER(val));
                    }
                    Json::BOOL(val) => {
                        values.push(Json::BOOL(val));
                    }
                    Json::NULL => {
                        values.push(Json::NULL);
                    }
                },
                Json::ARRAY(values) => match value {
                    Json::OBJECT { name, value } => {
                        values.push(Json::OBJECT { name, value });
                    }
                    Json::JSON(vals) => {
                        values.push(Json::JSON(vals));
                    }
                    Json::ARRAY(vals) => {
                        values.push(Json::ARRAY(vals));
                    }
                    Json::STRING(val) => {
                        values.push(Json::STRING(val));
                    }
                    Json::NUMBER(val) => {
                        values.push(Json::NUMBER(val));
                    }
                    Json::BOOL(val) => {
                        values.push(Json::BOOL(val));
                    }
                    Json::NULL => {
                        values.push(Json::NULL);
                    }
                },
                json => {
                    panic!("The function `add(`&mut self`,`name: String`,`value: Json`)` may only be called on a `Json::JSON`, `Json::ARRAY` or `Json::OBJECT` holding a `Json::JSON` or `Json::ARRAY`. It was called on: {:?}",json);
                }
            },
            Json::ARRAY(values) => match value {
                Json::OBJECT { name, value } => {
                    values.push(Json::OBJECT { name, value });
                }
                Json::JSON(vals) => {
                    values.push(Json::JSON(vals));
                }
                Json::ARRAY(vals) => {
                    values.push(Json::ARRAY(vals));
                }
                Json::STRING(val) => {
                    values.push(Json::STRING(val));
                }
                Json::NUMBER(val) => {
                    values.push(Json::NUMBER(val));
                }
                Json::BOOL(val) => {
                    values.push(Json::BOOL(val));
                }
                Json::NULL => {
                    values.push(Json::NULL);
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
    pub fn get(&self, search: &str) -> Option<&Json> {
        match self {
            Json::JSON(values) => {
                for n in 0..values.len() {
                    match &values[n] {
                        Json::OBJECT { name, value: _ } => {
                            if name == search {
                                return Some(&values[n]);
                            }
                        }
                        _ => {}
                    }
                }

                return None;
            }
            Json::OBJECT { name: _, value } => match value.unbox() {
                Json::JSON(values) => {
                    for n in 0..values.len() {
                        match &values[n] {
                            Json::OBJECT { name, value: _ } => {
                                if name == search {
                                    return Some(&values[n]);
                                }
                            }
                            _ => {}
                        }
                    }

                    return None;
                }
                json => {
                    panic!("The function `get(`&self`,`search: &str`)` may only be called on a `Json::JSON` or a `Json::OBJECT` holding a `Json::JSON`. I was called on: {:?}",json);
                }
            },
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
                            if name == search {
                                return Some(&mut values[n]);
                            }
                        }
                        _ => {}
                    }
                }
            }
            Json::OBJECT { name: _, value } => match value.unbox_mut() {
                Json::JSON(values) => {
                    for n in 0..values.len() {
                        match &values[n] {
                            Json::OBJECT { name, value: _ } => {
                                if name == search {
                                    return Some(&mut values[n]);
                                }
                            }
                            _ => {}
                        }
                    }
                }
                json => {
                    panic!("The function `get_mut(`&self`,`search: &str`)` may only be called on a `Json::JSON` or a `Json::OBJECT` holding a `Json::JSON`. I was called on: {:?}",json);
                }
            },
            json => {
                panic!("The function `get_mut(`&self`,`search: &str`)` may only be called on a `Json::JSON` or a `Json::OBJECT` holding a `Json::JSON`. I was called on: {:?}",json);
            }
        }

        None
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
                result.push_str(&format!("\"{}\":{}", name, value.print()));
            }
            Json::JSON(values) => {
                result.push('{');

                result.push_str(
                    values.into_iter()
                        .map(|v| v.print())
                        .collect::<Vec<String>>()
                        .join(",")
                        .as_str()
                );

                result.push('}');
            }
            Json::ARRAY(values) => {
                result.push('[');

                result.push_str(
                    values.into_iter()
                        .map(|v| v.print())
                        .collect::<Vec<String>>()
                        .join(",")
                        .as_str()
                );

                result.push(']');
            }
            Json::STRING(val) => {
                result.push_str(&format!("\"{}\"", val));
            }
            Json::NUMBER(val) => {
                result.push_str(&format!("{}", val));
            }
            Json::BOOL(val) => {
                if *val {
                    result.push_str("true");
                } else {
                    result.push_str("false")
                }
            }
            Json::NULL => {
                result.push_str("null");
            }
        }

        result
    }

    /// Parses the given bytes if a json structure is found. It even works with `\"Hello\":\"World\"`
    /// (doesn't have to be like `{...}`), i.e. it can return any of the variants in the `Json` enum.
    /// The error is returned in the form `(last position, what went wrong)`. Unfortunately the error
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
    pub fn parse(input: &[u8]) -> Result<Json, (usize, &'static str)> {
        let mut incr: usize = 0;

        match input[incr] as char {
            '{' => Self::parse_json(input, &mut incr),
            '\"' => Self::parse_string(input, &mut incr),
            '[' => Self::parse_array(input, &mut incr),
            't' | 'f' => Self::parse_bool(input, &mut incr),
            'n' => Self::parse_null(input, &mut incr),
            '0'..='9' => Self::parse_number(input, &mut incr),
            _ => Err((incr, "Not a valid json format")),
        }
    }

    // This must exclusively be used by `parse_string` to make any sense.
    fn parse_object(
        input: &[u8],
        incr: &mut usize,
        name: String,
    ) -> Result<Json, (usize, &'static str)> {
//        if input[*incr] as char != ':' {
//            return Err((*incr, "Error parsing object."));
//        }

        *incr += 1;

        if *incr >= input.len() {
            return Err((*incr, "Error parsing object."));
        }

        loop {
            match input[*incr] as char {
                '\r' | '\n' | '\t' | ' ' => {
                    *incr += 1;

                    if *incr >= input.len() {
                        return Err((*incr, "Error parsing object."));
                    }
                },
                _ => {
                 break;
                }
            }
        }

        let value = match input[*incr] as char {
            '{' => Self::parse_json(input, incr)?,
            '[' => Self::parse_array(input, incr)?,
            '\"' => Self::parse_string(input, incr)?,
            't' | 'f' => Self::parse_bool(input, incr)?,
            'n' => Self::parse_null(input, incr)?,
            '0'..='9' => Self::parse_number(input, incr)?,
            _ => {
                return Err((*incr, "Error parsing object."));
            }
        };

        Ok(Json::OBJECT {
            name,

            value: Box::new(value),
        })
    }

    // Parse if you thik it's something like `{...}`
    fn parse_json(input: &[u8], incr: &mut usize) -> Result<Json, (usize, &'static str)> {
        let mut result: Vec<Json> = Vec::new();

//        if input[*incr] as char != '{' {
//            return Err((*incr, "Error parsing json."));
//        }

        *incr += 1;

        if *incr >= input.len() {
            return Err((*incr, "Error parsing json."));
        }

        loop {
            let json = match input[*incr] as char {
                ',' => {
                    *incr += 1;
                    continue;
                }
                '\"' => Self::parse_string(input, incr)?,
                '[' => Self::parse_array(input, incr)?,
                't' | 'f' => Self::parse_bool(input, incr)?,
                'n' => Self::parse_null(input, incr)?,
                '0'..='9' => Self::parse_number(input, incr)?,
                '}' => {
                    *incr += 1;

                    return Ok(Json::JSON(result));
                }
                '{' => Self::parse_json(input, incr)?,
                '\r' | '\n' | '\t' | ' ' => {                    
                    *incr += 1;

                    if *incr >= input.len() {
                        return Err((*incr, "Error parsing json."));
                    }

                    continue;
                },
                _ => {
                    return Err((*incr, "Error parsing json."));
                }
            };

            result.push(json);
        }
    }

    // Parse a &str if you're sure it resembles `[...`
    fn parse_array(input: &[u8], incr: &mut usize) -> Result<Json, (usize, &'static str)> {
        let mut result: Vec<Json> = Vec::new();

//        if input[*incr] as char != '[' {
//            return Err((*incr, "Error parsing array."));
//        }

        *incr += 1;

        if *incr >= input.len() {
            return Err((*incr, "Error parsing array."));
        }

        loop {
            let json = match input[*incr] as char {
                ',' => {
                    *incr += 1;
                    continue;
                }
                '\"' => Self::parse_string(input, incr)?,
                '[' => Self::parse_array(input, incr)?,
                '{' => Self::parse_json(input, incr)?,
                't' | 'f' => Self::parse_bool(input, incr)?,
                'n' => Self::parse_null(input, incr)?,
                '0'..='9' => Self::parse_number(input, incr)?,
                ']' => {
                    *incr += 1;

                    return Ok(Json::ARRAY(result));
                },
                '\r' | '\n' | '\t' | ' ' => {
                    *incr += 1;

                    if *incr >= input.len() {
                        return Err((*incr, "Error parsing array."));
                    }

                    continue;
                },
                _ => {
                    return Err((*incr, "Error parsing array."));
                }
            };

            result.push(json);
        }
    }

    // Parse a &str if you know that it corresponds to/starts with a json String.
    fn parse_string(input: &[u8], incr: &mut usize) -> Result<Json, (usize, &'static str)> {
        let mut result: Vec<u8> = Vec::new();

//        if input[*incr] as char != '\"' {
//            return Err((*incr, "Error parsing string."));
//        }

        *incr += 1;

        if *incr >= input.len() {
            return Err((*incr, "Error parsing string."));
        }

        loop {
            match input[*incr] {
                b'\"' => {
                    *incr += 1;

                    let result = String::from_utf8(result)
                        .map_err(|_| (*incr, "Error parsing non-utf8 string."))?;

                    if *incr < input.len() {
                        if input[*incr] as char == ':' {
                            return Self::parse_object(input, incr, result);
                        } else {
                            return Ok(Json::STRING(result));
                        }
                    } else {
                        return Ok(Json::STRING(result));
                    }
                },
                b'\\' => {
                    Self::parse_string_escape_sequence(input, incr, &mut result)?;
                },
                c => {
                    result.push(c);

                    *incr += 1;

                    if *incr >= input.len() {
                        return Err((*incr, "Error parsing string."));
                    }
                }
            }
        }
    }

    // Parse an escape sequence inside a string
    fn parse_string_escape_sequence(
        input: &[u8],
        incr: &mut usize,
        result: &mut Vec<u8>,
    ) -> Result<(), (usize, &'static str)> {
//        if input[*incr] as char != '\\' {
//            return Err((*incr, "Error parsing string escape sequence."));
//        }

        *incr += 1;

        if *incr >= input.len() {
            return Err((*incr, "Error parsing string escape sequence."));
        }

        match input[*incr] as char {
            '\"' | '\\' | '/' => {
                result.push(input[*incr]);
            }
            'b' => {
                result.push(b'\x08');
            }
            'f' => {
                result.push(b'\x0c');
            }
            'n' => {
                result.push(b'\n');
            }
            'r' => {
                result.push(b'\r');
            }
            't' => {
                result.push(b'\t');
            }
            'u' => {
                const BAD_UNICODE: &str = "Error parsing unicode string escape sequence.";

                if *incr + 4 >= input.len() {
                    return Err((*incr, BAD_UNICODE));
                }

                let hex = (&input[*incr + 1..*incr + 5]).to_vec();
                let hex = String::from_utf8(hex).map_err(|_| (*incr, BAD_UNICODE))?;
                let value = u16::from_str_radix(&hex, 16).map_err(|_| (*incr, BAD_UNICODE))?;
                let value = std::char::from_u32(value as u32).ok_or((*incr, BAD_UNICODE))?;

                let mut buffer = [0; 4];
                result.extend(value.encode_utf8(&mut buffer).as_bytes());
                *incr += 4;
            }
            _ => {
                return Err((*incr, "Error parsing invalid string escape sequence."));
            }
        }

        *incr += 1;

        if *incr >= input.len() {
            return Err((*incr, "Error parsing string escape sequence."));
        }

        Ok(())
    }

    fn parse_number(input: &[u8], incr: &mut usize) -> Result<Json, (usize, &'static str)> {
        let mut result = String::new();

        loop {
            match input[*incr] as char {
                ',' | ']' | '}' | '\r' | '\n' | '\t' | ' ' => {
                    break;
                },
                c => {
                    result.push(c);

                    *incr += 1;

                    if *incr >= input.len() {
                        match result.parse::<f64>() {
                            Ok(num) => {
                                return Ok(Json::NUMBER(num));
                            }
                            Err(_) => {
                                return Err((*incr, "Error parsing number."));
                            }
                        }
                    }
                }
            }
        }

        match result.parse::<f64>() {
            Ok(num) => {
                return Ok(Json::NUMBER(num));
            }
            Err(_) => {
                return Err((*incr, "Error parsing number."));
            }
        }
    }

    fn parse_bool(input: &[u8], incr: &mut usize) -> Result<Json, (usize, &'static str)> {
        let mut result = String::new();

        loop {
            match input[*incr] as char {
                ',' | ']' | '}' | '\r' | '\n' | '\t' | ' ' => {
                    break;
                },
                c => {
                    result.push(c);

                    *incr += 1;

                    if *incr >= input.len() {
                        if result == "true" {
                            return Ok(Json::BOOL(true));
                        }

                        if result == "false" {
                            return Ok(Json::BOOL(false));
                        }

                        return Err((*incr, "Error parsing bool."));
                    }
                }
            }
        }

        if result == "true" {
            return Ok(Json::BOOL(true));
        }

        if result == "false" {
            return Ok(Json::BOOL(false));
        }

        return Err((*incr, "Error parsing bool."));
    }

    fn parse_null(input: &[u8], incr: &mut usize) -> Result<Json, (usize, &'static str)> {
        let mut result = String::new();

        loop {
            match input[*incr] as char {
                ',' | ']' | '}' | '\r' | '\n' | '\t' | ' ' => {
                    break;
                },
                c => {
                    result.push(c);

                    *incr += 1;

                    if *incr >= input.len() {
                        if result == "null" {
                            return Ok(Json::NULL);
                        } else {
                            return Err((*incr, "Error parsing null."));
                        }
                    }
                }
            }
        }

        if result == "null" {
            return Ok(Json::NULL);
        } else {
            return Err((*incr, "Error parsing null."));
        }
    }
}

#[cfg(test)]
mod tests;
