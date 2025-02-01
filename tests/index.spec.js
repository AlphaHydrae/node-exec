const { execSync } = require('node:child_process')

describe('exec', () => {
  test('it should work', () => {
    expect(execSync('node index2.js').toString()).toBe('index.spec.js\n')
  })
})
