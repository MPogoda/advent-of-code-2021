console.time("parser");
const filename = "input";
// const filename = "testinput";

const [rawData] = require("fs")
  .readFileSync(filename, "UTF-8")
  .split("\n");
// const rawData = "8A004A801A8002F478";
// const rawData = "620080001611562C8802118E34";
// const rawData = "C0015000016115A2E0802F182340";
// const rawData = "C200B40A82";
const s = rawData.split("").flatMap(ch =>
  parseInt(ch, 16)
    .toString(2)
    .padStart(4, "0")
    .split("")
);

function parseSubPacketsByLength(pos) {
  const packets = [];
  const length = parseInt(s.slice(pos, pos + 15).join(""), 2);
  pos += 15;
  const maxPos = pos + length;
  while (pos !== maxPos) {
    const [packet, newPos] = parse(pos);
    pos = newPos;
    packets.push(packet);
  }
  return [packets, pos];
}

function parseSubPacketsByNum(pos) {
  const packets = [];
  const num = parseInt(s.slice(pos, pos + 11).join(""), 2);
  pos += 11;
  for (let i = 0; i < num; ++i) {
    const [packet, newPos] = parse(pos);
    pos = newPos;
    packets.push(packet);
  }
  return [packets, pos];
}

function parse(pos) {
  const result = {};
  result.version = parseInt(s.slice(pos, pos + 3).join(""), 2);
  pos += 3;
  result.id = parseInt(s.slice(pos, pos + 3).join(""), 2);
  pos += 3;
  if (result.id === 4) {
    let value = "";
    while (true) {
      const isLast = s[pos];
      ++pos;
      value += s.slice(pos, pos + 4).join("");
      pos += 4;
      if (isLast === "0") {
        break;
      }
    }
    result.value = parseInt(value, 2);
  } else {
    const i = s[pos];
    ++pos;
    if (i === "0") {
      const [packets, newPos] = parseSubPacketsByLength(pos);
      result.packets = packets;
      pos = newPos;
    } else {
      const [packets, newPos] = parseSubPacketsByNum(pos);
      result.packets = packets;
      pos = newPos;
    }
  }
  return [result, pos];
}
const [thePacket] = parse(0);
// console.debug(JSON.stringify(thePacket, undefined, 2));
console.timeEnd("parser");

console.time("Part 1");
(() => {
  function getVersionSum({ version, packets = [] }) {
    return packets.reduce((acc, v) => acc + getVersionSum(v), version);
  }
  console.log(getVersionSum(thePacket));
})();
console.timeEnd("Part 1");

console.time("Part 2");
(() => {
  function process(packet) {
    switch (packet.id) {
      case 0:
        return packet.packets.reduce((acc, v) => acc + process(v), 0);
      case 1:
        return packet.packets.reduce((acc, v) => acc * process(v), 1);
      case 2:
        return Math.min(...packet.packets.map(process));
      case 3:
        return Math.max(...packet.packets.map(process));
      case 4:
        return packet.value;
      case 5: {
        const [lhs, rhs] = packet.packets.map(process);
        return lhs > rhs ? 1 : 0;
      }
      case 6: {
        const [lhs, rhs] = packet.packets.map(process);
        return lhs < rhs ? 1 : 0;
      }
      case 7: {
        const [lhs, rhs] = packet.packets.map(process);
        return lhs === rhs ? 1 : 0;
      }
    }
  }

  console.log(process(thePacket));
})();
console.timeEnd("Part 2");
