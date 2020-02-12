from collections import defaultdict

training_set = defaultdict(list)


def add_document(message: str, cat: str):
    training_set[cat].append(message)


add_document('Hello viagra', 'spam')
add_document('viagra again', 'spam')
add_document('viagra strikes', 'spam')
add_document('Hello world', 'notSpam')
