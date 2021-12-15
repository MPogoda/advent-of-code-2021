console.time("parser");
const filename = "input";
// const filename = "testinput";

const rawData = require("fs").readFileSync(filename, "UTF-8").split("\n");
rawData.pop();
const data = rawData.map((line) => line.split("").map(Number));
console.timeEnd("parser");
console.time("Part 1");
(() => {
  function search(ds) {
    const N = ds.length;
    const mem = Array(N)
      .fill(0)
      .map(() => Array(N).fill(0));
    mem[N - 1][N - 1] = ds[N - 1][N - 1];
    for (let i = N - 1; i >= 0; --i) {
      for (let j = N - 1; j >= 0; --j) {
        if (i === N - 1 && j === N - 1) continue;
        mem[i][j] =
          ds[i][j] +
          Math.min(mem[i + 1]?.[j] ?? Infinity, mem[i][j + 1] ?? Infinity);
      }
    }

    return mem[0][0] - ds[0][0];
  }
  console.log(search(data));
})();
console.timeEnd("Part 1");

console.time("Part 2");
(() => {
  const shortestPath = (ds) => {
    const N = ds.length;
    const adjacent = [
      [1, 0],
      [0, 1],
      [-1, 0],
      [0, -1],
    ];
    const queue = [{ x: 0, y: 0, cost: 0 }];
    const visited = new Set();
    while (queue.length) {
      const { x: x0, y: y0, cost } = queue.shift();
      if (y0 === N - 1 && x0 === N - 1) {
        return cost;
      }

      for (const [x, y] of adjacent
        .map(([dx, dy]) => [dx + x0, dy + y0])
        .filter(([x, y]) => ds[y]?.[x])
        .filter(([x, y]) => !visited.has(N * y + x))) {
        visited.add(N * y + x);
        queue.push({ x, y, cost: cost + ds[y][x] });
      }
      queue.sort((a, b) => a.cost - b.cost);
    }
  };

  const N = data.length;
  const biggerData = Array(5 * N)
    .fill(0)
    .map((_, i) =>
      Array(5 * N)
        .fill(0)
        .map((_, j) => {
          const di = Math.trunc(i / N);
          const dj = Math.trunc(j / N);
          const value = data[i % N][j % N] + di + dj;

          return value > 9 ? value - 9 : value;
        })
    );
  console.log(shortestPath(biggerData));
})();
console.timeEnd("Part 2");
