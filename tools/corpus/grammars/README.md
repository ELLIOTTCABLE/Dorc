# Vendored tree-sitter grammars

These prebuilt WebAssembly grammars are committed (not fetched at install) so the
scanner runs on any platform with just Node — no native toolchain, no `node-gyp`,
no install-time ABI surprises (which is exactly what bit us; see below).

## `tree-sitter-bash.wasm`
- Source: the `tree-sitter-bash` npm package, **v0.25.1** (its own bundled prebuilt wasm).
- sha256: `8292919c88a0f7d3fb31d0cd0253ca5a9531bc1ede82b0537f2c63dd8abe6a7a`
- size: 1358224 bytes
- Loaded by `src/scan.ts` via `Language.load(bytes)` with `web-tree-sitter@0.25.10`.

Re-acquire: `npm i tree-sitter-bash@0.25.1 --ignore-scripts` (skip the native build),
then copy `node_modules/tree-sitter-bash/tree-sitter-bash.wasm` here.

### Why vendored, and why not `tree-sitter-wasms`
`tree-sitter-wasms@0.1.13`'s bash grammar is ABI-incompatible with every published
web-tree-sitter: it *loads* under 0.25.x but `parse()` throws `resolved is not a
function` (an unresolved wasm dylink stub) the moment real shell exercises the
affected grammar rule; 0.24/0.26 fail even to load. The grammar shipped inside
`tree-sitter-bash` itself links correctly against web-tree-sitter, so we use that.
Native `tree-sitter` bindings were not an option here — no Windows prebuild for
node-22 and no MSVC to compile.
