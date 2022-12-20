const emptyRow = ['\t', '\t', '\t', '\t', '\t', '\t', '\t'];

const HORIZONTAL_BAR_SHAPE = ['\t', '\t', '@\t', '@\t', '@\t', '@\t', '\t'];

class Grid {
  constructor() {
    this.rows = new Array(1).fill(0).map(() => new Array(7).fill(0));
  }

  placeShape() {
    this.rows.unshift([...emptyRow]);
    this.rows.unshift([...emptyRow]);
    this.rows.unshift([...emptyRow]);
    this.rows.unshift([...HORIZONTAL_BAR_SHAPE]);
  }

  move(direction) {
    if (direction === 'v') {
      this.rows[0] = [...emptyRow];
      this.rows[1] = [...HORIZONTAL_BAR_SHAPE];
    }
  }

  draw() {
    this.rows.forEach(r => console.log(`|${r.map(i => i || '\t').join('')}|`));
    console.log(`+\t-\t-\t-\t-\t-\t-\t+`);
  }
}

const grid = new Grid();
grid.draw();
console.log('');
console.log('');
console.log('');

grid.placeShape();
grid.draw();
console.log('');
console.log('');
console.log('');
grid.move('v');
grid.draw();
