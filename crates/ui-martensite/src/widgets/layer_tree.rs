//! Layer hierarchy tree widget (the Layers panel's rows: layers, sublayers, groups and objects).

#[derive(Clone, Debug, PartialEq)]
pub struct LayerItemDef {
    pub id: u64,
    pub name: String,
    pub visible: bool,
    pub locked: bool,
    /// The row's target circle is filled: its appearance is targeted for effects.
    pub targeted: bool,
    /// The layer or group carries a clipping mask (the top of a clipping set).
    pub has_clipping_mask: bool,
    pub is_group: bool,
    pub expanded: bool,
    pub children: Vec<LayerItemDef>,
}

pub struct LayerTreeWidget {
    pub layers: Vec<LayerItemDef>,
    pub selected_layer_id: Option<u64>,
    pub opacity: f32,
    pub blend_mode: String,
}

impl LayerTreeWidget {
    pub fn new() -> Self {
        Self { layers: Vec::new(), selected_layer_id: None, opacity: 100.0, blend_mode: "Normal".to_string() }
    }

    pub fn select_layer(&mut self, id: u64) {
        self.selected_layer_id = Some(id);
    }

    pub fn toggle_visibility(&mut self, id: u64) {
        if let Some(item) = find_layer_mut(&mut self.layers, id) {
            item.visible = !item.visible;
        }
    }

    pub fn toggle_lock(&mut self, id: u64) {
        if let Some(item) = find_layer_mut(&mut self.layers, id) {
            item.locked = !item.locked;
        }
    }

    pub fn toggle_expanded(&mut self, id: u64) {
        if let Some(item) = find_layer_mut(&mut self.layers, id) {
            item.expanded = !item.expanded;
        }
    }
}

fn find_layer_mut(items: &mut [LayerItemDef], id: u64) -> Option<&mut LayerItemDef> {
    for item in items.iter_mut() {
        if item.id == id {
            return Some(item);
        }
        if let Some(found) = find_layer_mut(&mut item.children, id) {
            return Some(found);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layer_tree_mutation() {
        let mut tree = LayerTreeWidget::new();
        tree.layers.push(LayerItemDef {
            id: 1,
            name: "Layer 1".to_string(),
            visible: true,
            locked: false,
            targeted: true,
            has_clipping_mask: false,
            is_group: false,
            expanded: true,
            children: vec![LayerItemDef {
                id: 3,
                name: "<Path>".to_string(),
                visible: true,
                locked: false,
                targeted: false,
                has_clipping_mask: false,
                is_group: false,
                expanded: false,
                children: vec![],
            }],
        });
        tree.layers.push(LayerItemDef {
            id: 2,
            name: "Layer 2".to_string(),
            visible: true,
            locked: false,
            targeted: false,
            has_clipping_mask: true,
            is_group: true,
            expanded: false,
            children: vec![],
        });

        tree.select_layer(2);
        assert_eq!(tree.selected_layer_id, Some(2));

        tree.toggle_visibility(2);
        assert!(!tree.layers[1].visible);
        tree.toggle_visibility(2);
        assert!(tree.layers[1].visible);

        tree.toggle_lock(2);
        assert!(tree.layers[1].locked);

        // Nested rows are found too
        tree.toggle_visibility(3);
        let child = &tree.layers[0].children[0];
        assert!(!child.visible);

        tree.toggle_expanded(1);
        assert!(!tree.layers[0].expanded);
    }
}
