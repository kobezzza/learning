export default class Classifier {
	static tokenize(message) {
		message = message.toLowerCase();

		const
			words = message.match(/\p{L}+/gu);

		return words ? new Set(words) : new Set();
	}

	constructor(trainingSet, k = 0.5) {
		this.initTrainingSet(trainingSet, k);
	}

	initTrainingSet(data, k) {
		this.trainingLabels = [...data.keys()];

		const
			trainingSet = new Map();

		const total = this.trainingLabels.reduce(
			(map, key) => map.set(key, 0),
			new Map()
		);

		for (const [key, cluster] of data.entries()) {
			for (const message of cluster) {
				for (const word of this.constructor.tokenize(message)) {
					const map = trainingSet.get(word) || this.trainingLabels.reduce(
						(map, key) => map.set(key, 0),
						new Map()
					);

					map.set(key, map.get(key) + 1);
					total.set(key, total.get(key) + 1);
					trainingSet.set(word, map);
				}
			}
		}

		for (const [word, clusters] of trainingSet.entries()) {
			trainingSet.set(word, this.trainingLabels.reduce(
				(map, key) => map.set(key, (k + clusters.get(key)) / (2 * k + total.get(key))),
				new Map()
			));
		}

		this.trainingSet = trainingSet;
	}

	check(message) {
		const
			tokens = this.constructor.tokenize(message);

		let
			values = new Array(this.trainingLabels.length).fill(0);

		for (const [word, clusters] of this.trainingSet.entries()) {
			for (const [i, key] of [...clusters.keys()].entries()) {
				const
					val = clusters.get(key);

				if (tokens.has(word)) {
					values[i] += Math.log(val);

				} else {
					values[i] += Math.log(1 - val);
				}
			}
		}

		values = values.map(Math.exp);

		const
			sumOfValues = values.reduce((res, el) => res + el);

		return [...this.trainingLabels.entries()].reduce((map, [i, key]) => {
			map[key] = values[i] / sumOfValues;
			return map;
		}, {});
	}
}
