console.time("parser");
const filename = "input";
// const filename = "testinput";

const rawData = require("fs").readFileSync(filename, "UTF-8").split("\n");
rawData.pop();
const data = rawData.map((line) => {
  const [patterns, outputs] = line.split(" | ");
  return {
    patterns: patterns.split(" ").map((p) => Array.from(p).sort().join("")),
    outputs: outputs.split(" ").map((p) => Array.from(p).sort().join("")),
  };
});
console.timeEnd("parser");

console.time("Part 1");
(() => {
  let ans = 0;
  for (const { outputs } of data) {
    for (const output of outputs) {
      if ([2, 3, 4, 7].includes(output.length)) {
        ++ans;
      }
    }
  }
  console.log(ans);
})();
console.timeEnd("Part 1");

function solve({ patterns, outputs }) {
  const patternToDigit = new Map();
  const digitToPattern = Array(10);
  for (const pattern of patterns) {
    switch (pattern.length) {
      case 2:
        patternToDigit.set(pattern, 1);
        digitToPattern[1] = new Set(pattern);
        break;
      case 3:
        patternToDigit.set(pattern, 7);
        digitToPattern[7] = new Set(pattern);
        break;
      case 4:
        patternToDigit.set(pattern, 4);
        digitToPattern[4] = new Set(pattern);
        break;
      case 7:
        patternToDigit.set(pattern, 8);
        digitToPattern[8] = new Set(pattern);
        break;
    }
  }

  // top edge
  const [T] = (() => {
    return Array.from(digitToPattern[7]).filter(
      (x) => !digitToPattern[1].has(x)
    );
  })();

  // left bottom
  const [LB] = (() => {
    const almost9 = new Set(digitToPattern[4]);
    almost9.add(T);
    for (const pattern of patterns) {
      if (patternToDigit.has(pattern)) {
        continue;
      }
      const thisPattern = new Set(pattern);
      let found = true;
      for (const rr of almost9) {
        if (!thisPattern.has(rr)) {
          found = false;
          break;
        }
        thisPattern.delete(rr);
      }
      if (found && thisPattern.size === 1) {
        digitToPattern[9] = new Set(pattern);
        patternToDigit.set(pattern, 9);
        return Array.from(digitToPattern[8]).filter(
          (x) => !digitToPattern[9].has(x)
        );
      }
    }
  })();

  // right bottom
  const [RB] = (() => {
    const digit1 = Array.from(digitToPattern[1]);
    for (const [rt, rb] of [digit1, [digit1[1], digit1[0]]]) {
      const maybe6 = Array.from(digitToPattern[8])
        .filter((x) => x !== rt)
        .join("");
      for (const pattern of patterns) {
        if (patternToDigit.has(pattern)) {
          continue;
        }
        if (pattern === maybe6) {
          digitToPattern[6] = new Set(pattern);
          patternToDigit.set(pattern, 6);
          return [rb];
        }
      }
    }
  })();

  (() => {
    const pattern = Array.from(digitToPattern[6])
      .filter((x) => x !== LB)
      .join("");
    digitToPattern[5] = new Set(pattern);
    patternToDigit.set(pattern, 5);
  })();

  const LT = (() => {
    const almost2 = new Set(digitToPattern[8]);
    almost2.delete(RB);
    for (const pattern of patterns) {
      if (patternToDigit.has(pattern)) {
        continue;
      }
      const t = new Set(almost2);
      let found = true;
      for (const rr of pattern) {
        if (!t.has(rr)) {
          found = false;
          break;
        }
        t.delete(rr);
      }
      if (found && t.size === 1) {
        digitToPattern[2] = new Set(pattern);
        patternToDigit.set(pattern, 2);
        return t.values().next().value;
      }
    }
  })();

  (() => {
    const pattern = Array.from(digitToPattern[8])
      .filter((x) => x !== LB && x !== LT)
      .join("");
    digitToPattern[3] = new Set(pattern);
    patternToDigit.set(pattern, 3);
  })();

  (() => {
    for (const pattern of patterns) {
      if (patternToDigit.has(pattern)) {
        continue;
      }
      patternToDigit.set(pattern, 0);
      digitToPattern[0] = new Set(pattern);
      return;
    }
  })();

  return parseInt(outputs.map((output) => patternToDigit.get(output)).join(""));
}
console.time("Part 2");
(() => {
  console.log(data.map(solve).reduce((acc, v) => acc + v, 0));
})();
console.timeEnd("Part 2");
