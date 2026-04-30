// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::fdp::Simulation;
use irox_geometry::{Vector, Vector2D};
use irox_units::units::angle::Angle;

#[derive(Debug, Clone)]
pub struct Magnetic {
    pub iterations: usize,
    pub strength: f64,
    pub field_angles: Vec<Angle>,
}
impl Magnetic {
    pub fn force(&self, sim: &mut Simulation, alpha: f64) {
        if self.field_angles.is_empty() {
            return;
        }
        #[cfg(feature = "profiling")]
        profiling::scope!("Edge::force");
        sim.iter_edges(|data, sim| {
            for _ in 0..self.iterations {
                let dists = data.get_edge_vector(sim);
                let perp = self.get_force_for_vector(dists) * (alpha);
                // if !directed {
                sim.node_mut(&data.left, |n| {
                    n.current_velocity -= perp;
                });
                // }
                sim.node_mut(&data.right, |n| {
                    n.current_velocity += perp;
                });
            }
        });
    }
    pub fn get_force_for_vector(&self, v: Vector<f64>) -> Vector<f64> {
        let ang = v.direction();
        let mut best_angle = None;
        let mut ang_diff = Angle::new_degrees(360.);
        for angle in &self.field_angles {
            match &mut best_angle {
                Some(best) => {
                    let val = angle.angle_between(ang).abs();
                    if val < ang_diff {
                        *best = *angle;
                        ang_diff = val;
                    }
                }
                None => {
                    best_angle = Some(*angle);
                    ang_diff = angle.angle_between(ang).abs();
                }
            }
        }
        let Some(angle) = best_angle else {
            return Default::default();
        };
        let dang = ang.angle_between(angle);
        let factor = (dang / 2.0).sin();
        if dang.abs() < Angle::new_degrees(1.0) {
            return Default::default();
        }
        let m = factor.signum();

        Vector::new(1.0, 0.0).rotate(ang + Angle::new_degrees(90. * m))
            * factor.abs()
            * self.strength
    }
}
