//! End-to-end vector illustration integration tests.

use vectorcraft_ui_martensite::{
    VectorcraftApp, VectorTool,
    pathfinder::{BooleanOp, PathfinderEngine},
    shortcuts::map_shortcut,
};

#[test]
fn test_vector_editing_workflow() {
    let mut app = VectorcraftApp::new();

    // 1. Tool selection
    assert_eq!(map_shortcut("p"), Some(VectorTool::Pen));
    app.active_tool = VectorTool::Pen;

    // 2. Stroke adjustment
    app.set_stroke_width(3.0);
    assert_eq!(app.stroke_width, 3.0);

    // 3. Pathfinder union of two bounding boxes
    let a = [10.0, 10.0, 50.0, 50.0];
    let b = [40.0, 40.0, 100.0, 100.0];
    let united = PathfinderEngine::combine_bounding_boxes(BooleanOp::Unite, a, b);
    assert_eq!(united, Some([10.0, 10.0, 100.0, 100.0]));
}
