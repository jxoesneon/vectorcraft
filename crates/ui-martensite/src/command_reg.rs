//! Vector command registry.

pub struct Command {
    pub id: &'static str,
    pub label: &'static str,
    pub shortcut: Option<&'static str>,
}

pub const COMMANDS: &[Command] = &[
    Command { id: "object.group", label: "Group", shortcut: Some("Cmd+G") },
    Command { id: "object.ungroup", label: "Ungroup", shortcut: Some("Shift+Cmd+G") },
    Command { id: "path.join", label: "Join Path", shortcut: Some("Cmd+J") },
    Command { id: "view.outline", label: "Outline Mode", shortcut: Some("Cmd+Y") },
];
