# Enigma

An interpreter built in Go for the Enigma programming language.

The Enigma Programming Language has the following features:

* C-like syntax
* Variables binding
* Integers and booleans
* Arithmetic expressions
* Built-in functions
* First-class and higher order functions
* Closures
* A string data structure
* An array data structure
* A hash data structure

```C
let age = 1;
let name = "Enigma";
let result = 10 * (20 / 2);
let myArray = [1, 2, 3, 4, 5];
let ligma = {"name": "Ligma", "age": 28 };

myArray[0] // => 1
ligma["name"] // => "Ligma"

let add = fn(a,b) { a + b; };
add(1,2);

let fibonacci = fn(x) {
	if (x == 0) {
		0
	} else {
		if (x == 1) {
			1
		} else {
			fibonacci(x - 1) + fibonacci(x - 2);
		}
	}
};

let twice = fn(f, x) {
	return f(f(x));
};
let addTwo = fn(x) {
	return x + 2;
};
twice(addTwo, 2); // => 6
```