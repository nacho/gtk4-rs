// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Display;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GdkAppLaunchContext")]
    pub struct AppLaunchContext(Object<ffi::GdkAppLaunchContext>) @extends gio::AppLaunchContext;

    match fn {
        type_ => || ffi::gdk_app_launch_context_get_type(),
    }
}

impl AppLaunchContext {
    pub const NONE: Option<&'static AppLaunchContext> = None;
}

pub trait AppLaunchContextExt: 'static {
    #[doc(alias = "gdk_app_launch_context_get_display")]
    #[doc(alias = "get_display")]
    fn display(&self) -> Option<Display>;

    #[doc(alias = "gdk_app_launch_context_set_desktop")]
    fn set_desktop(&self, desktop: i32);

    #[doc(alias = "gdk_app_launch_context_set_icon")]
    fn set_icon(&self, icon: Option<&impl IsA<gio::Icon>>);

    #[doc(alias = "gdk_app_launch_context_set_icon_name")]
    fn set_icon_name(&self, icon_name: Option<&str>);

    #[doc(alias = "gdk_app_launch_context_set_timestamp")]
    fn set_timestamp(&self, timestamp: u32);
}

impl<O: IsA<AppLaunchContext>> AppLaunchContextExt for O {
    fn display(&self) -> Option<Display> {
        unsafe {
            from_glib_none(ffi::gdk_app_launch_context_get_display(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_desktop(&self, desktop: i32) {
        unsafe {
            ffi::gdk_app_launch_context_set_desktop(self.as_ref().to_glib_none().0, desktop);
        }
    }

    fn set_icon(&self, icon: Option<&impl IsA<gio::Icon>>) {
        unsafe {
            ffi::gdk_app_launch_context_set_icon(
                self.as_ref().to_glib_none().0,
                icon.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_icon_name(&self, icon_name: Option<&str>) {
        unsafe {
            ffi::gdk_app_launch_context_set_icon_name(
                self.as_ref().to_glib_none().0,
                icon_name.to_glib_none().0,
            );
        }
    }

    fn set_timestamp(&self, timestamp: u32) {
        unsafe {
            ffi::gdk_app_launch_context_set_timestamp(self.as_ref().to_glib_none().0, timestamp);
        }
    }
}

impl fmt::Display for AppLaunchContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("AppLaunchContext")
    }
}
