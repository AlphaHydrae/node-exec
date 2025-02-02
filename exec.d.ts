/**
 * Performs the execvp system call with the given file and arguments, replacing
 * the current process image with the new one.
 *
 *     execvp('ls', ['-l', '.']);
 *
 * This function does not return if successful, as the current process is
 * replaced by the new one. An error may be thrown, with an error message
 * containing the error code returned by execvp.
 *
 * @param {string} file The file to execute. If not a path, the PATH environment
 *                      variable is searched.
 * @param {string[]} args The arguments to pass to the new process.
 *                        `process.argv0` is automatically prepended to the
 *                        arguments.
 * @throws {Error} If the execvp system call fails.
 */
export declare function execvp(file: string, args: string[]): void;
