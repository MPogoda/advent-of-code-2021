console.time("parser");
const filename = "input";
// const filename = "testinput";

const rawData = require("fs")
  .readFileSync(filename, "UTF-8")
  .split("\n");
rawData.pop();
const data = rawData.map(row => row.split("").map(Number));
console.timeEnd("parser");

function neighbours(i, j) {
  const result = [];
  if (i < data.length - 1) {
    result.push(data[i + 1][j]);
  }
  if (i > 0) {
    result.push(data[i - 1][j]);
  }
  if (j < data[0].length - 1) {
    result.push(data[i][j + 1]);
  }
  if (j > 0) {
    result.push(data[i][j - 1]);
  }
  return result;
}
console.time("Part 1");
(() => {
  let ans = 0;
  for (let i = 0; i < data.length; ++i) {
    for (let j = 0; j < data[0].length; ++j) {
      const ns = neighbours(i, j);
      if (ns.every(n => n > data[i][j])) {
        ans += data[i][j] + 1;
      }
    }
  }
  console.log(ans);
})();
console.timeEnd("Part 1");

console.time("Part 2");
(() => {
  function getBasin(i, j) {
    const queue = [[i, j]];
    let size = 0;
    while (queue.length) {
      const [x, y] = queue.shift();
      if (data[x][y] === 9) continue;
      size += 1;
      data[x][y] = 9;
      if (x < data.length - 1) {
        queue.push([x + 1, y]);
      }
      if (y < data[0].length - 1) {
        queue.push([x, y + 1]);
      }
      if (x > 0) {
        queue.push([x - 1, y]);
      }
      if (y > 0) {
        queue.push([x, y - 1]);
      }
    }
    return size;
  }

  const sizes = [];
  for (let i = 0; i < data.length; ++i) {
    for (let j = 0; j < data[0].length; ++j) {
      if (data[i][j] === 9) continue;
      sizes.push(getBasin(i, j));
    }
  }
  console.log(
    sizes
      .sort((a, b) => a - b)
      .slice(-3)
      .reduce((acc, v) => acc * v, 1)
  );
})();
console.timeEnd("Part 2");
