const { execSync } = require('node:child_process');

describe('exec', () => {
  test('it should work', () => {
    expect(execSync('node tests/exec.js').toString()).toBe('exec.js\nexec.spec.js\n');
  });
});
