/// Core abstractions:
///     - Point<P> -> Represents a location in 2D space
///     - Direction -> Represents movement vectors (Cardinal/Octal)
///     - Grid<T> -> Represents a bounded 2D space containing values
mod helpers;
mod macros;
pub mod prelude {
    pub use crate::helpers::{
        direction::{
            CardinalDirections, DiagonalDirections, DirectionBehaviour, DirectionalMove,
            OctalDirections, RotationBehaviour,
        },
        grid::{BoolGrid, CharGrid, Grid, GridPoint, GridView, IntGrid, Rectangle, WorldPoint},
        point::Point,
    };
}
