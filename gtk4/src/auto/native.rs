// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, Accessible, Buildable, ConstraintTarget, Widget};
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GtkNative")]
    pub struct Native(Interface<ffi::GtkNative, ffi::GtkNativeInterface>) @requires Widget, Accessible, Buildable, ConstraintTarget;

    match fn {
        type_ => || ffi::gtk_native_get_type(),
    }
}

impl Native {
    pub const NONE: Option<&'static Native> = None;

    #[doc(alias = "gtk_native_get_for_surface")]
    #[doc(alias = "get_for_surface")]
    pub fn for_surface(surface: &impl IsA<gdk::Surface>) -> Option<Native> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_native_get_for_surface(
                surface.as_ref().to_glib_none().0,
            ))
        }
    }
}

pub trait NativeExt: IsA<Native> + 'static {
    #[doc(alias = "gtk_native_get_renderer")]
    #[doc(alias = "get_renderer")]
    fn renderer(&self) -> Option<gsk::Renderer> {
        unsafe { from_glib_none(ffi::gtk_native_get_renderer(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "gtk_native_get_surface")]
    #[doc(alias = "get_surface")]
    fn surface(&self) -> Option<gdk::Surface> {
        unsafe { from_glib_none(ffi::gtk_native_get_surface(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "gtk_native_get_surface_transform")]
    #[doc(alias = "get_surface_transform")]
    fn surface_transform(&self) -> (f64, f64) {
        unsafe {
            let mut x = std::mem::MaybeUninit::uninit();
            let mut y = std::mem::MaybeUninit::uninit();
            ffi::gtk_native_get_surface_transform(
                self.as_ref().to_glib_none().0,
                x.as_mut_ptr(),
                y.as_mut_ptr(),
            );
            (x.assume_init(), y.assume_init())
        }
    }

    #[doc(alias = "gtk_native_realize")]
    fn realize(&self) {
        unsafe {
            ffi::gtk_native_realize(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_native_unrealize")]
    fn unrealize(&self) {
        unsafe {
            ffi::gtk_native_unrealize(self.as_ref().to_glib_none().0);
        }
    }
}

impl<O: IsA<Native>> NativeExt for O {}
