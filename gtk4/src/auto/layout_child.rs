// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::LayoutManager;
use crate::Widget;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkLayoutChild")]
    pub struct LayoutChild(Object<ffi::GtkLayoutChild, ffi::GtkLayoutChildClass>);

    match fn {
        type_ => || ffi::gtk_layout_child_get_type(),
    }
}

impl LayoutChild {
    pub const NONE: Option<&'static LayoutChild> = None;
}

pub trait LayoutChildExt: 'static {
    #[doc(alias = "gtk_layout_child_get_child_widget")]
    #[doc(alias = "get_child_widget")]
    fn child_widget(&self) -> Option<Widget>;

    #[doc(alias = "gtk_layout_child_get_layout_manager")]
    #[doc(alias = "get_layout_manager")]
    fn layout_manager(&self) -> Option<LayoutManager>;
}

impl<O: IsA<LayoutChild>> LayoutChildExt for O {
    fn child_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_layout_child_get_child_widget(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn layout_manager(&self) -> Option<LayoutManager> {
        unsafe {
            from_glib_none(ffi::gtk_layout_child_get_layout_manager(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for LayoutChild {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("LayoutChild")
    }
}
