// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// DO NOT EDIT

use crate::{UserContentInjectedFrames, UserStyleLevel};
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct UserStyleSheet(Shared<ffi::WebKitUserStyleSheet>);

    match fn {
        ref => |ptr| ffi::webkit_user_style_sheet_ref(ptr),
        unref => |ptr| ffi::webkit_user_style_sheet_unref(ptr),
        type_ => || ffi::webkit_user_style_sheet_get_type(),
    }
}

impl UserStyleSheet {
  #[doc(alias = "webkit_user_style_sheet_new")]
  pub fn new(
    source: &str,
    injected_frames: UserContentInjectedFrames,
    level: UserStyleLevel,
    allow_list: &[&str],
    block_list: &[&str],
  ) -> UserStyleSheet {
    assert_initialized_main_thread!();
    unsafe {
      from_glib_full(ffi::webkit_user_style_sheet_new(
        source.to_glib_none().0,
        injected_frames.into_glib(),
        level.into_glib(),
        allow_list.to_glib_none().0,
        block_list.to_glib_none().0,
      ))
    }
  }

  #[cfg(any(feature = "v2_22", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_22")))]
  #[doc(alias = "webkit_user_style_sheet_new_for_world")]
  #[doc(alias = "new_for_world")]
  pub fn for_world(
    source: &str,
    injected_frames: UserContentInjectedFrames,
    level: UserStyleLevel,
    world_name: &str,
    allow_list: &[&str],
    block_list: &[&str],
  ) -> UserStyleSheet {
    assert_initialized_main_thread!();
    unsafe {
      from_glib_full(ffi::webkit_user_style_sheet_new_for_world(
        source.to_glib_none().0,
        injected_frames.into_glib(),
        level.into_glib(),
        world_name.to_glib_none().0,
        allow_list.to_glib_none().0,
        block_list.to_glib_none().0,
      ))
    }
  }
}
