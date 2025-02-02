import { readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';

const root = path.resolve(path.join(path.dirname(new URL(import.meta.url).pathname), '..'));
const packageJson = JSON.parse(await readFile(path.join(root, 'package.json'), 'utf-8'));
const variants = JSON.parse(await readFile(path.join(root, 'variants.json'), 'utf-8'));

const { devDependencies, ...commonPackageJsonProperties } = packageJson;

await Promise.all(
  variants.map(async variant => {
    const libcName = variant.libc === 'glibc' ? 'gnu' : variant.libc;
    const variantName = [variant.os, variant.cpu, libcName]
      .filter(part => part !== undefined)
      .join('-');
    const nativeBindingFile = `exec.${variantName}.node`;
    const variantPackageJson = {
      name: `${packageJson.name}-${variantName}`,
      version: packageJson.version,
      description: packageJson.description,
      author: packageJson.author,
      keywords: packageJson.keywords,
      license: packageJson.license,
      repository: packageJson.repository,
      main: nativeBindingFile,
      files: [nativeBindingFile],
      os: toArray(variant.os),
      cpu: toArray(variant.cpu),
      libc: toArray(variant.libc),
      engines: packageJson.engines
    };

    await writeFile(
      path.join(root, 'npm', variantName, 'package.json'),
      `${JSON.stringify(variantPackageJson, undefined, 2)}\n`,
      'utf8'
    );
  })
);

function toArray(value) {
  if (value === undefined) {
    return undefined;
  }

  return Array.isArray(value) ? value : [value];
}
