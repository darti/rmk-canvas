use std::iter::once;

use kurbo::BezPath;
use kurbo::PathEl;
use kurbo::Point as KPoint;
use kurbo::Vec2;
use rmk_format::format::Point;
use rmk_format::format::Stroke;

pub trait ToPath {
    fn to_path(&self) -> Option<BezPath>;
}

trait ToKPoint {
    fn to_kpoint(&self) -> KPoint;
}

impl ToKPoint for Point {
    #[inline]
    fn to_kpoint(&self) -> KPoint {
        self.coords().into()
    }
}

impl ToPath for Stroke {
    fn to_path(&self) -> Option<BezPath> {
        if self.points.len() < 2 {
            return None;
        }

        let normals: Vec<Vec2> = self
            .points
            .iter()
            .zip(self.points.iter().skip(1))
            .map(|(p0, p1)| {
                let p0 = p0.to_kpoint();
                let p1 = p1.to_kpoint();

                let t = p1 - p0;
                Vec2::new(-t.y, t.x).normalize()
            })
            .collect();

        let normals = once(normals.first().unwrap())
            .chain(normals.iter())
            .zip(normals.iter().chain(once(normals.last().unwrap())))
            .map(|(n0, n1)| (*n0 + *n1).normalize());

        let mut double_points = self.points.iter().zip(normals).map(|(p, n)| {
            let kp = p.to_kpoint();
            (kp /*+ n * p.width.into()*/, kp - n * p.width.into())
        });

        let (start, end) = double_points.next().unwrap();

        let mut forward_path = Vec::with_capacity(self.points.len());
        let mut backward_path = Vec::with_capacity(self.points.len());

        // let mut backward_path = BezPath::default();

        forward_path.push(PathEl::MoveTo(start));
        backward_path.push(PathEl::LineTo(end));

        for (p0, p1) in double_points {
            forward_path.push(PathEl::LineTo(p0));
            backward_path.push(PathEl::LineTo(p1));
        }

        // forward_path.extend(backward_path.elements().iter().rev().clone_into(target));

        Some(BezPath::from_vec(forward_path))
    }
}
