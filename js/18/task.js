console.time("parser");
const filename = "input";
// const filename = "testinput";

const rawData = require("fs").readFileSync(filename, "UTF-8").split("\n");
rawData.pop();

function parseNode(line) {
  const r = {};
  const stack = [];
  let current = r;
  for (const ch of line) {
    switch (ch) {
      case "[": {
        current.lhs = {};
        stack.push(current);
        current = current.lhs;
        break;
      }
      case ",": {
        const top = stack[stack.length - 1];
        top.rhs = {};
        current = top.rhs;
        break;
      }
      case "]": {
        current = stack.pop();
        break;
      }
      default: {
        current.value = Number(ch);
      }
    }
  }

  return r;
}

function addLeft(node, v) {
  if (node.value != null) {
    return { value: node.value + v };
  }
  return { ...node, lhs: addLeft(node.lhs, v) };
}

function addRight(node, v) {
  if (node.value != null) {
    return { value: node.value + v };
  }
  return { ...node, rhs: addRight(node.rhs, v) };
}

function traverseForExplode(node, depth = 0) {
  if (node.value != null) {
    return [false, node];
  }
  if (node.lhs.value != null && node.rhs.value != null) {
    if (depth >= 4) {
      return [true, { value: 0 }, node.lhs.value, node.rhs.value];
    }
    return [false, node];
  }

  const [lhs_exploded, lhs, lhs_left = 0, lhs_right = 0] = traverseForExplode(
    node.lhs,
    depth + 1
  );
  if (lhs_exploded) {
    return [true, { lhs, rhs: addLeft(node.rhs, lhs_right) }, lhs_left, 0];
  }

  const [rhs_exploded, rhs, rhs_left = 0, rhs_right = 0] = traverseForExplode(
    node.rhs,
    depth + 1
  );
  if (rhs_exploded) {
    return [true, { lhs: addRight(node.lhs, rhs_left), rhs }, 0, rhs_right];
  }
  return [false, node];
}

function split(node) {
  if (node.value != null) {
    if (node.value < 10) {
      return [false, node];
    }
    const v = node.value / 2;
    return [
      true,
      { lhs: { value: Math.floor(v) }, rhs: { value: Math.ceil(v) } },
    ];
  }
  const [lhs_splitted, lhs] = split(node.lhs);
  if (lhs_splitted) {
    return [true, { ...node, lhs }];
  }
  const [rhs_splitted, rhs] = split(node.rhs);
  if (rhs_splitted) {
    return [true, { ...node, rhs }];
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
  if (node.value != null) {
    return node.value;
  }
  return 3 * magnitude(node.lhs) + 2 * magnitude(node.rhs);
}

console.timeEnd("parser");

console.time("Part 1");
(() => {
  const ans = rawData
    .slice(1)
    .reduce(
      (lhs, v) => reduce({ lhs, rhs: parseNode(v) }),
      parseNode(rawData[0])
    );
  console.log(magnitude(ans));
})();
console.timeEnd("Part 1");

console.time("Part 2");
(() => {
  const snails = rawData.map(parseNode);
  let max = 0;
  for (let i = 0; i < snails.length; ++i) {
    for (let j = 0; j < snails.length; ++j) {
      if (i === j) continue;
      max = Math.max(
        max,
        magnitude(reduce({ lhs: snails[i], rhs: snails[j] }))
      );
    }
  }
  console.log(max);
})();
console.timeEnd("Part 2");
