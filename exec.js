const native = require('./index.js');

/**
 * Performs the execvp system call with the given file and arguments, replacing
 * the current process image with the new one.
 *
 * This function does not return if successful, as the current process image is
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
 *                        argument is automatically prepended to the arguments
 *                        and passed as the first argument to the `execvp`
 *                        system call.
 * @param {Object} [options] Optional options.
 * @param {string} [options.arg0=file] The value to pass as the first argument
 *                                     to the `execvp` system call. Defaults to
 *                                     the `file` argument.
 * @throws {Error} If the execvp system call fails.
 */
module.exports.execvp = function (file, args, options = {}) {
  const arg0 = options.arg0 ?? file;

  // Prevent the standard streams from being closed when the process is
  // replaced.
  native.doNotCloseOnExit(process.stdin.fd);
  native.doNotCloseOnExit(process.stdout.fd);
  native.doNotCloseOnExit(process.stderr.fd);

  native.execvp(file, [arg0, ...args]);
};
