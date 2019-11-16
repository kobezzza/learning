import trainingSet from './training-set.js';

function tokenize(message) {
	message = message.toLowerCase();

	const
		words = message.match(/\p{L}+/gu);

	return words ? [...new Set(words)] : [];
}

function countWords(trainingSet) {
	const
		dict = new Map();

	for (const {message, spam} of trainingSet) {
		for (const word of tokenize(message)) {
			const obj = dict.get(word) || {spam: 0, notSpan: 0};
			obj[spam ? 'spam' : 'notSpan']++;
			dict.set(word, obj);
		}
	}

	return dict;
}

console.log(countWords(trainingSet));
