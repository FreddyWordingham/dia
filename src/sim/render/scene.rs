//! Scene implementation.

use crate::{
    access,
    render::{Attribute, Camera, Settings},
    Adaptive, Set,
};
use palette::{Gradient, LinSrgba};

/// Rendering scene data collection.
pub struct Scene<'a> {
    /// Grid of surfaces.
    grid: &'a Adaptive<'a>,
    /// Rendering settings,
    sett: &'a Settings,
    /// Capturing camera,
    cam: &'a Camera,
    /// Colour set,
    cols: &'a Set<Gradient<LinSrgba>>,
    /// Attribute set.
    attrs: &'a Set<Attribute>,
}

impl<'a> Scene<'a> {
    access!(grid, Adaptive);
    access!(sett, Settings);
    access!(cam, Camera);
    access!(cols, Set<Gradient<LinSrgba>>);
    access!(attrs, Set<Attribute>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(
        grid: &'a Adaptive<'a>,
        sett: &'a Settings,
        cam: &'a Camera,
        cols: &'a Set<Gradient<LinSrgba>>,
        attrs: &'a Set<Attribute>,
    ) -> Self {
        Self {
            grid,
            sett,
            cam,
            cols,
            attrs,
        }
    }
}
