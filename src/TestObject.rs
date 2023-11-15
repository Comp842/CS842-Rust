pub(crate) struct TestObject {
    // Primitive Data Types
    integer_variable: i32,
    double_variable: f64,
    boolean_variable: bool,
    char_variable: char,

    // Reference Data Types
    string_variable: Option<String>,
    integer_array: Option<Vec<i32>>,
    object_variable: Option<Box<dyn std::any::Any>>, // Using a trait object for Object

    // Other common data types
    long_variable: i64,
    float_variable: f32,
    byte_variable: i8,
    short_variable: i16,
}

impl TestObject {
    // Constructor
    pub(crate) fn new() -> Self {
        TestObject {
            // Initialize default values for the variables
            integer_variable: 0,
            double_variable: 0.0,
            boolean_variable: false,
            char_variable: '\0',
            string_variable: None,
            integer_array: None,
            object_variable: None,
            long_variable: 0,
            float_variable: 0.0,
            byte_variable: 0,
            short_variable: 0,
        }
    }

    // Getter and Setter methods
    fn get_integer_variable(&self) -> i32 {
        self.integer_variable
    }

    fn set_integer_variable(&mut self, value: i32) {
        self.integer_variable = value;
    }

    fn get_double_variable(&self) -> f64 {
        self.double_variable
    }

    fn set_double_variable(&mut self, value: f64) {
        self.double_variable = value;
    }

    fn is_boolean_variable(&self) -> bool {
        self.boolean_variable
    }

    fn set_boolean_variable(&mut self, value: bool) {
        self.boolean_variable = value;
    }

    fn get_char_variable(&self) -> char {
        self.char_variable
    }

    fn set_char_variable(&mut self, value: char) {
        self.char_variable = value;
    }

    fn get_string_variable(&self) -> Option<&String> {
        self.string_variable.as_ref()
    }

    fn set_string_variable(&mut self, value: String) {
        self.string_variable = Some(value);
    }

    fn get_integer_array(&self) -> Option<&Vec<i32>> {
        self.integer_array.as_ref()
    }

    fn set_integer_array(&mut self, value: Vec<i32>) {
        self.integer_array = Some(value);
    }

    fn get_object_variable(&self) -> Option<&Box<dyn std::any::Any>> {
        self.object_variable.as_ref()
    }

    fn set_object_variable(&mut self, value: Box<dyn std::any::Any>) {
        self.object_variable = Some(value);
    }

    fn get_long_variable(&self) -> i64 {
        self.long_variable
    }

    fn set_long_variable(&mut self, value: i64) {
        self.long_variable = value;
    }

    fn get_float_variable(&self) -> f32 {
        self.float_variable
    }

    fn set_float_variable(&mut self, value: f32) {
        self.float_variable = value;
    }

    fn get_byte_variable(&self) -> i8 {
        self.byte_variable
    }

    fn set_byte_variable(&mut self, value: i8) {
        self.byte_variable = value;
    }

    fn get_short_variable(&self) -> i16 {
        self.short_variable
    }

    fn set_short_variable(&mut self, value: i16) {
        self.short_variable = value;
    }
}