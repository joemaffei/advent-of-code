/**
 * Prompt:
 *
 * Let's use JavaScript to solve this problem. First, fetch data from
 * https://adventofcode.com/2022/day/3/input. Each line in that file represents
 * a rucksack with two compartments. The content of the first compartment is
 * represented by the first half of the characters, while the second compartment
 * is represented by the second half.
 *
 * We need to find the letter that is common between the two compartments, and
 * assign a numeric value to it. Lowercase letters from a through z are worth 1
 * through 26, while uppercase letters from A through Z are worth 27 through 52.
 *
 * We need to repeat this for every line, adding each line's common letter value
 * to a total, then console.log the total.
 *
 * Results:
 *
 * Not even close. It was so bad I don't even want to paste it here. Instead,
 * here's my console one-liner...
 */

(await (await fetch('https://adventofcode.com/2022/day/3/input')).text())
.split('\n')
.filter(Boolean)
.map(x => [x.slice(0, x.length / 2), x.slice(x.length / 2)])
.map(([a,b]) => a.split('').filter(x => b.includes(x))[0])
.reduce((a,c) => a + (c.charCodeAt(0) - 96 < 0 ? c.charCodeAt(0) - 38 : c.charCodeAt(0) - 96), 0)
