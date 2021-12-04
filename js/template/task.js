console.time("parser");
const filename = "input";
// const filename = "testinput";

const rawData = require("fs").readFileSync(filename, "UTF-8").split("\n");
rawData.pop();
console.timeEnd("parser");

console.time("Part 1");
(() => {})();
console.timeEnd("Part 1");

console.time("Part 2");
(() => {})();
console.timeEnd("Part 2");
