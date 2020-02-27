use std::collections::hash_map;

#[derive(Debug)]
pub struct TrainingSet {
    pub store: hash_map::HashMap<String, Vec<String>>
}

impl TrainingSet {
    pub fn new() -> Self {
        TrainingSet {
            store: hash_map::HashMap::new()
        }
    }

    pub fn add_document<T: Into<String>>(&mut self, msg: T, cat: T) {
        self.store.entry(cat.into()).or_default().push(msg.into());
    }

    pub fn labels(&self) -> impl Iterator<Item=&String> {
        self.store.keys()
    }
}

pub fn init() -> TrainingSet {
    let mut training_set = TrainingSet::new();

    training_set.add_document("Hello viagra", "spam");
    training_set.add_document("viagra again", "spam");
    training_set.add_document("viagra strikes", "spam");
    training_set.add_document("Hello world", "notSpam");

    training_set
}
