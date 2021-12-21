console.time("parser");
const filename = "input";
// const filename = "testinput";

const [rawTemplate, rawRules] = require("fs")
  .readFileSync(filename, "UTF-8")
  .split("\n\n");
const rules = rawRules.split("\n").map(v => v.split(" -> "));
rules.pop();

const rulesMap = new Map(rules);

const inputTemplate = new Map();
for (let i = 0; i < rawTemplate.length - 1; ++i) {
  const slice = rawTemplate.slice(i, i + 2);
  inputTemplate.set(slice, (inputTemplate.get(slice) ?? 0) + 1);
}

console.timeEnd("parser");

function evolve(template) {
  const nextTemplate = new Map();
  for (const [k, v] of template.entries()) {
    const middle = rulesMap.get(k);
    if (!middle) {
      nextTemplate.set(k, (nextTemplate.get(k) ?? 0) + v);
    } else {
      const left = k[0] + middle;
      const right = middle + k[1];

      nextTemplate.set(left, (nextTemplate.get(left) ?? 0) + v);
      nextTemplate.set(right, (nextTemplate.get(right) ?? 0) + v);
    }
  }
  return nextTemplate;
}

function computeAnswer(template) {
  const frequencies = new Map();
  frequencies.set(rawTemplate.slice(-1)[0], 1);
  for (const [k, v] of template.entries()) {
    frequencies.set(k[0], (frequencies.get(k[0]) ?? 0) + v);
  }
  let [min, max] = [Infinity, 0];
  for (const k of frequencies.keys()) {
    const v = frequencies.get(k);
    if (v > max) {
      max = v;
    }
    if (v < min) {
      min = v;
    }
  }
  return max - min;
}

console.time("Part 1");
(() => {
  const template = Array(10)
    .fill(0)
    .reduce(acc => evolve(acc), inputTemplate);
  console.log(computeAnswer(template));
})();
console.timeEnd("Part 1");

console.time("Part 2");
(() => {
  const template = Array(40)
    .fill(0)
    .reduce(acc => evolve(acc), inputTemplate);
  console.log(computeAnswer(template));
})();
console.timeEnd("Part 2");
