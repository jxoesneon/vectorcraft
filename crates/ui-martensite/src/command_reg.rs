//! Decoupled command catalog and taxonomy for VectorCraft.
//!
//! Ids are the engine's command ids (see `vectorcraft_engine::cmd::command_specs`), so the
//! Martensite menus, the egui UI, the command line and the control channel dispatch the same
//! commands.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CommandCategory {
    File,
    Edit,
    Object,
    Type,
    Select,
    View,
    Window,
    Help,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CommandSpec {
    pub id: &'static str,
    pub label: &'static str,
    pub category: CommandCategory,
    pub default_shortcut: Option<&'static str>,
    pub secondary_shortcut: Option<&'static str>,
}

pub const COMMAND_REGISTRY: &[CommandSpec] = &[
    // File
    CommandSpec { id: "file.new", label: "New…", category: CommandCategory::File, default_shortcut: Some("Cmd+N"), secondary_shortcut: None },
    CommandSpec {
        id: "file.newFromTemplate",
        label: "New from Template…",
        category: CommandCategory::File,
        default_shortcut: Some("Shift+Cmd+N"),
        secondary_shortcut: None,
    },
    CommandSpec { id: "file.open", label: "Open…", category: CommandCategory::File, default_shortcut: Some("Cmd+O"), secondary_shortcut: None },
    CommandSpec { id: "file.close", label: "Close", category: CommandCategory::File, default_shortcut: Some("Cmd+W"), secondary_shortcut: None },
    CommandSpec { id: "file.save", label: "Save", category: CommandCategory::File, default_shortcut: Some("Cmd+S"), secondary_shortcut: None },
    CommandSpec {
        id: "file.saveAs",
        label: "Save As…",
        category: CommandCategory::File,
        default_shortcut: Some("Shift+Cmd+S"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "file.saveCopy",
        label: "Save a Copy…",
        category: CommandCategory::File,
        default_shortcut: Some("Alt+Cmd+S"),
        secondary_shortcut: None,
    },
    CommandSpec { id: "file.revert", label: "Revert", category: CommandCategory::File, default_shortcut: Some("F12"), secondary_shortcut: None },
    CommandSpec {
        id: "file.place",
        label: "Place…",
        category: CommandCategory::File,
        default_shortcut: Some("Shift+Cmd+P"),
        secondary_shortcut: None,
    },
    CommandSpec { id: "file.print", label: "Print…", category: CommandCategory::File, default_shortcut: Some("Cmd+P"), secondary_shortcut: None },
    // Edit
    CommandSpec { id: "edit.undo", label: "Undo", category: CommandCategory::Edit, default_shortcut: Some("Cmd+Z"), secondary_shortcut: None },
    CommandSpec { id: "edit.redo", label: "Redo", category: CommandCategory::Edit, default_shortcut: Some("Shift+Cmd+Z"), secondary_shortcut: None },
    CommandSpec { id: "edit.cut", label: "Cut", category: CommandCategory::Edit, default_shortcut: Some("Cmd+X"), secondary_shortcut: Some("F2") },
    CommandSpec { id: "edit.copy", label: "Copy", category: CommandCategory::Edit, default_shortcut: Some("Cmd+C"), secondary_shortcut: Some("F3") },
    CommandSpec {
        id: "edit.paste",
        label: "Paste",
        category: CommandCategory::Edit,
        default_shortcut: Some("Cmd+V"),
        secondary_shortcut: Some("F4"),
    },
    CommandSpec {
        id: "edit.pasteInFront",
        label: "Paste in Front",
        category: CommandCategory::Edit,
        default_shortcut: Some("Cmd+F"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "edit.pasteInBack",
        label: "Paste in Back",
        category: CommandCategory::Edit,
        default_shortcut: Some("Cmd+B"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "edit.pasteInPlace",
        label: "Paste in Place",
        category: CommandCategory::Edit,
        default_shortcut: Some("Shift+Cmd+V"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "edit.clear",
        label: "Clear",
        category: CommandCategory::Edit,
        default_shortcut: Some("Delete"),
        secondary_shortcut: Some("Backspace"),
    },
    // Object
    CommandSpec {
        id: "object.transformAgain",
        label: "Transform Again",
        category: CommandCategory::Object,
        default_shortcut: Some("Cmd+D"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "object.transformEach",
        label: "Transform Each…",
        category: CommandCategory::Object,
        default_shortcut: Some("Alt+Shift+Cmd+D"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "object.arrange.bringToFront",
        label: "Bring to Front",
        category: CommandCategory::Object,
        default_shortcut: Some("Shift+Cmd+]"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "object.arrange.bringForward",
        label: "Bring Forward",
        category: CommandCategory::Object,
        default_shortcut: Some("Cmd+]"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "object.arrange.sendBackward",
        label: "Send Backward",
        category: CommandCategory::Object,
        default_shortcut: Some("Cmd+["),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "object.arrange.sendToBack",
        label: "Send to Back",
        category: CommandCategory::Object,
        default_shortcut: Some("Shift+Cmd+["),
        secondary_shortcut: None,
    },
    CommandSpec { id: "object.group", label: "Group", category: CommandCategory::Object, default_shortcut: Some("Cmd+G"), secondary_shortcut: None },
    CommandSpec {
        id: "object.ungroup",
        label: "Ungroup",
        category: CommandCategory::Object,
        default_shortcut: Some("Shift+Cmd+G"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "object.lock",
        label: "Lock Selection",
        category: CommandCategory::Object,
        default_shortcut: Some("Cmd+2"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "object.hide",
        label: "Hide Selection",
        category: CommandCategory::Object,
        default_shortcut: Some("Cmd+3"),
        secondary_shortcut: None,
    },
    CommandSpec { id: "path.join", label: "Join", category: CommandCategory::Object, default_shortcut: Some("Cmd+J"), secondary_shortcut: None },
    CommandSpec {
        id: "path.average",
        label: "Average…",
        category: CommandCategory::Object,
        default_shortcut: Some("Alt+Cmd+J"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "object.pathfinder.unite",
        label: "Unite",
        category: CommandCategory::Object,
        default_shortcut: None,
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "object.pathfinder.minusFront",
        label: "Minus Front",
        category: CommandCategory::Object,
        default_shortcut: None,
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "object.pathfinder.intersect",
        label: "Intersect",
        category: CommandCategory::Object,
        default_shortcut: None,
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "object.pathfinder.exclude",
        label: "Exclude",
        category: CommandCategory::Object,
        default_shortcut: None,
        secondary_shortcut: None,
    },
    // Type
    CommandSpec {
        id: "type.createOutlines",
        label: "Create Outlines",
        category: CommandCategory::Type,
        default_shortcut: Some("Shift+Cmd+O"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "type.size.increase",
        label: "Increase Size",
        category: CommandCategory::Type,
        default_shortcut: Some("Shift+Cmd+>"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "type.size.decrease",
        label: "Decrease Size",
        category: CommandCategory::Type,
        default_shortcut: Some("Shift+Cmd+<"),
        secondary_shortcut: None,
    },
    // Select
    CommandSpec { id: "select.all", label: "All", category: CommandCategory::Select, default_shortcut: Some("Cmd+A"), secondary_shortcut: None },
    CommandSpec {
        id: "select.allOnArtboard",
        label: "All on Active Artboard",
        category: CommandCategory::Select,
        default_shortcut: Some("Alt+Cmd+A"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "select.none",
        label: "Deselect",
        category: CommandCategory::Select,
        default_shortcut: Some("Shift+Cmd+A"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "select.reselect",
        label: "Reselect",
        category: CommandCategory::Select,
        default_shortcut: Some("Cmd+6"),
        secondary_shortcut: None,
    },
    CommandSpec { id: "select.inverse", label: "Inverse", category: CommandCategory::Select, default_shortcut: None, secondary_shortcut: None },
    CommandSpec {
        id: "select.same.fillColor",
        label: "Same Fill Color",
        category: CommandCategory::Select,
        default_shortcut: None,
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "select.same.strokeColor",
        label: "Same Stroke Color",
        category: CommandCategory::Select,
        default_shortcut: None,
        secondary_shortcut: None,
    },
    // View
    CommandSpec { id: "view.outline", label: "Outline", category: CommandCategory::View, default_shortcut: Some("Cmd+Y"), secondary_shortcut: None },
    CommandSpec {
        id: "view.overprintPreview",
        label: "Overprint Preview",
        category: CommandCategory::View,
        default_shortcut: Some("Alt+Shift+Cmd+Y"),
        secondary_shortcut: None,
    },
    CommandSpec { id: "view.rulers", label: "Rulers", category: CommandCategory::View, default_shortcut: Some("Cmd+R"), secondary_shortcut: None },
    CommandSpec {
        id: "view.transparencyGrid",
        label: "Transparency Grid",
        category: CommandCategory::View,
        default_shortcut: Some("Shift+Cmd+D"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "view.guides.lock",
        label: "Lock Guides",
        category: CommandCategory::View,
        default_shortcut: Some("Alt+Cmd+;"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "view.guides.make",
        label: "Make Guides",
        category: CommandCategory::View,
        default_shortcut: Some("Cmd+5"),
        secondary_shortcut: None,
    },
];

pub fn find_command(id: &str) -> Option<&'static CommandSpec> {
    COMMAND_REGISTRY.iter().find(|cmd| cmd.id == id)
}

pub fn commands_by_category(category: CommandCategory) -> Vec<&'static CommandSpec> {
    COMMAND_REGISTRY.iter().filter(|cmd| cmd.category == category).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_command_ids_are_unique() {
        let mut ids = HashSet::new();
        for cmd in COMMAND_REGISTRY {
            assert!(ids.insert(cmd.id), "Duplicate command ID detected: {}", cmd.id);
        }
    }

    #[test]
    fn test_command_ids_exist_in_engine() {
        // Shell-level commands the app window dispatches itself (file dialogs, view toggles) —
        // the same split as `vectorcraft_ui_egui::menus::UI_COMMANDS`. Every other registry id
        // must be a real engine command.
        const SHELL_COMMANDS: &[&str] = &["file.open", "file.save", "view.outline", "view.rulers"];
        for cmd in COMMAND_REGISTRY {
            assert!(
                SHELL_COMMANDS.contains(&cmd.id) || vectorcraft_engine::find_command(cmd.id).is_some(),
                "registry id {} is neither a shell nor an engine command",
                cmd.id
            );
        }
    }

    #[test]
    fn test_lookup_finds_all_commands() {
        for cmd in COMMAND_REGISTRY {
            let found = find_command(cmd.id);
            assert!(found.is_some());
            assert_eq!(found.unwrap().label, cmd.label);
        }
    }

    #[test]
    fn test_categories_populated() {
        assert!(!commands_by_category(CommandCategory::File).is_empty());
        assert!(!commands_by_category(CommandCategory::Edit).is_empty());
        assert!(!commands_by_category(CommandCategory::Object).is_empty());
        assert!(!commands_by_category(CommandCategory::Type).is_empty());
        assert!(!commands_by_category(CommandCategory::Select).is_empty());
        assert!(!commands_by_category(CommandCategory::View).is_empty());
    }
}
