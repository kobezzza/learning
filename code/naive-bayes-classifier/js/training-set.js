const data = new Map();
export default data;

function addDocument(message, type) {
	const
		cluster = data.get(type) || [];

	data.set(type, cluster);
	cluster.push(message);
}

addDocument('Hello world!', 'notSpam');
addDocument('Hello viagra!', 'spam');
