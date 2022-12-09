// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Icon;
use crate::NotificationPriority;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GNotification")]
    pub struct Notification(Object<ffi::GNotification>);

    match fn {
        type_ => || ffi::g_notification_get_type(),
    }
}

impl Notification {
    #[doc(alias = "g_notification_new")]
    pub fn new(title: &str) -> Notification {
        unsafe { from_glib_full(ffi::g_notification_new(title.to_glib_none().0)) }
    }

    #[doc(alias = "g_notification_add_button")]
    pub fn add_button(&self, label: &str, detailed_action: &str) {
        unsafe {
            ffi::g_notification_add_button(
                self.to_glib_none().0,
                label.to_glib_none().0,
                detailed_action.to_glib_none().0,
            );
        }
    }

    //#[doc(alias = "g_notification_add_button_with_target")]
    //pub fn add_button_with_target(&self, label: &str, action: &str, target_format: Option<&str>, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi:g_notification_add_button_with_target() }
    //}

    #[doc(alias = "g_notification_add_button_with_target_value")]
    pub fn add_button_with_target_value(
        &self,
        label: &str,
        action: &str,
        target: Option<&glib::Variant>,
    ) {
        unsafe {
            ffi::g_notification_add_button_with_target_value(
                self.to_glib_none().0,
                label.to_glib_none().0,
                action.to_glib_none().0,
                target.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_notification_set_body")]
    pub fn set_body(&self, body: Option<&str>) {
        unsafe {
            ffi::g_notification_set_body(self.to_glib_none().0, body.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_70", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_70")))]
    #[doc(alias = "g_notification_set_category")]
    pub fn set_category(&self, category: Option<&str>) {
        unsafe {
            ffi::g_notification_set_category(self.to_glib_none().0, category.to_glib_none().0);
        }
    }

    #[doc(alias = "g_notification_set_default_action")]
    pub fn set_default_action(&self, detailed_action: &str) {
        unsafe {
            ffi::g_notification_set_default_action(
                self.to_glib_none().0,
                detailed_action.to_glib_none().0,
            );
        }
    }

    //#[doc(alias = "g_notification_set_default_action_and_target")]
    //pub fn set_default_action_and_target(&self, action: &str, target_format: Option<&str>, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi:g_notification_set_default_action_and_target() }
    //}

    #[doc(alias = "g_notification_set_default_action_and_target_value")]
    pub fn set_default_action_and_target_value(
        &self,
        action: &str,
        target: Option<&glib::Variant>,
    ) {
        unsafe {
            ffi::g_notification_set_default_action_and_target_value(
                self.to_glib_none().0,
                action.to_glib_none().0,
                target.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_notification_set_icon")]
    pub fn set_icon(&self, icon: &impl IsA<Icon>) {
        unsafe {
            ffi::g_notification_set_icon(self.to_glib_none().0, icon.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "g_notification_set_priority")]
    pub fn set_priority(&self, priority: NotificationPriority) {
        unsafe {
            ffi::g_notification_set_priority(self.to_glib_none().0, priority.into_glib());
        }
    }

    #[doc(alias = "g_notification_set_title")]
    pub fn set_title(&self, title: &str) {
        unsafe {
            ffi::g_notification_set_title(self.to_glib_none().0, title.to_glib_none().0);
        }
    }
}

impl fmt::Display for Notification {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Notification")
    }
}
