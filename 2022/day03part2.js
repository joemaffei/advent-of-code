/**
 * ChatGPT is completely bogged down at the moment. This is my solution:
 */

(await (await fetch('https://adventofcode.com/2022/day/3/input')).text())
.split('\n')
.filter(Boolean)
.reduce((a,_,i,r)=>i%3?a:[...a,r.slice(i,i+3)], [])
.map(x=>x.reduce((a, b) => [...a].filter(c => [...b].includes(c)))[0])
.reduce((a,c) => a + (c.charCodeAt(0) - 96 < 0 ? c.charCodeAt(0) - 38 : c.charCodeAt(0) - 96), 0)
