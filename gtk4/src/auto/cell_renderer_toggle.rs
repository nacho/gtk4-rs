// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT
#![allow(deprecated)]

use crate::{ffi, CellRenderer, CellRendererMode, TreePath};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkCellRendererToggle")]
    pub struct CellRendererToggle(Object<ffi::GtkCellRendererToggle>) @extends CellRenderer;

    match fn {
        type_ => || ffi::gtk_cell_renderer_toggle_get_type(),
    }
}

impl CellRendererToggle {
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_renderer_toggle_new")]
    pub fn new() -> CellRendererToggle {
        assert_initialized_main_thread!();
        unsafe { CellRenderer::from_glib_none(ffi::gtk_cell_renderer_toggle_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`CellRendererToggle`] objects.
    ///
    /// This method returns an instance of [`CellRendererToggleBuilder`](crate::builders::CellRendererToggleBuilder) which can be used to create [`CellRendererToggle`] objects.
    pub fn builder() -> CellRendererToggleBuilder {
        CellRendererToggleBuilder::new()
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_renderer_toggle_get_activatable")]
    #[doc(alias = "get_activatable")]
    #[doc(alias = "activatable")]
    pub fn is_activatable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_renderer_toggle_get_activatable(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_renderer_toggle_get_active")]
    #[doc(alias = "get_active")]
    #[doc(alias = "active")]
    pub fn is_active(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_renderer_toggle_get_active(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_renderer_toggle_get_radio")]
    #[doc(alias = "get_radio")]
    #[doc(alias = "radio")]
    pub fn is_radio(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_renderer_toggle_get_radio(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_renderer_toggle_set_activatable")]
    #[doc(alias = "activatable")]
    pub fn set_activatable(&self, setting: bool) {
        unsafe {
            ffi::gtk_cell_renderer_toggle_set_activatable(
                self.to_glib_none().0,
                setting.into_glib(),
            );
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_renderer_toggle_set_active")]
    #[doc(alias = "active")]
    pub fn set_active(&self, setting: bool) {
        unsafe {
            ffi::gtk_cell_renderer_toggle_set_active(self.to_glib_none().0, setting.into_glib());
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_renderer_toggle_set_radio")]
    #[doc(alias = "radio")]
    pub fn set_radio(&self, radio: bool) {
        unsafe {
            ffi::gtk_cell_renderer_toggle_set_radio(self.to_glib_none().0, radio.into_glib());
        }
    }

    pub fn is_inconsistent(&self) -> bool {
        ObjectExt::property(self, "inconsistent")
    }

    pub fn set_inconsistent(&self, inconsistent: bool) {
        ObjectExt::set_property(self, "inconsistent", inconsistent)
    }

    #[doc(alias = "toggled")]
    pub fn connect_toggled<F: Fn(&Self, TreePath) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn toggled_trampoline<F: Fn(&CellRendererToggle, TreePath) + 'static>(
            this: *mut ffi::GtkCellRendererToggle,
            path: *mut std::ffi::c_char,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            let path = from_glib_full(crate::ffi::gtk_tree_path_new_from_string(path));
            f(&from_glib_borrow(this), path)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"toggled\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    toggled_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "activatable")]
    pub fn connect_activatable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_activatable_trampoline<F: Fn(&CellRendererToggle) + 'static>(
            this: *mut ffi::GtkCellRendererToggle,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::activatable\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_activatable_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "active")]
    pub fn connect_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_active_trampoline<F: Fn(&CellRendererToggle) + 'static>(
            this: *mut ffi::GtkCellRendererToggle,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::active\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_active_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "inconsistent")]
    pub fn connect_inconsistent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_inconsistent_trampoline<
            F: Fn(&CellRendererToggle) + 'static,
        >(
            this: *mut ffi::GtkCellRendererToggle,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::inconsistent\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_inconsistent_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "radio")]
    pub fn connect_radio_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_radio_trampoline<F: Fn(&CellRendererToggle) + 'static>(
            this: *mut ffi::GtkCellRendererToggle,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::radio\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_radio_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for CellRendererToggle {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`CellRendererToggle`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct CellRendererToggleBuilder {
    builder: glib::object::ObjectBuilder<'static, CellRendererToggle>,
}

impl CellRendererToggleBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn activatable(self, activatable: bool) -> Self {
        Self {
            builder: self.builder.property("activatable", activatable),
        }
    }

    pub fn active(self, active: bool) -> Self {
        Self {
            builder: self.builder.property("active", active),
        }
    }

    pub fn inconsistent(self, inconsistent: bool) -> Self {
        Self {
            builder: self.builder.property("inconsistent", inconsistent),
        }
    }

    pub fn radio(self, radio: bool) -> Self {
        Self {
            builder: self.builder.property("radio", radio),
        }
    }

    pub fn cell_background(self, cell_background: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("cell-background", cell_background.into()),
        }
    }

    pub fn cell_background_rgba(self, cell_background_rgba: &gdk::RGBA) -> Self {
        Self {
            builder: self
                .builder
                .property("cell-background-rgba", cell_background_rgba),
        }
    }

    pub fn cell_background_set(self, cell_background_set: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("cell-background-set", cell_background_set),
        }
    }

    pub fn height(self, height: i32) -> Self {
        Self {
            builder: self.builder.property("height", height),
        }
    }

    pub fn is_expanded(self, is_expanded: bool) -> Self {
        Self {
            builder: self.builder.property("is-expanded", is_expanded),
        }
    }

    pub fn is_expander(self, is_expander: bool) -> Self {
        Self {
            builder: self.builder.property("is-expander", is_expander),
        }
    }

    pub fn mode(self, mode: CellRendererMode) -> Self {
        Self {
            builder: self.builder.property("mode", mode),
        }
    }

    pub fn sensitive(self, sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("sensitive", sensitive),
        }
    }

    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    pub fn width(self, width: i32) -> Self {
        Self {
            builder: self.builder.property("width", width),
        }
    }

    pub fn xalign(self, xalign: f32) -> Self {
        Self {
            builder: self.builder.property("xalign", xalign),
        }
    }

    pub fn xpad(self, xpad: u32) -> Self {
        Self {
            builder: self.builder.property("xpad", xpad),
        }
    }

    pub fn yalign(self, yalign: f32) -> Self {
        Self {
            builder: self.builder.property("yalign", yalign),
        }
    }

    pub fn ypad(self, ypad: u32) -> Self {
        Self {
            builder: self.builder.property("ypad", ypad),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`CellRendererToggle`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> CellRendererToggle {
        self.builder.build()
    }
}
