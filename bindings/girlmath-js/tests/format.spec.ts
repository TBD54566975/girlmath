import { expect } from 'chai';

import { Format, loadWasmSync } from '../pkg/girlmath_js.js';

describe('Format', () => {
  before(() => {
    loadWasmSync();
  });

  describe('formatCurrencyAmount', () => {
    it('formats the number amount of subunits into a string with currency symbol and separators', () => {
      const result = Format.formatCurrencyAmount("USD", 150)
      expect(result).to.eq("$1.50");
    });

    it('truncates float values of payin amount to int', () => {
      const result = Format.formatCurrencyAmount("USD", 150.1)
      expect(result).to.eq("$1.50");
    });
  });

  describe('parseCurrencyStr', () => {
    it('parses the currency string and returns the integer amount of subunits', () => {
      const result = Format.parseCurrencyStr("USD", "$1.50")
      expect(result).to.eq(150);
    });

    it('throws if the amount string is not a currency amount', () => {
      expect(() => Format.parseCurrencyStr("USD", "abcd")).to.throw();
    });
  });
});