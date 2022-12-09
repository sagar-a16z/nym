// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// DO NOT EDIT

#[cfg(any(feature = "v2_8", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
use glib::object::Cast;
use glib::object::IsA;
#[cfg(any(feature = "v2_8", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
use glib::signal::connect_raw;
#[cfg(any(feature = "v2_8", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v2_8", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
use glib::translate::*;
#[cfg(any(feature = "v2_8", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v2_8", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "WebKitNotification")]
    pub struct Notification(Object<ffi::WebKitNotification, ffi::WebKitNotificationClass>);

    match fn {
        type_ => || ffi::webkit_notification_get_type(),
    }
}

pub const NONE_NOTIFICATION: Option<&Notification> = None;

pub trait NotificationExt: 'static {
  #[cfg(any(feature = "v2_12", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_12")))]
  #[doc(alias = "webkit_notification_clicked")]
  fn clicked(&self);

  #[cfg(any(feature = "v2_8", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
  #[doc(alias = "webkit_notification_close")]
  fn close(&self);

  #[cfg(any(feature = "v2_8", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
  #[doc(alias = "webkit_notification_get_body")]
  #[doc(alias = "get_body")]
  fn body(&self) -> Option<glib::GString>;

  #[cfg(any(feature = "v2_8", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
  #[doc(alias = "webkit_notification_get_id")]
  #[doc(alias = "get_id")]
  fn id(&self) -> u64;

  #[cfg(any(feature = "v2_16", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
  #[doc(alias = "webkit_notification_get_tag")]
  #[doc(alias = "get_tag")]
  fn tag(&self) -> Option<glib::GString>;

  #[cfg(any(feature = "v2_8", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
  #[doc(alias = "webkit_notification_get_title")]
  #[doc(alias = "get_title")]
  fn title(&self) -> Option<glib::GString>;

  #[cfg(any(feature = "v2_12", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_12")))]
  #[doc(alias = "clicked")]
  fn connect_clicked<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

  #[cfg(any(feature = "v2_8", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
  #[doc(alias = "closed")]
  fn connect_closed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

  #[cfg(any(feature = "v2_8", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
  #[doc(alias = "body")]
  fn connect_body_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

  #[cfg(any(feature = "v2_8", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
  #[doc(alias = "id")]
  fn connect_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

  #[cfg(any(feature = "v2_16", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
  #[doc(alias = "tag")]
  fn connect_tag_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

  #[cfg(any(feature = "v2_8", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
  #[doc(alias = "title")]
  fn connect_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Notification>> NotificationExt for O {
  #[cfg(any(feature = "v2_12", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_12")))]
  fn clicked(&self) {
    unsafe {
      ffi::webkit_notification_clicked(self.as_ref().to_glib_none().0);
    }
  }

  #[cfg(any(feature = "v2_8", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
  fn close(&self) {
    unsafe {
      ffi::webkit_notification_close(self.as_ref().to_glib_none().0);
    }
  }

  #[cfg(any(feature = "v2_8", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
  fn body(&self) -> Option<glib::GString> {
    unsafe {
      from_glib_none(ffi::webkit_notification_get_body(
        self.as_ref().to_glib_none().0,
      ))
    }
  }

  #[cfg(any(feature = "v2_8", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
  fn id(&self) -> u64 {
    unsafe { ffi::webkit_notification_get_id(self.as_ref().to_glib_none().0) }
  }

  #[cfg(any(feature = "v2_16", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
  fn tag(&self) -> Option<glib::GString> {
    unsafe {
      from_glib_none(ffi::webkit_notification_get_tag(
        self.as_ref().to_glib_none().0,
      ))
    }
  }

  #[cfg(any(feature = "v2_8", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
  fn title(&self) -> Option<glib::GString> {
    unsafe {
      from_glib_none(ffi::webkit_notification_get_title(
        self.as_ref().to_glib_none().0,
      ))
    }
  }

  #[cfg(any(feature = "v2_12", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_12")))]
  fn connect_clicked<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
    unsafe extern "C" fn clicked_trampoline<P: IsA<Notification>, F: Fn(&P) + 'static>(
      this: *mut ffi::WebKitNotification,
      f: glib::ffi::gpointer,
    ) {
      let f: &F = &*(f as *const F);
      f(Notification::from_glib_borrow(this).unsafe_cast_ref())
    }
    unsafe {
      let f: Box_<F> = Box_::new(f);
      connect_raw(
        self.as_ptr() as *mut _,
        b"clicked\0".as_ptr() as *const _,
        Some(transmute::<_, unsafe extern "C" fn()>(
          clicked_trampoline::<Self, F> as *const (),
        )),
        Box_::into_raw(f),
      )
    }
  }

  #[cfg(any(feature = "v2_8", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
  fn connect_closed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
    unsafe extern "C" fn closed_trampoline<P: IsA<Notification>, F: Fn(&P) + 'static>(
      this: *mut ffi::WebKitNotification,
      f: glib::ffi::gpointer,
    ) {
      let f: &F = &*(f as *const F);
      f(Notification::from_glib_borrow(this).unsafe_cast_ref())
    }
    unsafe {
      let f: Box_<F> = Box_::new(f);
      connect_raw(
        self.as_ptr() as *mut _,
        b"closed\0".as_ptr() as *const _,
        Some(transmute::<_, unsafe extern "C" fn()>(
          closed_trampoline::<Self, F> as *const (),
        )),
        Box_::into_raw(f),
      )
    }
  }

  #[cfg(any(feature = "v2_8", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
  fn connect_body_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
    unsafe extern "C" fn notify_body_trampoline<P: IsA<Notification>, F: Fn(&P) + 'static>(
      this: *mut ffi::WebKitNotification,
      _param_spec: glib::ffi::gpointer,
      f: glib::ffi::gpointer,
    ) {
      let f: &F = &*(f as *const F);
      f(Notification::from_glib_borrow(this).unsafe_cast_ref())
    }
    unsafe {
      let f: Box_<F> = Box_::new(f);
      connect_raw(
        self.as_ptr() as *mut _,
        b"notify::body\0".as_ptr() as *const _,
        Some(transmute::<_, unsafe extern "C" fn()>(
          notify_body_trampoline::<Self, F> as *const (),
        )),
        Box_::into_raw(f),
      )
    }
  }

  #[cfg(any(feature = "v2_8", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
  fn connect_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
    unsafe extern "C" fn notify_id_trampoline<P: IsA<Notification>, F: Fn(&P) + 'static>(
      this: *mut ffi::WebKitNotification,
      _param_spec: glib::ffi::gpointer,
      f: glib::ffi::gpointer,
    ) {
      let f: &F = &*(f as *const F);
      f(Notification::from_glib_borrow(this).unsafe_cast_ref())
    }
    unsafe {
      let f: Box_<F> = Box_::new(f);
      connect_raw(
        self.as_ptr() as *mut _,
        b"notify::id\0".as_ptr() as *const _,
        Some(transmute::<_, unsafe extern "C" fn()>(
          notify_id_trampoline::<Self, F> as *const (),
        )),
        Box_::into_raw(f),
      )
    }
  }

  #[cfg(any(feature = "v2_16", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
  fn connect_tag_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
    unsafe extern "C" fn notify_tag_trampoline<P: IsA<Notification>, F: Fn(&P) + 'static>(
      this: *mut ffi::WebKitNotification,
      _param_spec: glib::ffi::gpointer,
      f: glib::ffi::gpointer,
    ) {
      let f: &F = &*(f as *const F);
      f(Notification::from_glib_borrow(this).unsafe_cast_ref())
    }
    unsafe {
      let f: Box_<F> = Box_::new(f);
      connect_raw(
        self.as_ptr() as *mut _,
        b"notify::tag\0".as_ptr() as *const _,
        Some(transmute::<_, unsafe extern "C" fn()>(
          notify_tag_trampoline::<Self, F> as *const (),
        )),
        Box_::into_raw(f),
      )
    }
  }

  #[cfg(any(feature = "v2_8", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
  fn connect_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
    unsafe extern "C" fn notify_title_trampoline<P: IsA<Notification>, F: Fn(&P) + 'static>(
      this: *mut ffi::WebKitNotification,
      _param_spec: glib::ffi::gpointer,
      f: glib::ffi::gpointer,
    ) {
      let f: &F = &*(f as *const F);
      f(Notification::from_glib_borrow(this).unsafe_cast_ref())
    }
    unsafe {
      let f: Box_<F> = Box_::new(f);
      connect_raw(
        self.as_ptr() as *mut _,
        b"notify::title\0".as_ptr() as *const _,
        Some(transmute::<_, unsafe extern "C" fn()>(
          notify_title_trampoline::<Self, F> as *const (),
        )),
        Box_::into_raw(f),
      )
    }
  }
}

impl fmt::Display for Notification {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str("Notification")
  }
}
