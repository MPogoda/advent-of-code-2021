console.time("parser");
const filename = "input";
const rawData = require("fs").readFileSync(filename, "UTF-8").split("\n");
rawData.pop();

const data = rawData.map((line) => {
  const [dir, v] = line.split(" ");
  return { dir, v: Number(v) };
});
console.timeEnd("parser");

console.time("Part 1");
(() => {
  let x = 0;
  let y = 0;
  for (const { dir, v } of data) {
    if (dir === "forward") {
      x += v;
    }
    if (dir === "down") {
      y += v;
    }
    if (dir === "up") {
      y -= v;
    }
  }
  console.log({ x, y, ans: x * y });
})();
console.timeEnd("Part 1");

console.time("Part 2");
(() => {
  let x = 0;
  let y = 0;
  let aim = 0;
  for (const { dir, v } of data) {
    if (dir === "forward") {
      x += v;
      y += aim * v;
    }
    if (dir === "down") {
      aim += v;
    }
    if (dir === "up") {
      aim -= v;
    }
  }
  console.log({ x, y, ans: x * y });
})();
console.timeEnd("Part 2");
