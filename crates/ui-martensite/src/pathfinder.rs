//! Vector boolean pathfinder operations.

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BooleanOp {
    Unite,
    MinusFront,
    Intersect,
    Exclude,
}

pub struct PathfinderEngine;

impl PathfinderEngine {
    pub fn combine_bounding_boxes(op: BooleanOp, a: [f32; 4], b: [f32; 4]) -> Option<[f32; 4]> {
        match op {
            BooleanOp::Unite => Some([a[0].min(b[0]), a[1].min(b[1]), a[2].max(b[2]), a[3].max(b[3])]),
            BooleanOp::Intersect => {
                let x0 = a[0].max(b[0]);
                let y0 = a[1].max(b[1]);
                let x1 = a[2].min(b[2]);
                let y1 = a[3].min(b[3]);
                if x0 < x1 && y0 < y1 { Some([x0, y0, x1, y1]) } else { None }
            }
            _ => Some(a),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pathfinder_box_unite() {
        let a = [0.0, 0.0, 10.0, 10.0];
        let b = [5.0, 5.0, 20.0, 20.0];
        let u = PathfinderEngine::combine_bounding_boxes(BooleanOp::Unite, a, b);
        assert_eq!(u, Some([0.0, 0.0, 20.0, 20.0]));
    }
}
