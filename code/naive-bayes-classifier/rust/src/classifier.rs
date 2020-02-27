/// A simple implementation of the Naive-Bayes classifier

use regex::Regex;
use std::collections::{HashMap, HashSet};
use crate::training_set::TrainingSet;

#[derive(Debug)]
pub struct Classifier<'a> {
    labels: Vec<&'a str>,
    training_set: HashMap<String, HashMap<&'a str, f32>>
}

impl <'a> Classifier<'a> {
    /// Finds all words from the specified message and returns a set object with their words
    pub fn tokenize(msg: &str) -> HashSet<String> {
        let re = Regex::new(r"\p{L}+").unwrap();

        re.find_iter(msg).fold(HashSet::new(), |mut set, el| {
            set.insert(el.as_str().to_owned());
            set
        })
    }

    /// Initializes the specified data as a training set
    pub fn init_training_set(data: &'a TrainingSet, k: Option<f32>) -> Self {
        let nk = k.or(Some(0.5)).unwrap();

        let labels: Vec<_> = data.store
            .keys()
            .map(|el| el.as_str())
            .collect();

        let mut total: HashMap<_, u32> = HashMap::new();
        let mut training_set = HashMap::new();

        for (label, messages) in &data.store {
            for msg in messages {
                for word in Classifier::tokenize(&msg) {
                    let clusters = training_set.entry(word).or_insert_with(|| {
                        let mut values = HashMap::new();

                        for label in &labels {
                            values.insert(label.to_owned(), 0.0);
                        }

                        values
                    });

                    *clusters.get_mut(label.as_str()).unwrap() += 1.0;
                    *total.entry(label.as_str()).or_default() += 1;
                }
            }
        }

        for (_, labels) in &mut training_set {
            for (label, value) in labels {
                *value = (nk + *value) / (2.0 * nk + *total.get(label).unwrap() as f32)
            }
        }

        Classifier {
            labels,
            training_set
        }
    }

    /// Checks the specified message with a training set and returns a dictionary with probabilities
    pub fn check(&self, msg: &str) -> HashMap<&str, f32> {
        let mut values = vec![0f32; self.labels.len()];
        let tokens = Classifier::tokenize(msg);

        for (word, clusters) in &self.training_set {
            for (i, label) in self.labels.iter().enumerate() {
                if tokens.contains(word) {
                    values[i] += clusters.get(label).unwrap().ln();

                } else {
                    values[i] += (1.0 - *clusters.get(label).unwrap()).ln();
                }
            }
        }

        values = values.iter().map(|el| el.exp()).collect();

        let sum_of_values: f32 = values.iter().sum();
        let mut result = HashMap::new();

        for (i, label) in self.labels.iter().enumerate() {
            result.insert(*label, values[i] / sum_of_values);
        }

        result
    }
}
