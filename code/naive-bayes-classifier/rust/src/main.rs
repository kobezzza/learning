use crate::classifier::Classifier;

mod training_set;
mod classifier;

fn main() {
    let data = training_set::init();
    let classifier = Classifier::init_training_set(&data, None);

    println!("{:?}", classifier.check("Hi my world!"));
    println!("{:?}", classifier.check("Hi my viagra!"));
    println!("{:?}", classifier.check("Hi everybody!"));
}
