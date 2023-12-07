import { fileURLToPath } from 'node:url'

import packageJson from '../pkg/package.json' assert { type: 'json' }
import esbuild from 'esbuild'
import path from 'node:path'
import fs from 'node:fs'

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const bundleResult = esbuild.buildSync({
  format: 'esm',
  bundle: true,
  entryPoints: [`${__dirname}/../pkg/girlmath_js.js`],
  outdir: 'dist',
  allowOverwrite: true,
  write: false
})

const [ bundleSrc ] = bundleResult.outputFiles
fs.writeFileSync(`${__dirname}/../dist/bundle.js`, bundleSrc.text, { encoding: 'utf8' })

delete packageJson['main']
packageJson['type'] = 'module'
packageJson['files'] = ['bundle.js', 'index.js', packageJson['types']]
packageJson['exports'] = {
  '.': {
    'import': './index.js',
    'types': `./${packageJson['types']}`
  }
}

fs.writeFileSync(`${__dirname}/../dist/package.json`, JSON.stringify(packageJson, null, 2), { encoding: 'utf8' })
fs.copyFileSync(`${__dirname}/../pkg/girlmath_js.d.ts`, `${__dirname}/../dist/girlmath_js.d.ts`)