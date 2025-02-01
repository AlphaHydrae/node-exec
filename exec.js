const { doNotCloseOnExit, execvp } = require('./index.js');

doNotCloseOnExit(process.stdin.fd);
doNotCloseOnExit(process.stdout.fd);
doNotCloseOnExit(process.stderr.fd);
execvp('ls', [process.argv0, 'tests']);
