pub(crate) struct SimpleLinkedList {
    data: i32,
    pub(crate) next: Option<Box<SimpleLinkedList>>,
}

impl SimpleLinkedList {
    // Default constructor
    pub(crate) fn new() -> Self {
        SimpleLinkedList { data: 0, next: None }
    }

    // Constructor with data and next
    fn with_data_and_next(data: i32, next: Option<Box<SimpleLinkedList>>) -> Self {
        SimpleLinkedList { data, next }
    }

    // Constructor with data
    fn with_data(data: i32) -> Self {
        SimpleLinkedList { data, next: None }
    }

    // Getter for data
    fn get_data(&self) -> i32 {
        self.data
    }

    // Setter for data
    fn set_data(&mut self, data: i32) {
        self.data = data;
    }

    // Getter for next
    fn get_next(&self) -> &Option<Box<SimpleLinkedList>> {
        &self.next
    }

    // Setter for next
    pub(crate) fn set_next(&mut self, next: Option<Box<SimpleLinkedList>>) {
        self.next = next;
    }
}