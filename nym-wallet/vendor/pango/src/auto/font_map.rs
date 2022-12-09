// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Context;
use crate::Font;
use crate::FontDescription;
use crate::FontFamily;
use crate::Fontset;
use crate::Language;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::mem;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "PangoFontMap")]
    pub struct FontMap(Object<ffi::PangoFontMap, ffi::PangoFontMapClass>);

    match fn {
        type_ => || ffi::pango_font_map_get_type(),
    }
}

impl FontMap {
    pub const NONE: Option<&'static FontMap> = None;
}

pub trait FontMapExt: 'static {
    #[doc(alias = "pango_font_map_changed")]
    fn changed(&self);

    #[doc(alias = "pango_font_map_create_context")]
    fn create_context(&self) -> Option<Context>;

    #[cfg(any(feature = "v1_46", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_46")))]
    #[doc(alias = "pango_font_map_get_family")]
    #[doc(alias = "get_family")]
    fn family(&self, name: &str) -> Option<FontFamily>;

    #[doc(alias = "pango_font_map_get_serial")]
    #[doc(alias = "get_serial")]
    fn serial(&self) -> u32;

    #[doc(alias = "pango_font_map_list_families")]
    fn list_families(&self) -> Vec<FontFamily>;

    #[doc(alias = "pango_font_map_load_font")]
    fn load_font(&self, context: &Context, desc: &FontDescription) -> Option<Font>;

    #[doc(alias = "pango_font_map_load_fontset")]
    fn load_fontset(
        &self,
        context: &Context,
        desc: &FontDescription,
        language: &Language,
    ) -> Option<Fontset>;
}

impl<O: IsA<FontMap>> FontMapExt for O {
    fn changed(&self) {
        unsafe {
            ffi::pango_font_map_changed(self.as_ref().to_glib_none().0);
        }
    }

    fn create_context(&self) -> Option<Context> {
        unsafe {
            from_glib_full(ffi::pango_font_map_create_context(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_46", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_46")))]
    fn family(&self, name: &str) -> Option<FontFamily> {
        unsafe {
            from_glib_none(ffi::pango_font_map_get_family(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    fn serial(&self) -> u32 {
        unsafe { ffi::pango_font_map_get_serial(self.as_ref().to_glib_none().0) }
    }

    fn list_families(&self) -> Vec<FontFamily> {
        unsafe {
            let mut families = ptr::null_mut();
            let mut n_families = mem::MaybeUninit::uninit();
            ffi::pango_font_map_list_families(
                self.as_ref().to_glib_none().0,
                &mut families,
                n_families.as_mut_ptr(),
            );
            FromGlibContainer::from_glib_container_num(families, n_families.assume_init() as usize)
        }
    }

    fn load_font(&self, context: &Context, desc: &FontDescription) -> Option<Font> {
        unsafe {
            from_glib_full(ffi::pango_font_map_load_font(
                self.as_ref().to_glib_none().0,
                context.to_glib_none().0,
                desc.to_glib_none().0,
            ))
        }
    }

    fn load_fontset(
        &self,
        context: &Context,
        desc: &FontDescription,
        language: &Language,
    ) -> Option<Fontset> {
        unsafe {
            from_glib_full(ffi::pango_font_map_load_fontset(
                self.as_ref().to_glib_none().0,
                context.to_glib_none().0,
                desc.to_glib_none().0,
                mut_override(language.to_glib_none().0),
            ))
        }
    }
}

impl fmt::Display for FontMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FontMap")
    }
}
