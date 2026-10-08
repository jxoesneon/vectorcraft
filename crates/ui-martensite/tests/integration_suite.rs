//! Comprehensive integration test suite for the VectorCraft Martensite UI.
//!
//! Validates end-to-end integration across commands, menus, shortcuts,
//! scrubby inputs, pathfinder booleans, and canvas coordinates.

use vectorcraft_engine::{Engine, ToolId};
use vectorcraft_ui_martensite::{
    VectorcraftApp,
    command_reg::{COMMAND_REGISTRY, find_command},
    menus::generate_main_menu,
    pathfinder::{BooleanOp, PathfinderEngine},
    theme::CraftTheme,
    widgets::{CanvasViewWidget, DockPanelGroup, LayerItemDef, LayerTreeWidget, OptionsBarWidget, ScrubbyInputWidget, ToolStripWidget},
};

#[test]
fn test_end_to_end_workspace_interaction() {
    let engine = Engine::new();
    let mut app = VectorcraftApp::new(engine);

    // 1. Initial State Verification
    assert_eq!(app.active_tool, ToolId::Selection);
    assert_eq!(app.zoom_level, 1.0);
    assert!(app.rulers_visible);
    assert!(!app.outline_preview);

    // 2. Keystroke Workflow: switch to Pen, zoom in, hold Space to pan
    let new_tool = app.keyboard.on_key_down("p", app.active_tool);
    assert_eq!(new_tool, Some(ToolId::Pen));
    app.set_tool(ToolId::Pen);

    app.set_zoom(2.0);
    assert_eq!(app.zoom_level, 2.0);

    // Spring-loaded Hand tool
    let hand_tool = app.keyboard.on_key_down("Space", app.active_tool);
    assert_eq!(hand_tool, Some(ToolId::Hand));
    app.set_tool(ToolId::Hand);

    app.pan_by(50.0, 100.0);
    assert_eq!(app.pan_offset, [50.0, 100.0]);

    // Release Space restores Pen
    let restored_tool = app.keyboard.on_key_up("Space");
    assert_eq!(restored_tool, Some(ToolId::Pen));
    app.set_tool(ToolId::Pen);

    // 3. Options Bar Interaction for the active tool
    let mut options = OptionsBarWidget::new();
    options.set_tool(ToolId::Pen);
    options.stroke_weight.on_pointer_down(0.0);
    options.stroke_weight.on_pointer_move(20.0, false, false);
    options.stroke_weight.on_pointer_up();
    assert_eq!(options.stroke_weight.value, 21.0); // 1 + 20

    // 4. Layer Tree & Hierarchy Updates
    let mut layers = LayerTreeWidget::new();
    layers.layers.push(LayerItemDef {
        id: 1,
        name: "Layer 1".to_string(),
        visible: true,
        locked: true,
        targeted: true,
        has_clipping_mask: false,
        is_group: false,
        expanded: false,
        children: vec![],
    });
    layers.layers.push(LayerItemDef {
        id: 2,
        name: "Artwork".to_string(),
        visible: true,
        locked: false,
        targeted: false,
        has_clipping_mask: true,
        is_group: true,
        expanded: false,
        children: vec![],
    });
    layers.select_layer(2);
    assert_eq!(layers.selected_layer_id, Some(2));
    layers.toggle_visibility(2);
    assert!(!layers.layers[1].visible);

    // 5. Pathfinder boolean bounding-box combination
    let a = [10.0, 10.0, 50.0, 50.0];
    let b = [40.0, 40.0, 100.0, 100.0];
    let united = PathfinderEngine::combine_bounding_boxes(BooleanOp::Unite, a, b);
    assert_eq!(united, Some([10.0, 10.0, 100.0, 100.0]));
    let intersected = PathfinderEngine::combine_bounding_boxes(BooleanOp::Intersect, a, b);
    assert_eq!(intersected, Some([40.0, 40.0, 50.0, 50.0]));

    // 6. Docking System Validation
    let mut dock = DockPanelGroup::new(&["Layers", "Appearance", "Pathfinder"]);
    dock.select_tab(2);
    assert_eq!(dock.active_tab, 2);
    dock.toggle_collapsed();
    assert!(dock.collapsed_to_icons);

    // 7. Menu Generation Consistency: every item is a registered command
    let menus = generate_main_menu();
    assert!(!menus.is_empty());
    for menu in &menus {
        for item in &menu.items {
            if let Some(cmd_id) = item.command_id {
                assert!(find_command(cmd_id).is_some(), "Unknown command in menu: {}", cmd_id);
            }
        }
    }

    // 8. Theme Color Space Consistency
    let theme = CraftTheme::dark_neutral();
    let obsidian = CraftTheme::studio_obsidian();
    assert_ne!(theme.surface_app_bg, obsidian.surface_app_bg);

    // 9. Registry is populated for every menu category
    assert!(COMMAND_REGISTRY.len() > 30);
}

#[test]
fn test_tool_strip_and_canvas_view() {
    let mut strip = ToolStripWidget::new();
    assert_eq!(strip.active_tool, ToolId::Selection);
    strip.swap_fill_stroke();
    assert_eq!(strip.fill_color, [0, 0, 0, 255]);
    strip.reset_default_colors();
    assert_eq!(strip.fill_color, [255, 255, 255, 255]);

    let mut canvas = CanvasViewWidget::new(612, 792); // US Letter at 72 dpi
    canvas.zoom_at(4.0, [306.0, 396.0]);
    assert_eq!(canvas.zoom, 4.0);

    let mut scrub = ScrubbyInputWidget::new("Stroke", 2.0, 0.0, 100.0, "pt");
    scrub.set_direct_value(150.0);
    assert_eq!(scrub.value, 100.0); // clamped
}
