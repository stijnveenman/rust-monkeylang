let globalSeed = 50;

let minusOne = fn() {
	let num = 1;
	globalSeed - num;
}

let minusTwo = fn() {
	let num = 2;
	globalSeed - num;
}

minusOne() + minusOne()
