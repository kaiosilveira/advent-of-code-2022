class Valve {
  constructor({ label, rate, leadsTo }) {
    this.label = label;
    this.rate = rate;
    this.leadsTo = leadsTo;
    this.leadsToLabels = leadsTo;
  }
}

function parseValve(str) {
  const parts = str.split(";");
  const valveLabelAndRate = parts[0].split("has");
  const label = valveLabelAndRate[0].replace("Valve ", "").trim();
  const rate = parseInt(valveLabelAndRate[1].replace(/\D/gi, "").trim());
  const targetParts = parts[1].split(/(valves|valve)/gi);
  const targetValves = targetParts[targetParts.length - 1].trim();
  const leadsTo = targetValves.includes(",")
    ? targetValves.split(",").map((i) => i.trim())
    : [targetValves];

  return new Valve({
    label: label.slice(0, 1).toLowerCase(),
    rate,
    leadsTo: leadsTo.map((i) => i.slice(0, 1).toLowerCase()),
  });
}

function DFS(valves, start, target) {
  console.log(`visiting ${start.label}`);
  let moves = 0;

  if (start.label === target) {
    console.log("Found!", start);
    return;
  }

  start.visited = true;

  let availableValves = valves
    .filter((v) => start.leadsTo.some((l) => l.label === v.label) && !v.visited)
    .sort((v1, v2) => (v1.rate > v2.rate ? -1 : 1));

  availableValves.forEach((v) => DFS(valves, v, target));
}

function BFS(valves, start, target) {
  const queue = [start];
  let weight = 0;
  let moves = 0;
  while (queue.length) {
    let item = queue.pop();

    if (item.visited) continue;
    if (item.label === target) {
      console.log("Found!", item);
      break;
    }

    console.log(`visiting ${item.label}`);
    moves++;
    weight += item.rate;
    item.visited = true;
    let availableValves = valves.filter((v) =>
      item.leadsTo.some((l) => l.label === v.label)
    );

    availableValves
      .sort((v1, v2) => (v1.rate > v2.rate ? -1 : 1))
      .forEach((v) => {
        // console.log(`enqueuing ${v.label}`);
        queue.push(v);
      });
  }

  console.log(`weight: ${weight} moves: ${moves}`);
}

module.exports = { Valve, parseValve, DFS, BFS };
