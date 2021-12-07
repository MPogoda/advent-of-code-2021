console.time("parser");
const filename = "input";
// const filename = "testinput";

const rawData = require("fs").readFileSync(filename, "UTF-8").split("\n");
rawData.pop();
const pos = {};
let min = Infinity;
let max = -Infinity;
for (const d of rawData[0].split(",").map(Number)) {
  pos[d] = (pos[d] ?? 0) + 1;
  if (d < min) {
    min = d;
  }
  if (d > max) {
    max = d;
  }
}
console.timeEnd("parser");

console.time("Part 1");
(() => {
  let ans = Infinity;
  for (let i = min; i <= max; ++i) {
    let tmp = 0;
    for (const k in pos) {
      tmp += Math.abs(Number(k) - i) * pos[k];
    }

    if (tmp < ans) {
      ans = tmp;
    }
  }
  console.log(ans);
})();
console.timeEnd("Part 1");

console.time("Part 2");
(() => {
  let ans = Infinity;
  for (let i = min; i <= max; ++i) {
    let tmp = 0;
    for (const k in pos) {
      const x = Math.abs(Number(k) - i);
      const cost = (x * (x + 1)) / 2;
      tmp += cost * pos[k];
    }

    if (tmp < ans) {
      ans = tmp;
    }
  }
  console.log(ans);
})();
console.timeEnd("Part 2");
