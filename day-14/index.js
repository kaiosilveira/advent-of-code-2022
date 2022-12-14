const fs = require('fs');

const input = fs.readFileSync('input.txt').toString().split('\n');

const directions = { VERTICAL: 'vertical', HORIZONTAL: 'horizontal' };
const SAND_ORIGIN = { x: 500, y: 0 };

class ToppedUpError extends Error {
  constructor() {
    super('Topped up!');
  }
}

class PointOutOfViewError extends Error {
  constructor({ x, y }) {
    super(`Failed when trying to access (${x}, ${y})`);
    this.x = x;
    this.y = y;
  }
}

function parseInput(input) {
  const result = input.map(line => {
    const parts = line.split('->').map(point => {
      const [x, y] = point.split(',').map(s => Number(s.trim()));
      return new PathItem({ x, y, filledWith: items.ROCK });
    });

    const paths = [];
    for (let i = 0; i < parts.length; i++) {
      const origin = parts[i];
      const destination = parts[i + 1];
      if (destination) {
        paths.push(new Path({ from: origin, to: destination }));
      }
    }

    return [parts, paths];
  });

  return result;
}

const items = {
  AIR: '.',
  ROCK: '#',
  SAND: 'o',
  SAND_ORIGIN: '+',
};

class PathItem {
  constructor({ x, y, filledWith = items.AIR }) {
    this.x = x;
    this.y = y;
    this.filledWith = filledWith;

    this.isLocatedAt = this.isLocatedAt.bind(this);
  }

  isLocatedAt({ x, y }) {
    return this.x === x && this.y === y;
  }
}

class GrainOfSand {
  constructor({ x, y, origin }) {
    this.x = x;
    this.y = y;
    this.origin = origin;
  }

  getNextMove() {
    const down = new GrainOfSand({ x: this.x, y: this.y + 1, origin: this.origin });
    const left = new GrainOfSand({ x: this.x - 1, y: this.y + 1, origin: this.origin });
    const right = new GrainOfSand({ x: this.x + 1, y: this.y + 1, origin: this.origin });

    return [down, left, right];
  }

  drop(gridRows) {
    const [down, left, right] = this.getNextMove();

    let sand = undefined;

    if (this.isAllowedMove(down, gridRows)) sand = down;
    else if (this.isAllowedMove(left, gridRows)) sand = left;
    else if (this.isAllowedMove(right, gridRows)) sand = right;

    if (!sand && this.y === 0) {
      throw new ToppedUpError();
    }

    return sand;
  }

  isAllowedMove(target, gridRows) {
    const match = gridRows[target.y][target.x];
    if (match === items.AIR) {
      return true;
    }

    if (!match && target.y <= gridRows[gridRows.length - 1].length) {
      throw new PointOutOfViewError({ x: target.x, y: target.y });
    }

    return false;
  }
}

class SandSpawner {
  constructor({ x, spawnCount = 0 }) {
    this.x = x;
    this.spawnCount = spawnCount;
    this.spawn = this.spawn.bind(this);
  }

  spawn() {
    this.spawnCount++;
    return new GrainOfSand({ x: this.x, y: 0, origin: this.x });
  }
}

class Path {
  constructor({ from, to }) {
    this.from = from;
    this.to = to;

    this.direction = from.x === to.x ? directions.VERTICAL : directions.HORIZONTAL;
  }

  contains({ x, y }) {
    if (this.direction === directions.VERTICAL) {
      const availableYs = Array.from(
        { length: Math.abs(this.to.y - this.from.y) + 1 },
        (_, i) => Math.min(this.to.y, this.from.y) + i
      );

      return x === this.to.x && availableYs.includes(y);
    } else if (this.direction === directions.HORIZONTAL) {
      const availableXs = Array.from(
        { length: Math.abs(this.to.x - this.from.x) + 1 },
        (_, i) => Math.min(this.to.x, this.from.x) + i
      );

      return y === this.to.y && availableXs.includes(x);
    }
  }
}

class Grid {
  constructor({ rows, paths }) {
    this.rows = rows;
    this.paths = paths;

    this.print = this.print.bind(this);
    this.set = this.set.bind(this);
  }

  print() {
    console.clear();
    this.rows.forEach(row => console.log(row.join(' ')));
    this.printLog();
    console.log('');
  }

  printLog() {
    console.log(this.log);
  }

  set(item, x, y) {
    this.rows[y][x] = item;
  }

  moveItem(p1, p2, item) {
    this.set(items.AIR, p1.x, p1.y);
    this.set(item, p2.x, p2.y);
  }

  setSpawner(spawner) {
    this.spawner = spawner;
  }

  dropSand() {
    let sand = this.spawner.spawn();

    try {
      while (sand) {
        const newSand = sand.drop(this.rows);
        if (newSand) this.moveItem(sand, newSand, items.SAND);
        sand = newSand;
      }
      this.log = `${this.spawner.spawnCount + 1} grains of sand dropped!`;
    } catch (ex) {
      if (ex instanceof PointOutOfViewError) {
        this.rows = this.rows.map((r, idx) => {
          const item = idx === this.rows.length - 1 ? items.ROCK : items.AIR;
          return [item, ...r, item];
        });

        const newSpawnerPosition = this.spawner.x + 1;
        this.moveItem(
          { x: this.spawner.x, y: 0 },
          { x: newSpawnerPosition, y: 0 },
          items.SAND_ORIGIN
        );

        this.spawner = new SandSpawner({
          x: newSpawnerPosition,
          spawnCount: this.spawner.spawnCount,
        });
      } else if (ex instanceof ToppedUpError) {
        this.toppedUp = true;
      } else {
        throw ex;
      }
    }
  }
}

function createGridRow({ y, minX, maxX, paths, defaultItem = items.AIR }) {
  const row = new Array(maxX - minX);
  for (let x = 0; x <= maxX - minX; x++) {
    let value;

    if (x === SAND_ORIGIN.x - minX && y === SAND_ORIGIN.y) value = '+';
    else if (paths.some(p => p.contains({ x: minX + x, y }))) value = items.ROCK;
    else value = defaultItem;

    row[x] = value;
  }

  return row.sort((a, b) => a < b);
}

function createGridRows(paths, minX, minY, maxX, maxY) {
  console.log(`x: ${minX} to ${maxX}`);
  console.log(`y: ${minY} to ${maxY}`);

  const gridRows = [];
  for (let y = 0; y <= maxY; y++) {
    const targetY = y;
    const row = createGridRow({ minX, maxX, y, targetY, paths });
    gridRows.push(row);
  }

  return gridRows;
}

function partTwo() {
  const info = parseInput(input);
  const paths = info.flatMap(([_, p]) => p);
  const points = info.flatMap(([p]) => p);

  const [minX, minY] = [Math.min(...points.map(p => p.x)), Math.min(...points.map(p => p.y))];
  const [maxX, maxY] = [Math.max(...points.map(p => p.x)), Math.max(...points.map(p => p.y))];

  const gridRows = createGridRows(paths, minX, minY, maxX, maxY);
  gridRows.push(createGridRow({ y: maxY + 1, minX, maxX, paths: [], defaultItem: items.AIR }));
  gridRows.push(createGridRow({ y: maxY + 2, minX, maxX, paths: [], defaultItem: items.ROCK }));

  const offsetX = minX;
  const grid = new Grid({ rows: [...gridRows], paths, minX, minY, maxX, maxY, offsetX });
  const sandSpawner = new SandSpawner({
    filledWith: items.SAND_ORIGIN,
    x: SAND_ORIGIN.x - offsetX,
  });

  grid.setSpawner(sandSpawner);

  while (!grid.toppedUp) {
    grid.dropSand();
    grid.printLog();
  }
}

partTwo();
