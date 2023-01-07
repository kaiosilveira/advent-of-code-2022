function resolveExpressionFor(monkey, yells) {
  const parsingAttempt = parseInt(monkey.yell, 10);
  if (!isNaN(parsingAttempt)) return parsingAttempt;

  const [firstPart, operator, secondPart] = monkey.yell.split(' ');
  return eval(
    `${resolveExpressionFor(
      yells.find(m => m.monkeyName === firstPart),
      yells
    )} ${operator} ${resolveExpressionFor(
      yells.find(m => m.monkeyName === secondPart),
      yells
    )}`
  );
}

module.exports = { resolveExpressionFor };
