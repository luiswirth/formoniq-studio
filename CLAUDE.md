# formoniq-studio

`formoniq-studio` is the visual, interactive counterpart to `formoniq`:
a viewer for inspecting PDE solutions, meshes and simplicial manifolds, cochains,
and the differential geometry underneath them.
It is meant to be both an instrument for a mathematician or engineer
and a way to see the abstractions directly.

## The engine is upstream, and it governs

[`formoniq`](https://github.com/luiswirth/formoniq) is a separate repository,
depended on as one git source pinned to one revision.
Its `CLAUDE.md` still governs here:
its invariants, conventions and house style bind unchanged,
and a concept expressible without a renderer belongs there rather than here.
The direction is strict, as it is inside the engine's own ladder:
this repository depends on the engine and nothing there depends on it,
which is what keeps the graphics stack out of the engine's build entirely.

The engine's `crates/realize` is the crate directly below this one,
and its `CLAUDE.md` carries the extrinsic side's design:
the one inversion of the parent's intrinsic-first discipline,
the bake (the seam out) and the two reductions, of grade to a mark and of dimension to a primitive.
Read it before changing anything about what a field looks like.
This file carries what is particular to the *viewer*.

## The seam in

The embedding is not assumed diffusely throughout the viewer.
It enters at the bake, which is `realize`'s,
and intrinsic structure is carried as far toward the screen as it can go before that commits.

**`Scene` is the seam in:**
it carries the engine's own types (`Complex`, `MeshCoords`, `Cochain`)
rather than a lossy export format,
so the coloring, the displacement and the choice of render mark
stay decisions of the viewer, made on the real object.

Between it and the bake the discipline is lived, not hoped for:
anything new belongs on that same spine,
intrinsic until the bake, extrinsic only after it.
An exporter and this viewer are peers over the same reduction,
which is what makes an external tool and the viewer agree about what a field looks like;
a reduction that only the viewer can reach has been put in the wrong repository.

## One renderer across dimension and grade

Ambient dimension is $3$, `realize`'s deliberate constant and the native space of the GPU.
Intrinsic dimension and form grade stay agnostic on the range it allows.
A point set, a curve and a surface are one pipeline across grades, not three renderers:
a curve renderer split off from a surface renderer
would be the `if dim == 3` of the engine, reappearing here.

Each case distinction is confined to its own reduction, both of which are `realize`'s,
never smeared through the renderer,
which sees only which *items* a frame has, never why.
The consequence is that one segment pipeline serves the wireframe overlay,
a line field's traced ribbons and a 1-manifold's own cells:
they were one technique described three times,
and what differed between them (ink, width, taper, whether the mark rides the wave) is material data.
What the current ambient does not yet reach (a reduced grade $>= 2$, a point cloud's mark)
is where these extend, not a branch to route around.

The freedom the viewer has over the engine is the *embedding* and the ambient geometry
read off it (normals, ambient distances, global position), and nothing else.
It is not a license about the metric:
a metric is not an extrinsic object, the core uses it freely,
and every metric here is the one the embedding already induces.

## The layers of the viewer

The two seams say where the embedding enters.
These say who may know what, and they bind the same way the parent's invariants do:

- **The model is GPU-free:**
  The gallery and the scene are the mathematics and the state of the viewer.
  Neither names a device, a buffer or a pipeline.
  What is shown is decided there and baked afterward, never the other way round.
- **The display is the callers' shared reduction:**
  What turns a scene and a selection into a draw list
  (the bake, the materials, the framing, the object-intrinsic fractions the marks are scaled by)
  belongs to neither caller.
  A window and a headless export build it identically
  and differ only in where the frame's time comes from.
  A material constructed at a caller instead
  is how the two silently come to disagree about what a field looks like.
  The corollary is the CLI's:
  nothing the code can decide from the object or the context is asked of the user,
  because a knob with one right answer only lets the answer be wrong.
- **What is asked divides by the object it reads:**
  Two objects are on screen (the mesh, and the field read on it)
  and `MeshDisplay`/`FieldDisplay` is already that split,
  so the settings mirror the seam rather than laying a second taxonomy over it.
  The mesh's are always live: a scene without geometry is not a scene.
  The field's are its reduced grade's answer, asked where that rule already lives,
  so a knob appears exactly when it has something to do.
  Neither costs a branch below the model:
  a setting naming an item drops it from the draw list,
  and one naming a deformation the items ride is a material at zero,
  the shape bloom's "off" already has.

  What builds the object and what draws it are the two sidebars,
  and they keep the two questions apart:
  the **browser** picks the point in `MeshSource × Study` (which mesh, which computation)
  and the **inspector** edits the parameters of the study picked there
  and the display of the two objects it produced.
  `Study`'s variant parameters (the eigenmode grade and count, a trajectory's sampling)
  are the inspector's, not the browser's,
  because they are knobs *on* the chosen study rather than the choice of it.
  An edit that drives a re-solve commits on release, not mid-drag,
  so the background solve fires once.
  What belongs to **neither** object,
  reading and writing files,
  and the view shell itself
  (which sidebars show, the projection, the light ladder, re-framing the camera),
  is a **menu bar**, the conventional home a reader reaches for these by reflex,
  and the one place a command that is not a property of the mesh or the field is allowed to live.
- **The renderer sees baked geometry and explicit time, and nothing else:**
  No FEEC types, no clock, no window, no surface.
  Time is an argument,
  so the interactive loop passes wall-clock seconds
  and an exporter passes the instant it means to render.
  The frames are deterministic either way,
  and the two cannot drift because there is one frame graph.
  Which instants those are is the exporter's business and not the renderer's:
  an oscillating field has a period,
  and a clip that samples it as $t_k = k T \/ N$ closes on itself exactly,
  where one pinned to the playback grid would not.
  A frame is a *draw list* (batches with their materials, in submission order),
  so the number of things on screen is the caller's,
  never a fixed set the renderer declares.

  **A simulation is stepped to an instant, not evaluated at one**,
  and that is the one honest extension of "time is an argument".
  A standing wave is a function of $t$, so any frame can be asked for directly.
  A mark that carries state (an advected population) has no such closed form,
  and a caller can only say how far to advance it.
  So the count of steps is an argument beside the seconds,
  and what keeps the two callers from drifting
  is no longer the stateless graph but a *deterministic* one:
  the state's own randomness must be a pure function of the thing and its generation,
  never of a clock,
  so that a given count means the same picture to a window and to an exporter.
  A mark that cannot promise that does not belong in the frame graph.

- **Radiance is the scene's, the display's range is the target's, and one pass crosses between them:**
  The scene target is float and unbounded because additive marks accumulate:
  clipping at the blend destroys the very quantity the mark is made of.
  Everything that must happen in radiance (filtering, spilling light) happens before the crossing.
  The tone map *is* the crossing, and it is last.
  This is a real ordering, not a preference:
  anything that maps the range earlier
  leaves the passes after it nothing above 1 to find,
  and they fail silently rather than loudly.

  What the crossing costs is unavoidable and worth stating plainly:
  $[0, 1]$ is already fully spent by the marks that live there,
  so headroom above 1 must take range from below it, and every mark shifts.
  There is no setting that avoids this:
  only the choice of whether the dynamic range or the palette matters more
  for what is being looked at,
  which is exactly the kind of question the code cannot settle from the object,
  and therefore one of the few the viewer is asked.
- **State lives on the manifold, the screen is presentation only:**
  Screen-space passes are not suspect in themselves,
  bloom, supersampling and the tone map are screen-space and correctly so,
  because they model the observation (the lens, the eye), never the field.
  The line is *persistence*:
  anything that survives a frame is a claim about the object,
  and the object lives on the manifold, where the camera cannot touch it.
  A pass that reads only this frame's image may be screen-space.
  A texture that accumulates across frames must be manifold data,
  indexed by chart and barycentric coordinate.
  The deposit atlas is the constructive example,
  and a screen-space trail or history buffer is the violation:
  it would bake the camera into the state,
  and every orbit or export would smear a history that was never the field's.
  The test is the same cut the radiance/display split makes, extended along time.
- **The UI is a pure function of the model,** returning requested changes, not a mutator of it.
- **Layout answers to the window, never to the platform:**
  What a narrow viewport changes
  is whether a sidebar is *docked beside* the scene or *laid over* it,
  never what the panels contain, and never which panel a control belongs to,
  because that taxonomy mirrors the two objects on screen
  and a second one keyed on screen size would cut across it.
  Below the width where both sidebars plus a usable viewport fit, nothing docks by default:
  the scene is what a reader sees first and a sidebar is something they open.
  A narrow desktop window gets exactly what a phone gets:
  there is no mobile build, only a narrow one.

  **The sidebars collapse at every width, and the layout only supplies the default.**
  Wanting the controls out of the way to look at the scene
  is not something only a small screen wants,
  so the toggles are always there.
  What the width decides is what an *untouched* sidebar does.
  A reader's explicit choice outranks it and survives a resize.
  That is a third state, not a boolean:
  the default has to stay derivable,
  or the first frame on a phone shows two panels meeting in the middle
  before anything can correct them.

## The platform is a product, presets are points in it

What the viewer shows is a point in `MeshSource × Study`:
any study on any mesh, the two axes independent and every pair total.
The cache, the background load and the placeholder machinery all key on the pair,
not on a fixed enumeration of views.
A `Preset` is a named point in that product together with the field it opens on:
selecting one sets the two axes and the selection,
and everything afterward is the ordinary platform.

**The shipped meshes are the asset directory, not a list of them.**
`build.rs` enumerates `assets/meshes` and generates the table the picker and the CLI read,
so a mesh is added by dropping the file in:
its extension picks the reader, its stem is its name.
Generated at build time rather than scanned at run time
because the assets are embedded in the binary,
which is what lets the web build (with no filesystem to scan) ship the same set.
A hand-written list of what is in a directory
is a second source of truth for something the directory already knows, and the two drift.

A preset is therefore a *configuration*, never a code path:
the moment a curated example would need its own branch to build or display,
it has stopped being a preset and the generalization has a hole.
This is the same dissolution the parent's invariants demand, one level up:
the reference cell is the mesh whose only cell is the unit simplex,
so the local shape functions are the Whitney study on it, not a study of their own.
The global shape functions are that study on the triforce.
The spherical harmonics are the eigenmode study on the sphere.
Anything that looks like a special view owes the same reduction into a mesh and a study.

## Rendering

Render the way an expert in computer graphics would:
prefer the visually most pleasing approach, and treat quality as a requirement rather than a finish.
The mathematics decides *what* is drawn
(the reduced grade picks the mark, the eigenvalue drives the standing wave)
and the graphics craft decides *how well*.

Three durable conventions, kept general on purpose:

- **A mark drawn on a surface is biased in depth, never displaced in space:**
  A glyph in its cell and a wireframe edge along its simplex
  are coplanar with the fill and must win the depth comparison.
  That is a claim about $z$ alone,
  and the rasterizer's depth bias is what makes it,
  after the mark's screen position is already fixed.
  Translating the mark toward the camera instead is the plausible wrong answer:
  it puts the mark at a depth its surface does not have,
  so the two show parallax and the mark slides across its own face as the camera orbits,
  and the offset is measured in the mark's size rather than the gap to what is in front,
  so on a *closed* surface a far face's marks pass through the near one.
  One open sheet hides both faults
  (there is nothing to slide against and nothing in front to pierce),
  so this is a fault that only a solid's boundary reveals,
  and it is why the convention is written down rather than rediscovered.
- **Shaders are checked by the test suite**, not only at pipeline creation,
  so a broken shader fails `cargo test` rather than the running viewer.
  The check runs naga's frontend (the one the *native* build uses),
  so it catches parse and validation errors
  but *not* what a browser's own WGSL→backend compiler rejects.
  The web target is WebKit/Metal, and it is stricter and buggier than naga and Tint:
  a shader that validates and runs on Chrome can still fail there
  ("Vertex library failed creation" is its generic symptom).
  The concrete rule this cost us,
  **no pipeline-overridable `override` constants specialized through the pipeline `constants` map**:
  WebKit fails to specialize them.
  Bake such a value into the WGSL as a `const` from the Rust side instead
  (see `render::ssaa_prelude`).
  WebKit is the strict oracle.
  When a shader change is non-trivial,
  it is the browser, not `cargo test`, that has the last word.
- **The graphics stack is pinned as a unit:**
  The types crossing the boundary between the UI layer and the renderer
  must come from the *same* underlying GPU crate, not merely semver-compatible versions.
  Bump them together, and let the build enforce it.

## Two platforms, one viewer

The viewer runs native and on the web (`wasm32`, WebGPU), from the same code.
The split is a discipline, not a fork.
**Everything browser-specific is confined to `web.rs`**:
the `wasm-bindgen` entry point, mounting winit's canvas into the document,
and bridging the *async* GPU bootstrap back into the event loop
(device and surface creation cannot block the browser,
so the finished `State` is parked in a slot the loop drains).
Nothing web-flavored is allowed to leak into the shared viewer code.
What remains there are thin `#[cfg]` gates at genuine platform seams,
never web logic interleaved with native.

The web is the constrained side, and the constraints are honest, not worked around:

- **No filesystem, no subprocess:**
  OBJ loading, PNG/MP4 export and the CLI are native features,
  gated off the web build, which has nowhere to read or write.
  A feature that needs local files is native by nature, not a web regression to fix.
- **Single-threaded within a context, so the solve moves to another one:**
  The plain `wasm32` target has no background thread and `faer` builds without its `rayon` pool,
  so a study cannot be solved off the main thread the way native does it.
  It is sent to a *worker* instead:
  the build is a request and an outcome (`solve`), not a closure,
  because a closure cannot cross a `postMessage` boundary and a value can.
  Message passing, not shared memory:
  `SharedArrayBuffer` threads would need cross-origin isolation and a nightly toolchain,
  which a static host cannot give and this does not need.
  The worker loads the same module and calls the same solver,
  so there is one implementation and the boundary is only transport.

  The request carries the mesh itself rather than a descriptor of it.
  A descriptor would cover every mesh the gallery can regenerate and miss the one that matters:
  a mesh the reader loaded,
  which exists nowhere else and is the one whose size nobody has bounded.
- **WebGPU only, by choice:**
  No WebGL2 fallback,
  the viewer targets the modern backend and fails legibly where it is absent,
  rather than constraining the render features to the WebGL2 subset.

The native build is untouched by any of this:
the per-target dependency and feature splits
(the `faer` thread pool, the clipboard backend, the `getrandom` web entropy source)
restore the exact native set,
so a native change never pays for the web target's existence.

## Workflow

Every commit passes all five.
They are the bar, not a suggestion:

```sh
cargo fmt --all
cargo clippy --all-targets --all-features
cargo test --all-features
cargo doc --no-deps
cargo clippy --lib --target wasm32-unknown-unknown
```

The web check is the fifth because the platform split is code:
a native-only change can still break the target that has no filesystem and no threads.
CI runs the same five on every push and pull request.

Commit messages: `scope: imperative summary`, the scope being the module,
e.g. `render: bias marks in depth rather than in space`.

A change to the design is not finished until this file reflects it, in the same commit.
Where this file and the code disagree, one of them is a bug,
and it is usually worth asking which.

## Anti-goals

- No renderer specialized to a fixed intrinsic dimension or grade
  where `realize`'s two reductions cover it.
  Marks chosen by the grade's reduction, primitives by the dimension's.
  No second pipeline for what is one technique at a different ink.
- No web-specific logic outside `web.rs`.
  The shared viewer stays platform-neutral.
  A `wasm`/native divergence is a thin `#[cfg]` at a real seam,
  never a browser concern smeared through the render or model code.
- No dimension dispatch outside the bake, and no grade dispatch outside the mark.
  A `match` on either anywhere else is the case distinction escaping its reduction.
- No reduction of a field living here.
  If an exporter could not reach it, it belongs in `realize`, upstream.
- No claiming metric use as the extrinsic divergence:
  the divergence is the embedding and the ambient space,
  and saying otherwise misreads the engine's invariant 5.
- Nothing transient here, exactly as in the engine:
  no current state, no in-flight passes, no version pins written out,
  no roadmap phrased as a promise.
