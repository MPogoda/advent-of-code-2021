console.time("parser");
const filename = "input";
// const filename = "testinput";

const rawData = require("fs").readFileSync(filename, "UTF-8").split("\n");
rawData.pop();
const data = rawData;
console.timeEnd("parser");

console.time("Part 1");
(() => {
  const openPairs = { "{": "}", "(": ")", "[": "]", "<": ">" };
  const closePairs = { "}": "{", ")": "(", "]": "[", ">": "<" };
  const score = { "{": 1197, "(": 3, "[": 57, "<": 25137 };
  function getScore(line) {
    const open = [];
    for (const ch of line) {
      if (ch in openPairs) {
        open.push(ch);
      } else {
        const ch_ = open.pop();
        if (ch_ !== closePairs[ch]) {
          return score[closePairs[ch]];
        }
      }
    }
    return 0;
  }
  console.log(data.map(getScore).reduce((acc, v) => acc + v, 0));
})();
console.timeEnd("Part 1");

console.time("Part 2");
(() => {
  const openPairs = { "{": "}", "(": ")", "[": "]", "<": ">" };
  const closePairs = { "}": "{", ")": "(", "]": "[", ">": "<" };
  const score = { "{": 3, "(": 1, "[": 2, "<": 4 };
  function getScore(line) {
    const open = [];
    for (const ch of line) {
      if (ch in openPairs) {
        open.push(ch);
      } else {
        const ch_ = open.pop();
        if (ch_ !== closePairs[ch]) {
          return 0;
        }
      }
    }
    return open.reverse().reduce((acc, v) => acc * 5 + score[v], 0);
  }
  const scores = data
    .map(getScore)
    .filter((s) => !!s)
    .sort((a, b) => a - b);
  console.log(scores[Math.floor(scores.length / 2)]);
})();
console.timeEnd("Part 2");
