# `@alphahydrae/exec`

The [`execvp` function](https://linux.die.net/man/3/execvp) for Node.js.

```js
import { execvp } from '@alphahydrae/exec';

execvp('ls', ['.']);

console.log(`
  This will never print because the executing Node.js
  program is replaced by the executed command, keeping
  the same process ID and file descriptors.
`);
```

If you're familiar with Bash's `exec` function, this is the same for Node.js.

This package was developed to be used in Node.js scripts that are frontends to
execute other commands. For example, a script that would build and execute a
complex SSH or Ansible command.

## Installation

```bash
npm i @alphahydrae/exec
```

## Support matrix

| OS & Architecture | Node 18 | Node 20 | Node 22 |
| ----------------- | ------- | ------- | ------- |
| macOS x64         | ✓       | ✓       | ✓       |
| macOS arm64       | ✓       | ✓       | ✓       |
| Linux x64 gnu     | ✓       | ✓       | ✓       |
| Linux x64 musl    | ✓       | ✓       | ✓       |
| Linux arm64 gnu   | ✓       | ✓       | ✓       |
| Linux arm64 musl  | ✓       | ✓       | ✓       |

> The `exec` family of functions is part of the
> [POSIX](https://en.wikipedia.org/wiki/POSIX) operating system API, so it will
> not work on Windows.
