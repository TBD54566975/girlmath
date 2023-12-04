import { expect } from 'chai';

import { Subunits, loadWasmSync } from '../pkg/girlmath_js.js';

describe('Subunitd', () => {
  before(() => {
    loadWasmSync();
  });

  describe('majorToMinor', () => {
    it('returns a string containing the amount of subunits', () => {
      const result = Subunits.majorToMinor("USD", "$1.00")
      expect(result).to.eq("100");
    });

    it('throws if the amount string is not a currency string', () => {
      expect(() => Subunits.majorToMinor("USD", "abcd")).to.throw();
    });
  });

  describe('minorToMajor', () => {
    it('returns a string containing the amount of major units', () => {
      const result = Subunits.minorToMajor("USD", "1500")
      expect(result).to.eq("15");
    });

    it('throws if the amount string is not a currency string', () => {
      expect(() => Subunits.minorToMajor("USD", "abcd")).to.throw();
    });
  });
});