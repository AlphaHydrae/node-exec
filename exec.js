const native = require('./index.js');

/**
 * Performs the execvp system call with the given file and arguments, replacing
 * the current process image with the new one.
 *
 * This function does not return if successful, as the current process is
 * replaced by the new one. An error may be thrown, with an error message
 * containing the error code returned by execvp.
 *
 * @example
 * import { execvp } from '@alphahydrae/exec';
 * execvp('ls', ['-l', '.']);
 *
 * @param {string} file The file to execute. If not a path, the PATH environment
 *                      variable is searched.
 * @param {string[]} args The arguments to pass to the new process. The `file`
 *                        argument is automatically prepended to the arguments.
 * @throws {Error} If the execvp system call fails.
 */
module.exports.execvp = function (file, args) {
  native.doNotCloseOnExit(process.stdin.fd);
  native.doNotCloseOnExit(process.stdout.fd);
  native.doNotCloseOnExit(process.stderr.fd);
  native.execvp(file, [file, ...args]);
};
