//! Vector menus.

pub struct MenuCategory {
    pub title: &'static str,
    pub items: &'static [&'static str],
}

pub const MENUS: &[MenuCategory] = &[
    MenuCategory { title: "File", items: &["file.new", "file.export_svg"] },
    MenuCategory { title: "Object", items: &["object.group", "object.ungroup"] },
    MenuCategory { title: "Path", items: &["path.join"] },
];
