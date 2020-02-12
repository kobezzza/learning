from training_set import training_set
from classifier import Classifier

classifier = Classifier(training_set)

print(classifier.check('Hi my world'))
print(classifier.check('Hi viagra'))
print(classifier.check('Hi everybody'))
