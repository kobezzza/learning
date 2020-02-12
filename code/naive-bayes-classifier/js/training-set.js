const data = new Map();
export default data;

function addDocument(message, type) {
	const
		cluster = data.get(type) || [];

	data.set(type, cluster);
	cluster.push(message);
}

addDocument('Hello viagra', 'spam');
addDocument('viagra again', 'spam');
addDocument('viagra strikes', 'spam');
addDocument('Hello world', 'notSpam');
