use rand::Rng;
use crate::TestObject::TestObject;

pub(crate) struct AdvancedLinkedList {
    data: Box<dyn std::any::Any>, // Using a trait object for Object
    pub(crate) next: Option<Box<AdvancedLinkedList>>,
}

impl AdvancedLinkedList {
    // Default constructor
    pub(crate) fn new() -> Self {
        let mut rng = rand::thread_rng();
        let num_of_objects = rng.gen_range(1..=5);

        let mut arr = Vec::with_capacity(num_of_objects);
        for _ in 0..num_of_objects {
            arr.push(Box::new(TestObject::new()) as Box<dyn std::any::Any>);
        }

        AdvancedLinkedList {
            data: Box::new(arr),
            next: None,
        }
    }

    // Constructor with data and next
    fn with_data_and_next(data: Box<dyn std::any::Any>, next: Option<Box<AdvancedLinkedList>>) -> Self {
        AdvancedLinkedList { data, next }
    }

    // Constructor with data
    fn with_data(data: Box<dyn std::any::Any>) -> Self {
        AdvancedLinkedList { data, next: None }
    }

    // Getter for data
    fn get_data(&self) -> &Box<dyn std::any::Any> {
        &self.data
    }

    // Setter for data
    fn set_data(&mut self, data: Box<dyn std::any::Any>) {
        self.data = data;
    }

    // Getter for next
    fn get_next(&self) -> &Option<Box<AdvancedLinkedList>> {
        &self.next
    }

    // Setter for next
    pub(crate) fn set_next(&mut self, next: Option<Box<AdvancedLinkedList>>) {
        self.next = next;
    }
}