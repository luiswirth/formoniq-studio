//! Writing the baked surface as a Wavefront OBJ.
//!
//! A leaf of the bake: an OBJ is a wound triangle surface in $RR^3$, which is
//! exactly what the dimension reduction already produced, so this writes
//! [`BakedMesh`]'s vertex table and fill rather than reducing a complex a
//! second time.
//!
//! Reading one is the other direction and lives in `regge::io::obj`: a file
//! read off disk becomes a mesh, which is the engine's object, where a file
//! written here consumes a bake, which is the viewer's.

use std::fmt::Write as _;

use crate::realize::bake::BakedMesh;

/// The baked surface as an OBJ document: its vertex table as `v` lines and its
/// fill's wound triangles as `f` lines.
///
/// The winding is the bake's, hence the manifold's own coherent orientation
/// where it has one, so a reader downstream sees the same outward faces the
/// viewer draws. A bake with no fill (a curve, a point cloud, a closed solid)
/// writes its vertices and no faces, which is what the format has to say about
/// an object with no surface.
pub fn to_string(baked: &BakedMesh) -> String {
  let mut obj = String::new();
  for vertex in &baked.positions {
    let [x, y, z] = vertex.position;
    writeln!(obj, "v {x:.6} {y:.6} {z:.6}").unwrap();
  }
  for triangle in baked.fill_triangles() {
    // OBJ indexes vertices from one.
    writeln!(
      obj,
      "f {} {} {}",
      triangle[0] + 1,
      triangle[1] + 1,
      triangle[2] + 1
    )
    .unwrap();
  }
  obj
}

/// Writes the baked surface to `path` as OBJ. Native only: the browser has no
/// filesystem to write to.
#[cfg(not(target_arch = "wasm32"))]
pub fn write(path: impl AsRef<std::path::Path>, baked: &BakedMesh) -> std::io::Result<()> {
  std::fs::write(path, to_string(baked))
}
