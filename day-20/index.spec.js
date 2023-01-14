class Mixer {
  constructor(input) {
    this._input = input.map((n, idx) => ({ id: idx, value: n, next: idx + 1 }));

    this._mixingResult = [...this._input];
    this._currentItem = this._mixingResult[0];
  }

  mix() {
    const idx = this._mixingResult.indexOf(this._currentItem);
    if (this._currentItem.value >= 0) {
      for (let i = idx; i < idx + this._currentItem.value; i++) {
        const target = this._mixingResult[i + 1];
        this._mixingResult[i + 1] = this._currentItem;
        this._mixingResult[i] = target;
      }
    } else {
      for (let i = 0; i < idx + Math.abs(this._currentItem.value); i++) {
        const newIdx = (idx - i) % this._mixingResult.length;
        console.log(newIdx);
        const target = this._mixingResult[newIdx];
        this._mixingResult[newIdx] = this._currentItem;
        this._mixingResult[i] = target;
      }
    }

    this._currentItem = this._input.find(i => i.id === this._currentItem.next);
  }

  inspect() {
    return this._mixingResult.map(i => i.value);
  }
}

describe('Mixer', () => {
  describe('mix', () => {
    it('should move positive numbers', () => {
      const input = [1, 2, -3, 3, -2, 0, 4];
      const mixer = new Mixer(input);

      mixer.mix();
      expect(mixer.inspect()).toEqual([2, 1, -3, 3, -2, 0, 4]);

      mixer.mix();
      expect(mixer.inspect()).toEqual([1, -3, 2, 3, -2, 0, 4]);
    });

    it.only('should move positive a negative number', () => {
      const input = [1, 2, -3, 3, -2, 0, 4];
      const mixer = new Mixer(input);

      mixer.mix();
      expect(mixer.inspect()).toEqual([2, 1, -3, 3, -2, 0, 4]);

      mixer.mix();
      expect(mixer.inspect()).toEqual([1, -3, 2, 3, -2, 0, 4]);

      mixer.mix();
      console.log(mixer.inspect());
      expect(mixer.inspect()).toEqual([1, 2, 3, -2, -3, 0, 4]);
    });
  });
});
