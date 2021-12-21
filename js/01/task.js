console.time("parser");
const filename = "input";
// const filename = "testinput";
const rawData = require("fs")
  .readFileSync(filename, "UTF-8")
  .split("\n");
rawData.pop();

const data = rawData.map(x => Number(x));
console.timeEnd("parser");

console.time("Part 1");
(() => {
  let c = 0;
  let prev = Infinity;
  for (const x of data) {
    if (prev < x) {
      ++c;
    }
    prev = x;
  }
  console.log(c);
})();
console.timeEnd("Part 1");

console.time("Part 2");
(() => {
  function sum(x) {
    return x.reduce((acc, v) => acc + v, 0);
  }
  let c = 0;
  const prev = [];
  const current = [];
  for (const x of data) {
    current.push(x);
    if (current.length > 3) {
      current.shift();
    }

    const prevSum = prev.length === 3 ? sum(prev) : Infinity;
    const currentSum = sum(current);

    if (prevSum < currentSum) {
      ++c;
    }
    prev.push(x);
    if (prev.length > 3) {
      prev.shift();
    }
  }
  console.log(c);
})();
console.timeEnd("Part 2");
