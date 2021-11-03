// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::IMContext;
use crate::InputHints;
use crate::InputPurpose;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkIMMulticontext")]
    pub struct IMMulticontext(Object<ffi::GtkIMMulticontext, ffi::GtkIMMulticontextClass>) @extends IMContext;

    match fn {
        type_ => || ffi::gtk_im_multicontext_get_type(),
    }
}

impl IMMulticontext {
    #[doc(alias = "gtk_im_multicontext_new")]
    pub fn new() -> IMMulticontext {
        assert_initialized_main_thread!();
        unsafe { IMContext::from_glib_full(ffi::gtk_im_multicontext_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`IMMulticontext`] objects.
    ///
    /// This method returns an instance of [`IMMulticontextBuilder`] which can be used to create [`IMMulticontext`] objects.
    pub fn builder() -> IMMulticontextBuilder {
        IMMulticontextBuilder::default()
    }
}

impl Default for IMMulticontext {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`IMMulticontext`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
pub struct IMMulticontextBuilder {
    input_hints: Option<InputHints>,
    input_purpose: Option<InputPurpose>,
}

impl IMMulticontextBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`IMMulticontextBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`IMMulticontext`].
    pub fn build(self) -> IMMulticontext {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref input_hints) = self.input_hints {
            properties.push(("input-hints", input_hints));
        }
        if let Some(ref input_purpose) = self.input_purpose {
            properties.push(("input-purpose", input_purpose));
        }
        glib::Object::new::<IMMulticontext>(&properties)
            .expect("Failed to create an instance of IMMulticontext")
    }

    pub fn input_hints(mut self, input_hints: InputHints) -> Self {
        self.input_hints = Some(input_hints);
        self
    }

    pub fn input_purpose(mut self, input_purpose: InputPurpose) -> Self {
        self.input_purpose = Some(input_purpose);
        self
    }
}

impl IMMulticontext {
    pub const NONE: Option<&'static IMMulticontext> = None;
}

pub trait IMMulticontextExt: 'static {
    #[doc(alias = "gtk_im_multicontext_get_context_id")]
    #[doc(alias = "get_context_id")]
    fn context_id(&self) -> Option<glib::GString>;

    #[doc(alias = "gtk_im_multicontext_set_context_id")]
    fn set_context_id(&self, context_id: Option<&str>);
}

impl<O: IsA<IMMulticontext>> IMMulticontextExt for O {
    fn context_id(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_im_multicontext_get_context_id(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_context_id(&self, context_id: Option<&str>) {
        unsafe {
            ffi::gtk_im_multicontext_set_context_id(
                self.as_ref().to_glib_none().0,
                context_id.to_glib_none().0,
            );
        }
    }
}

impl fmt::Display for IMMulticontext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("IMMulticontext")
    }
}
