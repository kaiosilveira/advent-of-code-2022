const fs = require("fs");
const { parseValve, DFS, BFS } = require("./solution");

const alphabet = "abcdefghijklmnopqrstuvwxyz";

function partOne(input) {
  let valves = input.map(parseValve);

  valves.forEach((valve) => {
    valve.leadsTo = valves.filter((v) => valve.leadsTo.includes(v.label));
  });

  const mostValuablePaths = valves.sort((v1, v2) =>
    v1.rate > v2.rate ? -1 : 1
  );
  const start = valves.find((v) => v.label == "a");

  console.log(mostValuablePaths);
  let itemLabel;
  let visitedNodes = new Set();
  while (item !== "a") {
    const item = start.leadsTo.find((v) => v.label === "b");
    visitedNodes.add(item.label);
  }
  BFS(valves, start, mostValuablePaths[1].label);
}

const input = fs.readFileSync("sample.txt").toString().split("\n");
partOne(input);
