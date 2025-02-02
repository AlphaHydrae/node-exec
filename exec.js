const native = require('./index.js');

module.exports.execvp = function (file, args) {
  native.doNotCloseOnExit(process.stdin.fd);
  native.doNotCloseOnExit(process.stdout.fd);
  native.doNotCloseOnExit(process.stderr.fd);
  native.execvp(file, [process.argv0, ...args]);
};
