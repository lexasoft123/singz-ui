/*
 * "Is the committed dist/ actually built from the committed src/?"
 *
 * dist/ is checked in on purpose: consumers install this package straight
 * from git, and a `prepare` build would make every consumer install the
 * kit's whole toolchain and turn a kit build failure into a confusing
 * failure in THEIR CI. The cost of that choice is that dist/ can go stale,
 * so this guards it.
 */
import { execFileSync } from 'node:child_process'

execFileSync('npm', ['run', '--silent', 'build'], { stdio: 'inherit' })

const dirty = execFileSync('git', ['status', '--porcelain', '--', 'dist'], { encoding: 'utf8' }).trim()
if (dirty) {
  console.error('dist/ is stale — rebuild and commit it:\n' + dirty)
  process.exit(1)
}
console.log('dist/ is current')
