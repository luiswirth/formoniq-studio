//! Where the viewer's data becomes extrinsic: the dimension reduction, the
//! bake, and the marks over it.
//!
//! The engine is intrinsic by discipline: a `Complex` is combinatorics, a
//! geometry is edge lengths, a field is a cochain, and none of it has a
//! position or a dimension a screen can show. Something has to spend the
//! embedding, and this is where the viewer spends it, once, for everything
//! drawn afterwards.
//!
//! Two reductions carry it, and they are the same move made on the two axes:
//!
//! - Dimension reduces to a render primitive ([`surface`], [`bake`]): an
//!   $n$-manifold to $min(n, 2)$, a surface to wound triangles, a curve to
//!   segments, a point cloud to points, a solid to the 2-simplices of its
//!   boundary, all of it realized in $RR^3$ through an embedding.
//! - Grade reduces to a mark: a $k$-form to its reduced grade $min(k, n-k)$
//!   through the Hodge star, a scalar density at 0 and a tangent line field at
//!   1. The reduction itself is [`derham::reduce`], the engine's, and an
//!      exporter reads it there too, which is what makes a disagreement between
//!      the viewer and an external tool a bug in one place instead of a drift
//!      between two. What is here ([`reduce`]) is what a renderer does with it,
//!      per rendered corner rather than per simplex.
//!
//! The two compose, and the order is fixed: dimension first, grade second.
//! The object a mark is a mark *of* is the render surface, so the $n$ in
//! $min(k, n-k)$ is the surface's, never the mesh's. A 2-form on a solid
//! reduces to arrows against the volume but to a density against the boundary,
//! and only the latter is a claim about anything on screen: a flux has no
//! direction in the surface carrying it.
//!
//! Nothing here draws. A device, a buffer and a pipeline are [`crate::render`]'s,
//! and the bake is the seam between the two: everything above it is a pure data
//! transformation, and everything below it sees ambient geometry and no FEEC
//! type at all.

pub mod advect;
pub mod bake;
pub mod deposit;
pub mod glyph;
pub mod obj;
pub mod reduce;
pub mod surface;
pub mod volume;
