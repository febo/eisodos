#!/usr/bin/env zx
import 'zx/globals';
import {
  cliArguments,
  getProgramFolders,
  workingDirectory,
} from '../utils.mjs';

// Save external programs binaries to the output directory.
import './dump.mjs';

// Configure additional arguments here, e.g.:
// ['--arg1', '--arg2', ...cliArguments()]
const buildArgs = cliArguments();

// Build the programs.
for (const folder of getProgramFolders()) {
  cd(`${path.join(workingDirectory, folder)}`);
  await $`cargo-build-sbf ${process.argv.slice(3)}`;
}
