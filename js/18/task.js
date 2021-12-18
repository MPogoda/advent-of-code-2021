console.time("parser");
const filename = "input";
// const filename = "testinput";

const rawData = require("fs").readFileSync(filename, "UTF-8").split("\n");
rawData.pop();

function parseNode(line) {
  return JSON.parse(line);
}
const data = rawData.map(parseNode);
console.timeEnd("parser");

function isLeaf(node) {
  return typeof node === "number";
}

function addLeft(node, v) {
  if (isLeaf(node)) {
    return node + v;
  }
  const [lhs, rhs] = node;
  return [addLeft(lhs, v), rhs];
}

function addRight(node, v) {
  if (isLeaf(node)) {
    return node + v;
  }
  const [lhs, rhs] = node;
  return [lhs, addRight(rhs, v)];
}

function traverseForExplode(node, depth = 0) {
  if (isLeaf(node)) {
    return [false, node];
  }
  const [node_lhs, node_rhs] = node;
  if (isLeaf(node_lhs) && isLeaf(node_rhs)) {
    if (depth >= 4) {
      return [true, 0, node_lhs, node_rhs];
    }
    return [false, node];
  }

  const [lhs_exploded, lhs, lhs_left = 0, lhs_right = 0] = traverseForExplode(
    node_lhs,
    depth + 1
  );
  if (lhs_exploded) {
    return [true, [lhs, addLeft(node_rhs, lhs_right)], lhs_left, 0];
  }

  const [rhs_exploded, rhs, rhs_left = 0, rhs_right = 0] = traverseForExplode(
    node_rhs,
    depth + 1
  );
  if (rhs_exploded) {
    return [true, [addRight(node_lhs, rhs_left), rhs], 0, rhs_right];
  }
  return [false, node];
}

function split(node) {
  if (isLeaf(node)) {
    if (node < 10) {
      return [false, node];
    }
    const v = node / 2;
    return [true, [Math.floor(v), Math.ceil(v)]];
  }
  const [node_lhs, node_rhs] = node;
  const [lhs_splitted, lhs] = split(node_lhs);
  if (lhs_splitted) {
    return [true, [lhs, node_rhs]];
  }
  const [rhs_splitted, rhs] = split(node_rhs);
  if (rhs_splitted) {
    return [true, [node_lhs, rhs]];
  }

  return [false, node];
}

function reduce(node) {
  const [exploded, after_explode] = traverseForExplode(node);
  if (exploded) {
    return reduce(after_explode);
  }
  const [splitted, after_split] = split(node);
  if (splitted) {
    return reduce(after_split);
  }
  return node;
}

function magnitude(node) {
  if (isLeaf(node)) {
    return node;
  }
  const [node_lhs, node_rhs] = node;
  return 3 * magnitude(node_lhs) + 2 * magnitude(node_rhs);
}

console.time("Part 1");
(() => {
  const ans = data.slice(1).reduce((lhs, rhs) => reduce([lhs, rhs]), data[0]);
  console.log(magnitude(ans));
})();
console.timeEnd("Part 1");

console.time("Part 2");
(() => {
  let max = 0;
  for (let i = 0; i < data.length; ++i) {
    for (let j = 0; j < data.length; ++j) {
      if (i === j) continue;
      max = Math.max(max, magnitude(reduce([data[i], data[j]])));
    }
  }
  console.log(max);
})();
console.timeEnd("Part 2");
