const { resolveExpressionFor } = require('.');

describe('resolveExpressionFor', () => {
  it('should return the resolved value of an expression if that is the case', () => {
    const monkey = { monkeyName: 'm1', yell: '3' };
    const yells = [monkey];

    const result = resolveExpressionFor(monkey, yells);

    expect(result).toEqual(3);
  });

  it('should resolve each expression part if it is a complex unresolved expression', () => {
    const monkey1 = { monkeyName: 'm1', yell: 'm2 + m3' };
    const monkey2 = { monkeyName: 'm2', yell: '3' };
    const monkey3 = { monkeyName: 'm3', yell: '4' };
    const yells = [monkey1, monkey2, monkey3];

    const result = resolveExpressionFor(monkey1, yells);

    expect(result).toEqual(7);
  });
});
