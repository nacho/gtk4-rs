// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Ordering;
use crate::SorterChange;
use crate::SorterOrder;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GtkSorter")]
    pub struct Sorter(Object<ffi::GtkSorter, ffi::GtkSorterClass>);

    match fn {
        type_ => || ffi::gtk_sorter_get_type(),
    }
}

impl Sorter {
    pub const NONE: Option<&'static Sorter> = None;
}

pub trait SorterExt: 'static {
    #[doc(alias = "gtk_sorter_changed")]
    fn changed(&self, change: SorterChange);

    #[doc(alias = "gtk_sorter_compare")]
    fn compare(&self, item1: &impl IsA<glib::Object>, item2: &impl IsA<glib::Object>) -> Ordering;

    #[doc(alias = "gtk_sorter_get_order")]
    #[doc(alias = "get_order")]
    fn order(&self) -> SorterOrder;

    #[doc(alias = "changed")]
    fn connect_changed<F: Fn(&Self, SorterChange) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Sorter>> SorterExt for O {
    fn changed(&self, change: SorterChange) {
        unsafe {
            ffi::gtk_sorter_changed(self.as_ref().to_glib_none().0, change.into_glib());
        }
    }

    fn compare(&self, item1: &impl IsA<glib::Object>, item2: &impl IsA<glib::Object>) -> Ordering {
        unsafe {
            from_glib(ffi::gtk_sorter_compare(
                self.as_ref().to_glib_none().0,
                item1.as_ref().to_glib_none().0,
                item2.as_ref().to_glib_none().0,
            ))
        }
    }

    fn order(&self) -> SorterOrder {
        unsafe { from_glib(ffi::gtk_sorter_get_order(self.as_ref().to_glib_none().0)) }
    }

    fn connect_changed<F: Fn(&Self, SorterChange) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn changed_trampoline<
            P: IsA<Sorter>,
            F: Fn(&P, SorterChange) + 'static,
        >(
            this: *mut ffi::GtkSorter,
            change: ffi::GtkSorterChange,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Sorter::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(change),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Sorter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Sorter")
    }
}
