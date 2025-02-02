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

[![build](https://github.com/AlphaHydrae/node-exec/actions/workflows/build.yml/badge.svg)](https://github.com/AlphaHydrae/node-exec/actions/workflows/build.yml)
[![MIT License](https://img.shields.io/static/v1?label=license&message=MIT&color=informational)](https://opensource.org/licenses/MIT)

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->

- [Installation](#installation)
- [Support matrix](#support-matrix)
- [Credits](#credits)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

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

## Credits

This package is a re-implementation of
https://github.com/jprichardson/node-kexec in Rust, also inspired by the
following conversations:

- [A way to call execl, execle, execlp, execv, execvP or execvp from Node.js](https://stackoverflow.com/a/77774287/249893)
- [execve in node](https://groups.google.com/g/nodejs/c/4vtWG1KCQC4)

Also a big thank you to the following Rust projects for making it easy:

- [NAPI-RS](https://napi.rs) (build pre-compiled Node.js addons in Rust)
- [nix](https://docs.rs/nix) (Rust-friendly bindings to the various *nix system functions)
