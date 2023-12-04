import { expect } from 'chai';

import { Exchange, loadWasmSync } from '../pkg/girlmath_js.js';

describe('Exchange', () => {
  before(() => {
    loadWasmSync();
  });

  describe('convertCurrencyAsInt', () => {
    it('converts the currency amount represented as a number', () => {
      const result = Exchange.convertCurrencyAsInt("USD", "EUR", 100, 1.5);
      expect(result).to.eq(150);
    });

    it('truncates float values of payin amount to int', () => {
      // TODO(diehuxx): Maybe subunit amount should be a float on the rust side
      const result = Exchange.convertCurrencyAsInt("USD", "EUR", 100.9, 1.5)
      expect(result).to.eq(150);
    });
  });

  describe('convertCurrencyAsStr', () => {
    it('converts the currency amount represented as a string', () => {
      const result = Exchange.convertCurrencyAsStr("USD", "EUR", "100", 1.5);
      expect(result).to.eq("€150,00");
    });

    it('Does not truncate payin amount to integer value', () => {
      const result = Exchange.convertCurrencyAsStr("USD", "EUR", "100.9", 1.5)
      expect(result).to.eq("€151,35");
    });
  });
});