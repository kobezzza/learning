# Нюансы объекта Promise в JavaScript
## Вызов then/catch без параметров

Если для объекта промиса вызвать методы then/catch без передачи параметров, то возвращаемый новый промис будет содержать значение старого.

```js
var 
  a = Promise.resolve(1);

console.log(a !== a.then());
a.then().then((r) => console.log(r)); // 1
```

## finally

Метод finally вызывается когда объект переходит в любое из состояний fulfilled/rejected. Функция обработчик при этом не принимает никаких аргументов и возвращает новый промис, содержащий значение старого (т.е. возвращаемое значение из функции игнорируется). Также, этот метод не перехватывает распространение ошибки из промиса.

```js
var 
  a = Promise.resolve(1);

console.log(a !== a.finally());

a.finally((value) => {
	console.log(value); // undefined
	return 2;
}).then((r) => console.log(r)); // 1
```

```js
Promise.reject(1).finally(() => {}) // uncaught exception: 1
```

## Асинхронная функция в конструкторе промиса

Если использовать асинхронную функция в конструкторе промиса, то он будет игнорировать исключения внутри конструктора.

```js
new Promise(() => {
	throw new Error('boom!');
}).catch((err) => console.log(err)); // Error: boom!

new Promise(async () => {
	throw new Error('boom!');
}).catch((err) => console.log(err)); // Ошибка не перехватится, а промис никогда не зарезолвится
```

## Разглаживание промисов

Если зарезолвить промис другим промисом, то следущая по конвееру функция then получит финальное значение, а не объект промиса.

```js
new Promise((resolve) => {
	resolve(Promise.resolve(1));
}).then((r) => console.log(r)); // 1
```

Если зарезолвить зареджекчиным промисом, то следущая по конвееру функция catch (или then второй параметр) получит финальное значение, а не объект промиса.

```js
new Promise((resolve) => {
	resolve(Promise.reject(1));
}).catch((r) => console.log(r)); // 1
```

Если зарежектить промис другим промисом, то он будет обработан "как есть".

```js
new Promise((resolve, reject) => {
	reject(Promise.resolve(1));
}).catch((r) => console.log(r)); // Promise<1>

new Promise((resolve, reject) => {
	reject(Promise.reject(1));
}).catch((r) => console.log(r)); // Promise<1>
```

Промис, который зарезолвился другим промисом будет исполнятся позже, чем просто зарезолвленый промис.
Это происходит потому, что все методы промиса выполняются отложенно (then().then() > then()).

```js
var a = new Promise((resolve) => {
	resolve(Promise.resolve(1));
});

var b = Promise.resolve(2);

Promise.race([a, b]).then((r) => console.log(r)); // 2
```

Если зарезовлить промис объектом, у которого есть метод then, то он будет трактоваться как промис и будет разглажен (при этом возвращаемое значение then не важно).

```js
var a = new Promise((resolve) => {
	resolve({then(onFulfilled) { onFulfilled(1); return 9; }});
});

a.then((r) => console.log(r)); // 1
```
