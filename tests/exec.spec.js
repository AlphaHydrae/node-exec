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

  test('the execvp error code is in the error message if the call fails', async () => {
    await expect(() => execAsync('node tests/exec.js foo')).rejects.toThrow(
      'execvp failed with code ENOENT: No such file or directory'
    );
  });
});
