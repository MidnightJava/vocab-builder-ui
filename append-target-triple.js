import { execa } from 'execa'
import fs from 'fs'

const USAGE = 'node ./append-target-triple file [...file]'

let extension = ''
if (process.platform === 'win32') {
  extension = '.exe'
}

const usage = () => {
  console.log(`Usage: ${USAGE}`)
}

async function main(files) {
  const rustInfo = (await execa('rustc', ['-vV'])).stdout
  const targetTriple = /host: (\S+)/g.exec(rustInfo)[1]
  if (!targetTriple) {
    console.error('Failed to determine platform target triple')
  }
  files.forEach(file => {
    fs.renameSync(
      `src-tauri/binaries/${file}/${file}${extension}`,
      `src-tauri/binaries/${file}/${file}-${targetTriple}${extension}`
    )
  })
}

if (process.argv.length <= 2) {
  usage()
  process.exit()
}
main(process.argv.slice(2)).catch(e => {
  throw e
})
