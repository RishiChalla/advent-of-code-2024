// Note - this script uses lodash

const example = `MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX`;

function part1Solution(input) {
	let count = 0;
	const text = input.split('\n'); // Original text for horizontal search
	const transposed = _.zip(...text.map(x => x.split(''))).map(x => x.join('')); // Transpose for vertical search
	const diag1 = new Array(text.length*2-1).fill().map((e, i) => (
		_.zip(_.range(i+1), _.rangeRight(i+1))
			.filter(([x, y]) => x >= 0 && x < text.length && y >= 0 && y < text.length)
			.map(([x, y]) => text[x][y])
			.join('')
	));
	const diag2 = new Array(text.length*2-1).fill().map((e, i) => (
		_.zip(_.range(i+1), _.rangeRight(i+1))
			.filter(([x, y]) => x >= 0 && x < text.length && y >= 0 && y < text.length)
			.map(([x, y]) => text[text.length - x - 1][y])
			.join('')
	));
	[text, transposed, diag1, diag2].forEach((grid) => grid.forEach((line) => {
		count += [...(line.match(/XMAS/g) || []), ...(line.match(/SAMX/g) || [])].length;
	}));
	return count;
}

console.log(part1Solution(example)); // 18
console.log(part1Solution(fullText)); // 2593

function part2Solution(input) {
	const text = input.split('\n');
	const permutations = [
		/M.S\n.A.\nM.S/,
		/M.M\n.A.\nS.S/,
		/S.M\n.A.\nS.M/,
		/S.S\n.A.\nM.M/,
	];
	let count = 0;
	_.range(text.length - 2).forEach(x => {
		_.range(text.length - 2).forEach(y => {
			const grid = [0, 1, 2].map(i => text[x + i].slice(y, y + 3)).join('\n'); // 3x3 section as newline separated string
			if (permutations.some(regex => grid.match(regex))) count++;
		})
	});
	return count;
}

console.log(part2Solution(example)); // 9
console.log(part2Solution(fullText)); // 1950
