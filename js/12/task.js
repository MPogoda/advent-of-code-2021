console.time("parser");
const filename = "input";
// const filename = "testinput1";

const rawData = require("fs").readFileSync(filename, "UTF-8").split("\n");
rawData.pop();

const map = {};
for (const row of rawData) {
  const [a, b] = row.split("-");
  map[a] = map[a] ?? [];
  map[b] = map[b] ?? [];
  if (b !== "start" && a !== "end") {
    map[a].push(b);
  }
  if (a !== "start" && b !== "end") {
    map[b].push(a);
  }
}
console.timeEnd("parser");

console.time("Part 1");
(() => {
  const knownPaths = new Set();
  function go({ currentPath, node }) {
    if (node === "end") {
      knownPaths.add(currentPath.join("+"));
      return;
    }
    for (const connection of map[node]) {
      if (
        !currentPath.includes(connection) ||
        connection.toUpperCase() === connection
      ) {
        go({ currentPath: [...currentPath, connection], node: connection });
      }
    }
  }
  go({ currentPath: ["start"], node: "start" });
  console.log(knownPaths.size);
})();
console.timeEnd("Part 1");

console.time("Part 2");
(() => {
  const knownPaths = new Set();
  const queue = [{ path: [], node: "start", visitedTwice: false }];
  while (queue.length) {
    const { node, path, visitedTwice } = queue.shift();
    const currentPath = [...path, node];
    if (node === "end") {
      knownPaths.add(currentPath.join("+"));
      continue;
    }
    for (const connection of map[node]) {
      const visited =
        connection.toUpperCase() !== connection &&
        currentPath.includes(connection);
      if (visited && visitedTwice) {
        continue;
      }

      queue.push({
        path: currentPath,
        node: connection,
        visitedTwice: visited || visitedTwice,
      });
    }
  }
  console.log(knownPaths.size);
})();
console.timeEnd("Part 2");
