function extractOperationParts(operationStr) {
  const rightHandSide = operationStr.split("= ")[1];

  if (rightHandSide.includes("*")) {
    return [
      ...rightHandSide
        .split("* ")
        .map((p) => p.trim())
        .map((p) => (p === "old" ? p : Number(p))),
      "*",
    ];
  } else {
    return [
      ...rightHandSide
        .split("+ ")
        .map((p) => p.trim())
        .map((p) => (p === "old" ? p : Number(p))),
      "+",
    ];
  }
}

function computeNewWorryLevel(item, operation) {
  let rightHandSide = operation.split("= ")[1];
  let updatedWithValue = rightHandSide.replace(/old/gi, item);
  let result = eval(updatedWithValue);
  return result;
}

function getProductOfInspectedItems(monkeys) {
  return monkeys
    .map((m) => m.itemsInspected)
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
    operationParts,
    testDivisibilityBy,
    ifTrueThrowTo,
    ifFalseThrowTo,
  }) {
    this.identifier = identifier;
    this.startingItems = startingItems;
    this.operation = operation;
    this.operationParts = operationParts;
    this.testDivisibilityBy = testDivisibilityBy;
    this.ifTrueThrowTo = ifTrueThrowTo;
    this.ifFalseThrowTo = ifFalseThrowTo;
    this.itemsInspected = 0;
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

  inspect(worryLevelCalcStrategyFn) {
    const newValues = [];

    while (this.startingItems.length > 0) {
      this.itemsInspected++;
      const item = this.startingItems.shift();

      const newItem = worryLevelCalcStrategyFn(item, this.operation);

      newValues.push([
        newItem,
        newItem % this.testDivisibilityBy === 0
          ? this.ifTrueThrowTo
          : this.ifFalseThrowTo,
      ]);
    }

    return newValues;
  }

  catch(item) {
    this.startingItems.push(item);
  }
}

function parseInput(input) {
  const monkeys = [];
  let monkey;
  let currentIndex = 0;

  input.push("");
  input.forEach((line) => {
    if (line === "") {
      if (monkey) {
        monkeys.push(monkey.clone());
        monkey = null;
      }
    } else if (line.includes("Monkey")) {
      monkey = new Monkey({ identifier: currentIndex });
      currentIndex++;
    } else if (line.includes("Starting items:")) {
      monkey.startingItems = line.split(": ")[1].split(", ").map(Number);
    } else if (line.includes("Operation")) {
      monkey.operation = line.split("Operation: ")[1];
      monkey.operationParts = extractOperationParts(line);
    } else if (line.includes("Test")) {
      monkey.testDivisibilityBy = Number(line.replace(/\D+/, ""));
    } else if (line.includes("If true")) {
      monkey.ifTrueThrowTo = Number(line[line.length - 1]);
    } else if (line.includes("If false")) {
      monkey.ifFalseThrowTo = Number(line[line.length - 1]);
    }
  });

  return monkeys;
}

function runRounds(n, monkeys, strategyFn) {
  return new Array(n)
    .fill(1)
    .map((_, i) => i + 1)
    .map((round) => {
      monkeys.forEach((monkey) => {
        const newValues = monkey.inspect(strategyFn);
        newValues.forEach(([item, targetMonkeyId]) =>
          monkeys[targetMonkeyId].catch(item)
        );
      });

      return {
        round,
        monkeys: monkeys.map((m) => ({
          id: m.identifier,
          inspected: m.itemsInspected,
        })),
      };
    });
}

function partOne(input) {
  const monkeys = parseInput(input);
  const strategyFn = (item, operation) =>
    Math.floor(computeNewWorryLevel(item, operation) / 3);

  runRounds(20, monkeys, strategyFn);

  return getProductOfInspectedItems(monkeys);
}

function partTwo(input) {
  const monkeys = parseInput(input);
  const allMultipliers = monkeys
    .map((monkey) => monkey.testDivisibilityBy)
    .sort((a, b) => a - b);

  const sharedProduct = allMultipliers.reduce((t, i) => (t *= i), 1);
  const strategyFn = (item, operation) =>
    Math.floor(computeNewWorryLevel(item, operation) % sharedProduct);

  runRounds(10000, monkeys, strategyFn);

  return getProductOfInspectedItems(monkeys);
}

module.exports = { partOne, partTwo };
