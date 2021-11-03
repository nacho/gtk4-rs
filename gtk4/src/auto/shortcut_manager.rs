// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use glib::object::IsA;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkShortcutManager")]
    pub struct ShortcutManager(Interface<ffi::GtkShortcutManager, ffi::GtkShortcutManagerInterface>);

    match fn {
        type_ => || ffi::gtk_shortcut_manager_get_type(),
    }
}

impl ShortcutManager {
    pub const NONE: Option<&'static ShortcutManager> = None;
}

pub trait ShortcutManagerExt: 'static {}

impl<O: IsA<ShortcutManager>> ShortcutManagerExt for O {}

impl fmt::Display for ShortcutManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ShortcutManager")
    }
}
