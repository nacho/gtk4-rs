// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(feature = "v4_10")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
use crate::AccessibleRange;
use crate::{
    ffi, Accessible, AccessibleRole, Align, Buildable, ConstraintTarget, LayoutManager,
    LevelBarMode, Orientable, Orientation, Overflow, Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

#[cfg(feature = "v4_10")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
glib::wrapper! {
    #[doc(alias = "GtkLevelBar")]
    pub struct LevelBar(Object<ffi::GtkLevelBar>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget, AccessibleRange, Orientable;

    match fn {
        type_ => || ffi::gtk_level_bar_get_type(),
    }
}

#[cfg(not(any(feature = "v4_10")))]
glib::wrapper! {
    #[doc(alias = "GtkLevelBar")]
    pub struct LevelBar(Object<ffi::GtkLevelBar>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget, Orientable;

    match fn {
        type_ => || ffi::gtk_level_bar_get_type(),
    }
}

impl LevelBar {
    #[doc(alias = "gtk_level_bar_new")]
    pub fn new() -> LevelBar {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_level_bar_new()).unsafe_cast() }
    }

    #[doc(alias = "gtk_level_bar_new_for_interval")]
    #[doc(alias = "new_for_interval")]
    pub fn for_interval(min_value: f64, max_value: f64) -> LevelBar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_level_bar_new_for_interval(min_value, max_value))
                .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`LevelBar`] objects.
    ///
    /// This method returns an instance of [`LevelBarBuilder`](crate::builders::LevelBarBuilder) which can be used to create [`LevelBar`] objects.
    pub fn builder() -> LevelBarBuilder {
        LevelBarBuilder::new()
    }

    #[doc(alias = "gtk_level_bar_add_offset_value")]
    pub fn add_offset_value(&self, name: &str, value: f64) {
        unsafe {
            ffi::gtk_level_bar_add_offset_value(
                self.to_glib_none().0,
                name.to_glib_none().0,
                value,
            );
        }
    }

    #[doc(alias = "gtk_level_bar_get_inverted")]
    #[doc(alias = "get_inverted")]
    #[doc(alias = "inverted")]
    pub fn is_inverted(&self) -> bool {
        unsafe { from_glib(ffi::gtk_level_bar_get_inverted(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_level_bar_get_max_value")]
    #[doc(alias = "get_max_value")]
    #[doc(alias = "max-value")]
    pub fn max_value(&self) -> f64 {
        unsafe { ffi::gtk_level_bar_get_max_value(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_level_bar_get_min_value")]
    #[doc(alias = "get_min_value")]
    #[doc(alias = "min-value")]
    pub fn min_value(&self) -> f64 {
        unsafe { ffi::gtk_level_bar_get_min_value(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_level_bar_get_mode")]
    #[doc(alias = "get_mode")]
    pub fn mode(&self) -> LevelBarMode {
        unsafe { from_glib(ffi::gtk_level_bar_get_mode(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_level_bar_get_offset_value")]
    #[doc(alias = "get_offset_value")]
    pub fn offset_value(&self, name: Option<&str>) -> Option<f64> {
        unsafe {
            let mut value = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gtk_level_bar_get_offset_value(
                self.to_glib_none().0,
                name.to_glib_none().0,
                value.as_mut_ptr(),
            ));
            if ret {
                Some(value.assume_init())
            } else {
                None
            }
        }
    }

    #[doc(alias = "gtk_level_bar_get_value")]
    #[doc(alias = "get_value")]
    pub fn value(&self) -> f64 {
        unsafe { ffi::gtk_level_bar_get_value(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_level_bar_remove_offset_value")]
    pub fn remove_offset_value(&self, name: Option<&str>) {
        unsafe {
            ffi::gtk_level_bar_remove_offset_value(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_level_bar_set_inverted")]
    #[doc(alias = "inverted")]
    pub fn set_inverted(&self, inverted: bool) {
        unsafe {
            ffi::gtk_level_bar_set_inverted(self.to_glib_none().0, inverted.into_glib());
        }
    }

    #[doc(alias = "gtk_level_bar_set_max_value")]
    #[doc(alias = "max-value")]
    pub fn set_max_value(&self, value: f64) {
        unsafe {
            ffi::gtk_level_bar_set_max_value(self.to_glib_none().0, value);
        }
    }

    #[doc(alias = "gtk_level_bar_set_min_value")]
    #[doc(alias = "min-value")]
    pub fn set_min_value(&self, value: f64) {
        unsafe {
            ffi::gtk_level_bar_set_min_value(self.to_glib_none().0, value);
        }
    }

    #[doc(alias = "gtk_level_bar_set_mode")]
    #[doc(alias = "mode")]
    pub fn set_mode(&self, mode: LevelBarMode) {
        unsafe {
            ffi::gtk_level_bar_set_mode(self.to_glib_none().0, mode.into_glib());
        }
    }

    #[doc(alias = "gtk_level_bar_set_value")]
    #[doc(alias = "value")]
    pub fn set_value(&self, value: f64) {
        unsafe {
            ffi::gtk_level_bar_set_value(self.to_glib_none().0, value);
        }
    }

    #[doc(alias = "offset-changed")]
    pub fn connect_offset_changed<F: Fn(&Self, &str) + 'static>(
        &self,
        detail: Option<&str>,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn offset_changed_trampoline<F: Fn(&LevelBar, &str) + 'static>(
            this: *mut ffi::GtkLevelBar,
            name: *mut std::ffi::c_char,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                &glib::GString::from_glib_borrow(name),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            let detailed_signal_name = detail.map(|name| format!("offset-changed::{name}\0"));
            let signal_name: &[u8] = detailed_signal_name
                .as_ref()
                .map_or(&b"offset-changed\0"[..], |n| n.as_bytes());
            connect_raw(
                self.as_ptr() as *mut _,
                signal_name.as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    offset_changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "inverted")]
    pub fn connect_inverted_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_inverted_trampoline<F: Fn(&LevelBar) + 'static>(
            this: *mut ffi::GtkLevelBar,
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
                b"notify::inverted\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_inverted_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "max-value")]
    pub fn connect_max_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_value_trampoline<F: Fn(&LevelBar) + 'static>(
            this: *mut ffi::GtkLevelBar,
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
                b"notify::max-value\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_max_value_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "min-value")]
    pub fn connect_min_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_min_value_trampoline<F: Fn(&LevelBar) + 'static>(
            this: *mut ffi::GtkLevelBar,
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
                b"notify::min-value\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_min_value_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "mode")]
    pub fn connect_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_mode_trampoline<F: Fn(&LevelBar) + 'static>(
            this: *mut ffi::GtkLevelBar,
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
                b"notify::mode\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_mode_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "value")]
    pub fn connect_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_value_trampoline<F: Fn(&LevelBar) + 'static>(
            this: *mut ffi::GtkLevelBar,
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
                b"notify::value\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_value_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for LevelBar {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`LevelBar`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct LevelBarBuilder {
    builder: glib::object::ObjectBuilder<'static, LevelBar>,
}

impl LevelBarBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn inverted(self, inverted: bool) -> Self {
        Self {
            builder: self.builder.property("inverted", inverted),
        }
    }

    pub fn max_value(self, max_value: f64) -> Self {
        Self {
            builder: self.builder.property("max-value", max_value),
        }
    }

    pub fn min_value(self, min_value: f64) -> Self {
        Self {
            builder: self.builder.property("min-value", min_value),
        }
    }

    pub fn mode(self, mode: LevelBarMode) -> Self {
        Self {
            builder: self.builder.property("mode", mode),
        }
    }

    pub fn value(self, value: f64) -> Self {
        Self {
            builder: self.builder.property("value", value),
        }
    }

    pub fn can_focus(self, can_focus: bool) -> Self {
        Self {
            builder: self.builder.property("can-focus", can_focus),
        }
    }

    pub fn can_target(self, can_target: bool) -> Self {
        Self {
            builder: self.builder.property("can-target", can_target),
        }
    }

    pub fn css_classes(self, css_classes: impl Into<glib::StrV>) -> Self {
        Self {
            builder: self.builder.property("css-classes", css_classes.into()),
        }
    }

    pub fn css_name(self, css_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("css-name", css_name.into()),
        }
    }

    pub fn cursor(self, cursor: &gdk::Cursor) -> Self {
        Self {
            builder: self.builder.property("cursor", cursor.clone()),
        }
    }

    pub fn focus_on_click(self, focus_on_click: bool) -> Self {
        Self {
            builder: self.builder.property("focus-on-click", focus_on_click),
        }
    }

    pub fn focusable(self, focusable: bool) -> Self {
        Self {
            builder: self.builder.property("focusable", focusable),
        }
    }

    pub fn halign(self, halign: Align) -> Self {
        Self {
            builder: self.builder.property("halign", halign),
        }
    }

    pub fn has_tooltip(self, has_tooltip: bool) -> Self {
        Self {
            builder: self.builder.property("has-tooltip", has_tooltip),
        }
    }

    pub fn height_request(self, height_request: i32) -> Self {
        Self {
            builder: self.builder.property("height-request", height_request),
        }
    }

    pub fn hexpand(self, hexpand: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand", hexpand),
        }
    }

    pub fn hexpand_set(self, hexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand-set", hexpand_set),
        }
    }

    pub fn layout_manager(self, layout_manager: &impl IsA<LayoutManager>) -> Self {
        Self {
            builder: self
                .builder
                .property("layout-manager", layout_manager.clone().upcast()),
        }
    }

    pub fn margin_bottom(self, margin_bottom: i32) -> Self {
        Self {
            builder: self.builder.property("margin-bottom", margin_bottom),
        }
    }

    pub fn margin_end(self, margin_end: i32) -> Self {
        Self {
            builder: self.builder.property("margin-end", margin_end),
        }
    }

    pub fn margin_start(self, margin_start: i32) -> Self {
        Self {
            builder: self.builder.property("margin-start", margin_start),
        }
    }

    pub fn margin_top(self, margin_top: i32) -> Self {
        Self {
            builder: self.builder.property("margin-top", margin_top),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn opacity(self, opacity: f64) -> Self {
        Self {
            builder: self.builder.property("opacity", opacity),
        }
    }

    pub fn overflow(self, overflow: Overflow) -> Self {
        Self {
            builder: self.builder.property("overflow", overflow),
        }
    }

    pub fn receives_default(self, receives_default: bool) -> Self {
        Self {
            builder: self.builder.property("receives-default", receives_default),
        }
    }

    pub fn sensitive(self, sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("sensitive", sensitive),
        }
    }

    pub fn tooltip_markup(self, tooltip_markup: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("tooltip-markup", tooltip_markup.into()),
        }
    }

    pub fn tooltip_text(self, tooltip_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("tooltip-text", tooltip_text.into()),
        }
    }

    pub fn valign(self, valign: Align) -> Self {
        Self {
            builder: self.builder.property("valign", valign),
        }
    }

    pub fn vexpand(self, vexpand: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand", vexpand),
        }
    }

    pub fn vexpand_set(self, vexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand-set", vexpand_set),
        }
    }

    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    pub fn width_request(self, width_request: i32) -> Self {
        Self {
            builder: self.builder.property("width-request", width_request),
        }
    }

    pub fn accessible_role(self, accessible_role: AccessibleRole) -> Self {
        Self {
            builder: self.builder.property("accessible-role", accessible_role),
        }
    }

    pub fn orientation(self, orientation: Orientation) -> Self {
        Self {
            builder: self.builder.property("orientation", orientation),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`LevelBar`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> LevelBar {
        self.builder.build()
    }
}
