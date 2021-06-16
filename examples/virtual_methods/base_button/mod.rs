/// Public part of the BaseButton
mod imp;

pub use self::imp::PinnedFuture;
use gtk::glib::{
    self,
    subclass::prelude::*,
    translate::{from_glib_borrow, ToGlibPtr},
};
use gtk::{prelude::*, subclass::prelude::*};

glib::wrapper! {
    pub struct BaseButton(ObjectSubclass<imp::BaseButton>)
        @extends gtk::Widget, gtk::Button;
}

/// Public trait that implements our functions for everything that derives from BaseButton
pub trait BaseButtonExt {
    fn sync_method(&self, extra_text: Box<Option<String>>);
    fn async_method(&self) -> PinnedFuture;
}

/// We call into imp::BaseButton_$method_name for each function. These will retrieve the
/// correct class (the base class for the BaseButton or the derived class for DerivedButton)
/// and call the correct implementation of the function.
impl<O: IsA<BaseButton>> BaseButtonExt for O {
    fn sync_method(&self, extra_text: Box<Option<String>>) {
        unsafe {
            imp::base_button_sync_method(self.as_ref().to_glib_none().0, Box::into_raw(extra_text))
        }
    }

    fn async_method(&self) -> PinnedFuture {
        unsafe { imp::base_button_async_method(self.as_ref().to_glib_none().0) }
    }
}

/// The BaseButtonImpl that each derived private struct has to implement. See derived_button/imp.rs for how
/// to override functions.
pub trait BaseButtonImpl: ButtonImpl + ObjectImpl + 'static {
    fn sync_method(&self, obj: &BaseButton, extra_text: Box<Option<String>>) {
        self.parent_sync_method(obj, extra_text)
    }

    fn async_method(&self, obj: &BaseButton) -> PinnedFuture {
        self.parent_async_method(obj)
    }

    fn parent_sync_method(&self, obj: &BaseButton, extra_text: Box<Option<String>>) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut imp::BaseButtonClass;
            if let Some(ref f) = (*parent_class).sync_method {
                f(obj.to_glib_none().0, Box::into_raw(extra_text))
            } else {
                unimplemented!()
            }
        }
    }

    fn parent_async_method(&self, obj: &BaseButton) -> PinnedFuture {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut imp::BaseButtonClass;
            if let Some(ref f) = (*parent_class).async_method {
                f(obj.to_glib_none().0)
            } else {
                unimplemented!()
            }
        }
    }
}

/// Make the BaseButton subclassable
unsafe impl<T: BaseButtonImpl> IsSubclassable<T> for BaseButton {
    fn class_init(class: &mut glib::Class<Self>) {
        <gtk::Widget as IsSubclassable<T>>::class_init(class);

        let klass = class.as_mut();
        klass.sync_method = Some(sync_method_trampoline::<T>);
        klass.async_method = Some(async_method_trampoline::<T>);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <gtk::Button as IsSubclassable<T>>::instance_init(instance);
    }
}

// Virtual method default implementation trampolines
unsafe extern "C" fn sync_method_trampoline<T: ObjectSubclass>(
    this: *mut imp::BaseButtonInstance,
    extra_text: *mut Option<String>,
) where
    T: BaseButtonImpl,
{
    let instance = &*(this as *const T::Instance);
    let imp = instance.impl_();
    imp.sync_method(&from_glib_borrow(this), Box::from_raw(extra_text))
}

unsafe extern "C" fn async_method_trampoline<T: ObjectSubclass>(
    this: *mut imp::BaseButtonInstance,
) -> PinnedFuture
where
    T: BaseButtonImpl,
{
    let instance = &*(this as *const T::Instance);
    let imp = instance.impl_();
    imp.async_method(&from_glib_borrow(this))
}
