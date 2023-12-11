import girlmath from '../dist/index.js'
import { expect } from 'chai'

describe('girlmath', () => {
  describe('majorToMinor', () => {
    it('works', () => {
      const cents = girlmath.majorToMinor('10', 'USD')
      expect(cents).to.equal('1000')
    })
  })
})