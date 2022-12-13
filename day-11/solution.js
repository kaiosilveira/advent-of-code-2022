function parseInput(input) {
  const monkeys = [];
  let monkey;
  let currentIndex = 0;

  input.push('');
  input.forEach(line => {
    if (line === '') {
      if (monkey) {
        monkeys.push(monkey.clone());
        monkey = null;
      }
    } else if (line.includes('Monkey')) {
      monkey = new Monkey({ identifier: currentIndex });
      currentIndex++;
    } else if (line.includes('Starting items:')) {
      monkey.startingItems = line.split(': ')[1].split(', ').map(Number);
    } else if (line.includes('Operation')) {
      monkey.operation = line.split('Operation: ')[1];
    } else if (line.includes('Test')) {
      monkey.testDivisibilityBy = Number(line.replace(/\D+/, ''));
    } else if (line.includes('If true')) {
      monkey.ifTrueThrowTo = Number(line[line.length - 1]);
    } else if (line.includes('If false')) {
      monkey.ifFalseThrowTo = Number(line[line.length - 1]);
    }
  });

  return monkeys;
}

function getProductOfInspectedItems(monkeys) {
  return monkeys
    .map(m => m.itemsInspected)
    .sort((a, b) => a - b)
    .reverse()
    .slice(0, 2)
    .reduce((t, p) => t * p, 1);
}

class Monkey {
  constructor({
    identifier,
    startingItems,
    operation,
    testDivisibilityBy,
    ifTrueThrowTo,
    ifFalseThrowTo,
  }) {
    this.identifier = identifier;
    this.startingItems = startingItems;
    this.operation = operation;
    this.testDivisibilityBy = testDivisibilityBy;
    this.ifTrueThrowTo = ifTrueThrowTo;
    this.ifFalseThrowTo = ifFalseThrowTo;
    this.itemsInspected = 0;

    this.clone = this.clone.bind(this);
    this.inspect = this.inspect.bind(this);
    this.throw = this.throw.bind(this);
    this.catch = this.catch.bind(this);
    this.computeNewWorryLevelFor = this.computeNewWorryLevelFor.bind(this);
  }

  clone() {
    return new Monkey({
      identifier: this.identifier,
      startingItems: this.startingItems,
      operation: this.operation,
      testDivisibilityBy: this.testDivisibilityBy,
      ifTrueThrowTo: this.ifTrueThrowTo,
      ifFalseThrowTo: this.ifFalseThrowTo,
    });
  }

  inspect(worryLevelCalcStrategyFn, monkeys) {
    const newValues = [];

    while (this.startingItems.length > 0) {
      this.itemsInspected++;
      const item = this.startingItems.shift();

      const newItem = worryLevelCalcStrategyFn(this.computeNewWorryLevelFor(item), this.operation);

      newValues.push([
        newItem,
        newItem % this.testDivisibilityBy === 0 ? this.ifTrueThrowTo : this.ifFalseThrowTo,
      ]);
    }

    newValues.forEach(([item, toMonkeyId]) => this.throw(item, monkeys[toMonkeyId]));
  }

  throw(item, toMonkey) {
    toMonkey.catch(item);
  }

  catch(item) {
    this.startingItems.push(item);
  }

  computeNewWorryLevelFor(item) {
    let rightHandSide = this.operation.split('= ')[1];
    let updatedWithValue = rightHandSide.replace(/old/gi, item);
    let result = eval(updatedWithValue);

    return result;
  }
}

function runRounds(n, monkeys, strategyFn) {
  return new Array(n)
    .fill(1)
    .map((_, i) => i + 1)
    .map(round => {
      monkeys.forEach(monkey => monkey.inspect(strategyFn, monkeys));
      return {
        round,
        monkeys: monkeys.map(m => ({ id: m.identifier, inspected: m.itemsInspected })),
      };
    });
}

function partOne(input) {
  const monkeys = parseInput(input);

  const strategyFn = item => Math.floor(item / 3);
  runRounds(20, monkeys, strategyFn);

  return getProductOfInspectedItems(monkeys);
}

function partTwo(input) {
  const monkeys = parseInput(input);
  const allMultipliers = monkeys.map(monkey => monkey.testDivisibilityBy).sort((a, b) => a - b);
  const sharedProduct = allMultipliers.reduce((t, i) => (t *= i), 1);

  const normalizeWorryLevelFor = item => Math.floor(item % sharedProduct);
  runRounds(10000, monkeys, normalizeWorryLevelFor);

  return getProductOfInspectedItems(monkeys);
}

module.exports = { partOne, partTwo };
