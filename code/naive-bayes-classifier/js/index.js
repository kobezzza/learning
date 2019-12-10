import trainingSet from './training-set.js';
import Classifier from './classifier.js';

const
	classifier = new Classifier(trainingSet);

console.log(classifier.check('Hi my world!'));
console.log(classifier.check('Hi my viagra!'));
console.log(classifier.check('Hi everybody!'));
