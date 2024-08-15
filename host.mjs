import { transpile } from '@bytecodealliance/jco'
import { readFile, writeFile } from 'node:fs/promises';

const base = import.meta.url
const component = await readFile(new URL("./foo.wasm", base))
const transpiled = await transpile(component, {
    name: "foo",
    typescript: false,
})
await writeFile(new URL("./foo.core.wasm", base), transpiled.files["foo.core.wasm"])
await writeFile(new URL("./foo.core2.wasm", base), transpiled.files["foo.core2.wasm"])
await writeFile(new URL("./foo.mjs", base), transpiled.files["foo.js"])
const instance = await import(new URL("./foo.mjs", base))
instance.test()
