const { isOrdered } = require('./solution');

describe('checkOrder', () => {
  describe('both sides are lists', () => {
    it('should return true if the first left side item is smaller', () => {
      const left = [1, 1, 3, 1, 1];
      const right = [1, 1, 5, 1, 1];

      expect(isOrdered(left, right)).toBe(true);
    });

    it('should return false if the first left side item is greater than the item in the same position in the right side', () => {
      const left = [1, 1, 6, 1, 1];
      const right = [1, 1, 5, 1, 1];

      expect(isOrdered(left, right)).toBe(false);
    });
  });

  describe('nested lists', () => {
    it('should recursively compare list items with list items, using the same rules', () => {
      const left = [[1], 2, 3, 4];
      const right = [[1], 4];

      expect(isOrdered(left, right)).toBe(true);
    });

    it('should compare list items with list items', () => {
      const left = [[2], 2, 3, 4];
      const right = [[1], 2, 5, 5];

      expect(isOrdered(left, right)).toBe(false);
    });
  });

  describe('mixed types', () => {
    it('should parse the right side to a list and run the comparison again', () => {
      const left = [[1], [2, 3, 4]];
      const right = [[1], 4];

      expect(isOrdered(left, right)).toBe(true);
    });

    it('should parse the right side to a list and run the comparison again', () => {
      const left = [[1], [5, 3, 4]];
      const right = [[1], 4];

      expect(isOrdered(left, right)).toBe(false);
    });

    it('should parse the right side to a list and run the comparison again', () => {
      const left = [9];
      const right = [[8, 7, 6]];

      expect(isOrdered(left, right)).toBe(false);
    });
  });

  describe('left side running out of items', () => {
    it('should return true if left runs out of items with analysis status UNKNOWN', () => {
      const left = [];
      const right = [3];

      expect(isOrdered(left, right)).toBe(true);
    });

    it('should return true if left runs out of items with analysis status UNKNOWN', () => {
      const left = [[4, 4], 4, 4];
      const right = [[4, 4], 4, 4, 4];

      expect(isOrdered(left, right)).toBe(true);
    });
  });

  describe('right side running out of items', () => {
    it('should return true if right runs out of items with analysis status UNKNOWN', () => {
      const left = [[[]]];
      const right = [[]];

      expect(isOrdered(left, right)).toBe(false);
    });
  });

  it('should check equality for empty sets', () => {
    const left = [1, [], 4];
    const right = [1, [], 3];
    expect(isOrdered(left, right)).toBe(false);
  });

  it('should return false', () => {
    const left = [1, [2, [3, [4, [5, 6, 7]]]], 8, 9];
    const right = [1, [2, [3, [4, [5, 6, 0]]]], 8, 9];
    expect(isOrdered(left, right)).toBe(false);
  });
});
