console.time("parser");
const filename = "input";
// const filename = "testinput";

const rawData = require("fs").readFileSync(filename, "UTF-8").split("\n");
rawData.pop();
const data = rawData.map((line) => line.split("").map(Number));
console.timeEnd("parser");

function getNeighbours(x, y) {
  return [
    [x - 1, y - 1],
    [x - 1, y],
    [x - 1, y + 1],
    [x, y - 1],
    [x, y + 1],
    [x + 1, y - 1],
    [x + 1, y],
    [x + 1, y + 1],
  ];
}
function evolve(ds) {
  const queue = [];
  for (let i = 0; i < ds.length; ++i) {
    for (let j = 0; j < ds.length; ++j) {
      ds[i][j] += 1;
      if (ds[i][j] > 9) {
        queue.push([i, j]);
        ds[i][j] = 0;
      }
    }
  }

  let ans = 0;
  while (queue.length) {
    const [x, y] = queue.shift();
    ++ans;
    for (const [i, j] of getNeighbours(x, y)) {
      if (!ds[i]?.[j]) continue;

      ds[i][j] += 1;
      if (ds[i][j] > 9) {
        queue.push([i, j]);
        ds[i][j] = 0;
      }
    }
  }

  return ans;
}
console.time("Part 1");
(() => {
  let result = 0;
  const ds = data.map((d) => Array.from(d));
  for (let i = 0; i < 100; ++i) {
    result += evolve(ds);
  }
  console.log(result);
})();
console.timeEnd("Part 1");

console.time("Part 2");
(() => {
  const ds = data.map((d) => Array.from(d));
  const desiredAns = ds.length * ds[0].length;
  let i = 0;
  while (true) {
    ++i;
    if (evolve(ds) === desiredAns) {
      console.log(i);
      return;
    }
  }
})();
console.timeEnd("Part 2");
