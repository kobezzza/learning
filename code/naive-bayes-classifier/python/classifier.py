import re
import math
from collections import Counter, defaultdict


class Classifier:
    """
    A simple implementation of the Naive-Bayes classifier
    """

    training_set: dict
    training_labels: set

    def __init__(self, training_set: dict):
        self.init_training_set(training_set)

    def tokenize(self, msg: str) -> set:
        """
        Finds all words from the specified message and returns a set object with their words
        :param msg:
        :return:
        """

        return set(re.findall(r'\w+', msg.lower()))

    def init_training_set(self, data: dict):
        """
        Initializes the specified data as a training set
        :param data:
        :return:
        """

        self.training_labels = set(data.keys())

        words_count = dict()
        total = Counter()

        for key, cluster in data.items():
            for message in cluster:
                for word in self.tokenize(message):
                    if word not in words_count:
                        words_count[word] = {key: 0 for key in self.training_labels}

                    words_count[word][key] += 1
                    total[key] += 1

        training_set = defaultdict(lambda: defaultdict(float))
        k = 0.5

        for word, clusters in words_count.items():
            for key in clusters:
                training_set[word][key] = (k + clusters[key]) / (2 * k + total[key])

        self.training_set = training_set

    def check(self, msg: str) -> dict:
        """
        Checks the specified message with a training set and returns a dictionary with probabilities
        :param msg:
        :return:
        """

        tokens = self.tokenize(msg)
        values = [0] * len(self.training_labels)

        for word, clusters in self.training_set.items():
            for i, key in enumerate(clusters.keys()):
                if word in tokens:
                    values[i] += math.log(clusters[key])

                else:
                    values[i] += math.log(1 - clusters[key])

        values = list(map(math.exp, values))
        sum_of_values = sum(values)
        result = dict()

        for i, key in enumerate(self.training_labels):
            result[key] = values[i] / sum_of_values

        return result
