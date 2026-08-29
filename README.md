# formoniq-studio

The interactive viewer for [formoniq](https://github.com/luiswirth/formoniq),
a finite element exterior calculus engine:
a wgpu/winit/egui application for inspecting meshes, simplicial manifolds, cochains
and the PDE solutions computed on them.

It runs natively and in the browser via WebAssembly and WebGPU,
with the solve running client-side,
so the viewer is reachable without a toolchain at
[lwirth.com/formoniq-studio](https://lwirth.com/formoniq-studio).

## What it shows

What is on screen is a point in a product: a mesh source and a study.
Any study runs on any mesh, the two axes independent,
so a curated example is a named point in that product rather than its own code path.

The engine is intrinsic-first and needs no embedding.
A viewer needs one, because nothing reaches the screen until a point has a position,
and this is the deliberate consumer of that extrinsic carve-out.
It is kept intrinsic as far as it can be:
a curve integrator, for instance, works in the barycentric charts of the atlas
and crosses between cells through their affine transition maps,
committing to an ambient position only at the last step.

Ambient dimension is fixed at 3, the native space of the GPU,
while intrinsic dimension and form grade stay general within it.
Two reductions carry this.
Form grade reduces to a render mark through the reduced grade min(k, n−k):
a scalar density coloring, a glyph or particle line field, a standing-wave displacement height.
That reduction lives upstream in the engine,
so an exporter and this viewer read a field the same way.
Intrinsic dimension reduces to a render primitive min(n, 2):
a surface to wound triangles, a curve to segments, a point set to points,
and a solid to the 2-simplices of its boundary.

`AGENTS.md` documents the design in full.

## Running it

```sh
cargo run --release
```

The shipped meshes live in `assets/meshes` and are tracked in git LFS,
so a clone needs `git lfs pull` before the build can embed them.

The web build is what the deploy workflow runs:

```sh
cargo build --release --lib --target wasm32-unknown-unknown
wasm-bindgen --target web --no-typescript --out-dir dist \
  --out-name formoniq_studio \
  target/wasm32-unknown-unknown/release/formoniq_studio.wasm
cp web/index.html web/solve_worker.js dist/
```

The `wasm-bindgen` CLI must match the `wasm-bindgen` crate version in `Cargo.lock` exactly.

## Developing against a local engine

The engine is a git dependency pinned to one revision, so a clone builds standalone.
To work on both at once, point that source at a checkout with a `[patch]` block
in `.cargo/local.toml` (untracked, and read only when asked for):

```toml
[patch."https://github.com/luiswirth/formoniq"]
formoniq = { path = "../formoniq/crates/formoniq" }
# ... one line per engine crate in `Cargo.toml`
```

```sh
cargo --config .cargo/local.toml run --release
```

Changes that belong to the engine are committed there and the revision bumped here,
never worked around in the viewer.

## License

MIT or Apache-2.0, at your option.
