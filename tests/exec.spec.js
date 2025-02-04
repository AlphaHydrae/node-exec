const { exec } = require('node:child_process');
const { promisify } = require('node:util');

const execAsync = promisify(exec);

describe('exec', () => {
  test('it should work', async () => {
    await expect(execAsync('node tests/exec.js ls tests')).resolves.toEqual({
      stderr: '',
      stdout: 'exec.js\nexec.spec.js\n'
    });
  });
});
