//! Light structure.

use crate::{display_field, display_field_ln, mcrt::Photon, Emit, Mesh, Pos3, Probability, Ray};
use ndarray::Array1;
use rand::{rngs::ThreadRng, Rng};
use std::fmt::{Display, Formatter, Result};

/// Photon emission enumeration.
pub enum Light {
    /// Surface type
    Surface {
        /// Surface.
        surf: Mesh,
        /// Emission Formula.
        spec: Probability,
        /// Power [J/s].
        power: f64,
    },
    /// Points type.
    Points {
        /// Weighted emission points.
        points: Array1<(Pos3, f64)>,
        /// Power [J/s].
        power: f64,
        /// Emission Formula.
        spec: Probability,
    },
}

impl Light {
    /// Construct a new surface instance.
    #[inline]
    #[must_use]
    pub fn new(surf: Mesh, spec: Probability, power: f64) -> Self {
        debug_assert!(power > 0.0);

        Self::Surface { surf, spec, power }
    }

    /// Construct a new points instance.
    #[inline]
    #[must_use]
    pub fn new_points(mut points: Vec<(Pos3, f64)>, spec: Probability, power: f64) -> Self {
        debug_assert!(!points.is_empty());

        let mut total_weight = 0.0;
        for (_p, w) in &points {
            total_weight += w;
        }

        let mut cum = 0.0;
        for (_p, w) in &mut points {
            cum += *w / total_weight;
            *w = cum;
        }

        Self::Points {
            points: Array1::from(points),
            spec,
            power,
        }
    }

    /// Reference the light's surface mesh.
    #[inline]
    #[must_use]
    pub fn surf(&self) -> &Mesh {
        match self {
            Self::Surface { surf, .. } => surf,
            Self::Points { .. } => {
                panic!("No surface.");
            }
        }
    }

    /// Reference the light's emission formula.
    #[inline]
    #[must_use]
    pub fn spec(&self) -> &Probability {
        match self {
            Self::Surface { spec, .. } | Self::Points { spec, .. } => spec,
        }
    }

    /// Reference the light's power.
    #[inline]
    #[must_use]
    pub fn power(&self) -> f64 {
        match self {
            Self::Surface { power, .. } | Self::Points { power, .. } => *power,
        }
    }

    /// Generate a ray.
    #[inline]
    #[must_use]
    pub fn gen_ray(&self, rng: &mut ThreadRng) -> Ray {
        match self {
            Self::Surface {
                surf,
                spec: _,
                power: _,
            } => surf.cast(rng),
            Self::Points {
                points,
                power: _,
                spec: _,
            } => {
                let r = rng.gen::<f64>();
                for (p, x) in points {
                    if r <= *x {
                        let dir = crate::Vec3::x_axis(); // TODO: Make random.
                        return Ray::new(*p, dir);
                    }
                }
                panic!("Invalid point weightings.");
            }
        }
    }

    /// Emit a photon.
    #[inline]
    #[must_use]
    pub fn emit(&self, total_phot: u64, rng: &mut ThreadRng) -> Photon {
        debug_assert!(total_phot > 0);

        let ray = self.gen_ray(rng);

        let wavelength = self.spec().gen(rng);
        let power = self.power() / total_phot as f64;

        Photon::new(ray, wavelength, power)
    }
}

impl Display for Light {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        match self {
            Self::Surface { .. } => {
                display_field_ln!(fmt, "type", "Emission surface")?;
            }
            Self::Points { .. } => {
                display_field_ln!(fmt, "type", "Point array")?;
            }
        }
        display_field_ln!(fmt, "formula", self.spec())?;
        display_field!(fmt, "power", self.power(), "J/s")
    }
}
