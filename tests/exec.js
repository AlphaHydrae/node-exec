const { execvp } = require('../exec.js');

if (process.argv.length < 3) {
  throw new Error('Missing command');
}

const [_node, _file, command, ...args] = process.argv;

execvp(command, args);
