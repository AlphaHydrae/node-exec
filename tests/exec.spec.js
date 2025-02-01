const { execSync } = require('node:child_process');

describe('exec', () => {
  test('it should work', () => {
    expect(execSync('node exec.js').toString()).toBe('exec.spec.js\n');
  });
});
