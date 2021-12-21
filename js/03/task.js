console.time("parser");
const filename = "input";
// const filename = "testinput";
const rawData = require("fs")
  .readFileSync(filename, "UTF-8")
  .split("\n");
rawData.pop();

console.timeEnd("parser");

console.time("Part 1");
(() => {
  const total = rawData.length;
  const rates = Array(rawData[0].length).fill(0);
  for (const d of rawData) {
    for (let i = 0; i < d.length; ++i) {
      rates[i] += d[i] === "1" ? 1 : 0;
    }
  }
  let gammaStr = "";
  let epsilonStr = "";
  for (const rate of rates) {
    gammaStr += rate > total / 2 ? "1" : "0";
    epsilonStr += rate > total / 2 ? "0" : "1";
  }

  const gamma = parseInt(gammaStr, 2);
  const epsilon = parseInt(epsilonStr, 2);

  console.log({
    gamma,
    gammaStr,
    epsilon,
    epsilonStr,
    answer: gamma * epsilon
  });
})();
console.timeEnd("Part 1");

console.time("Part 2");
(() => {
  function mostCommonBit(data, i) {
    let rate = 0;
    for (const d of data) {
      rate += d[i] === "1";
    }

    return rate >= data.length / 2;
  }

  let oxygens = rawData;
  for (let i = 0; i < oxygens[0].length; ++i) {
    if (oxygens.length === 1) break;
    const m = mostCommonBit(oxygens, i);
    oxygens = oxygens.filter(s => s[i] === (m ? "1" : "0"));
  }

  let co2s = rawData;
  for (let i = 0; i < co2s[0].length; ++i) {
    if (co2s.length === 1) break;
    const m = mostCommonBit(co2s, i);
    co2s = co2s.filter(s => s[i] !== (m ? "1" : "0"));
  }

  const [oxygenStr] = oxygens;
  const [co2Str] = co2s;
  const oxygen = parseInt(oxygenStr, 2);
  const co2 = parseInt(co2Str, 2);

  console.log({
    oxygenStr,
    co2Str,
    oxygen,
    co2,
    answer: oxygen * co2
  });
})();
console.timeEnd("Part 2");
