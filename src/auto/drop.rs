// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ContentFormats;
use Device;
use Display;
use Drag;
use DragAction;
use Surface;
use ffi;
use glib::object::ObjectType;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct Drop(Object<ffi::GdkDrop, DropClass>);

    match fn {
        get_type => || ffi::gdk_drop_get_type(),
    }
}

impl Drop {
    pub fn finish(&self, action: DragAction) {
        unsafe {
            ffi::gdk_drop_finish(self.to_glib_none().0, action.to_glib());
        }
    }

    pub fn get_actions(&self) -> DragAction {
        unsafe {
            from_glib(ffi::gdk_drop_get_actions(self.to_glib_none().0))
        }
    }

    pub fn get_device(&self) -> Option<Device> {
        unsafe {
            from_glib_none(ffi::gdk_drop_get_device(self.to_glib_none().0))
        }
    }

    pub fn get_display(&self) -> Option<Display> {
        unsafe {
            from_glib_none(ffi::gdk_drop_get_display(self.to_glib_none().0))
        }
    }

    pub fn get_drag(&self) -> Option<Drag> {
        unsafe {
            from_glib_none(ffi::gdk_drop_get_drag(self.to_glib_none().0))
        }
    }

    pub fn get_formats(&self) -> Option<ContentFormats> {
        unsafe {
            from_glib_none(ffi::gdk_drop_get_formats(self.to_glib_none().0))
        }
    }

    pub fn get_surface(&self) -> Option<Surface> {
        unsafe {
            from_glib_none(ffi::gdk_drop_get_surface(self.to_glib_none().0))
        }
    }

    //pub fn read_async<P: FnOnce(Result<(/*Ignored*/gio::InputStream, GString), Error>) + Send + 'static>(&self, mime_types: &[&str], io_priority: /*Ignored*/glib::Priority, cancellable: /*Ignored*/Option<&gio::Cancellable>, callback: P) {
    //    unsafe { TODO: call ffi::gdk_drop_read_async() }
    //}

    //#[cfg(feature = "futures")]
    //pub fn read_async_future(&self, mime_types: &[&str], io_priority: /*Ignored*/glib::Priority) -> Box_<futures_core::Future<Item = (Self, (/*Ignored*/gio::InputStream, GString)), Error = (Self, Error)>> where Self: Sized + Clone {
        //use gio::GioFuture;
        //use fragile::Fragile;

        //let mime_types = mime_types.clone();
        //GioFuture::new(self, move |obj, send| {
        //    let cancellable = gio::Cancellable::new();
        //    let send = Fragile::new(send);
        //    let obj_clone = Fragile::new(obj.clone());
        //    obj.read_async(
        //        &mime_types,
        //        io_priority,
        //        Some(&cancellable),
        //        move |res| {
        //            let obj = obj_clone.into_inner();
        //            let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
        //            let _ = send.into_inner().send(res);
        //        },
        //    );

        //    cancellable
        //})
    //}

    //pub fn read_text_async<P: FnOnce(Result<GString, Error>) + Send + 'static>(&self, cancellable: /*Ignored*/Option<&gio::Cancellable>, callback: P) {
    //    unsafe { TODO: call ffi::gdk_drop_read_text_async() }
    //}

    //#[cfg(feature = "futures")]
    //pub fn read_text_async_future(&self) -> Box_<futures_core::Future<Item = (Self, GString), Error = (Self, Error)>> where Self: Sized + Clone {
        //use gio::GioFuture;
        //use fragile::Fragile;

        //GioFuture::new(self, move |obj, send| {
        //    let cancellable = gio::Cancellable::new();
        //    let send = Fragile::new(send);
        //    let obj_clone = Fragile::new(obj.clone());
        //    obj.read_text_async(
        //        Some(&cancellable),
        //        move |res| {
        //            let obj = obj_clone.into_inner();
        //            let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
        //            let _ = send.into_inner().send(res);
        //        },
        //    );

        //    cancellable
        //})
    //}

    //pub fn read_value_async<P: FnOnce(Result</*Ignored*/glib::Value, Error>) + Send + 'static>(&self, type_: glib::types::Type, io_priority: /*Ignored*/glib::Priority, cancellable: /*Ignored*/Option<&gio::Cancellable>, callback: P) {
    //    unsafe { TODO: call ffi::gdk_drop_read_value_async() }
    //}

    //#[cfg(feature = "futures")]
    //pub fn read_value_async_future(&self, type_: glib::types::Type, io_priority: /*Ignored*/glib::Priority) -> Box_<futures_core::Future<Item = (Self, /*Ignored*/glib::Value), Error = (Self, Error)>> where Self: Sized + Clone {
        //use gio::GioFuture;
        //use fragile::Fragile;

        //GioFuture::new(self, move |obj, send| {
        //    let cancellable = gio::Cancellable::new();
        //    let send = Fragile::new(send);
        //    let obj_clone = Fragile::new(obj.clone());
        //    obj.read_value_async(
        //        type_,
        //        io_priority,
        //        Some(&cancellable),
        //        move |res| {
        //            let obj = obj_clone.into_inner();
        //            let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
        //            let _ = send.into_inner().send(res);
        //        },
        //    );

        //    cancellable
        //})
    //}

    pub fn status(&self, actions: DragAction) {
        unsafe {
            ffi::gdk_drop_status(self.to_glib_none().0, actions.to_glib());
        }
    }

    pub fn connect_property_display_notify<F: Fn(&Drop) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::display\0".as_ptr() as *const _,
                Some(transmute(notify_display_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_display_trampoline<F: Fn(&Drop) + 'static>(this: *mut ffi::GdkDrop, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}

impl fmt::Display for Drop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Drop")
    }
}
