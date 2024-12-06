
const exampleRules = `47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13`;

const exampleOrders = `75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47`;

function part1Solution(rawRules, rawOrders) {
	// Each rule is an edge in a directed graph. We assume it is acyclic.
	// We must affirm the orders are in topological order.
	const rules = rawRules.split('\n') // Map of pages to a list of other pages it must appear before
		.map(x => x.split('|').map(Number))
		.reduce((obj, [page1, page2]) => ({
			...obj, [page1]: [...(obj[page1] || []), page2],
		}), {});
	
	return rawOrders.split('\n')
		.map(x => x.split(',').map(Number))
		.filter(pages => pages.slice(0,-1).every((page, i) => !rules[pages[i+1]] || !rules[pages[i+1]].includes(page)))
		.map(pages => pages[Math.floor(pages.length / 2)])
		.reduce((a, b) => a + b, 0);
}

console.log(part1Solution(exampleRules, exampleOrders)); // 143
console.log(part1Solution(rules, orders)); // 4185

function part2Solution(rawRules, rawOrders) {
	// Each rule is an edge in a directed graph. We assume it is acyclic.
	// We must perform a topological sort on all items not already sorted.
	const rules = rawRules.split('\n') // Map of pages to a list of other pages it must appear before
		.map(x => x.split('|').map(Number))
		.reduce((obj, [page1, page2]) => ({
			...obj, [page1]: [...(obj[page1] || []), page2],
		}), {});
	
	return rawOrders.split('\n')
		.map(x => x.split(',').map(Number))
		.filter(pages => !pages.slice(0,-1).every((page, i) => !rules[pages[i+1]] || !rules[pages[i+1]].includes(page)))
		.map(pages => pages.sort((a, b) => !rules[b] || !rules[b].includes(a) ? 1 : -1))
		.map(pages => pages[Math.floor(pages.length / 2)])
		.reduce((a, b) => a + b, 0);
}

console.log(part2Solution(exampleRules, exampleOrders)); // 123
console.log(part2Solution(rules, orders)); // 4480
