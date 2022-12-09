// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::TreeModel;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GtkTreeSortable")]
    pub struct TreeSortable(Interface<ffi::GtkTreeSortable, ffi::GtkTreeSortableIface>) @requires TreeModel;

    match fn {
        type_ => || ffi::gtk_tree_sortable_get_type(),
    }
}

impl TreeSortable {
    pub const NONE: Option<&'static TreeSortable> = None;
}

pub trait TreeSortableExt: 'static {
    #[doc(alias = "gtk_tree_sortable_has_default_sort_func")]
    fn has_default_sort_func(&self) -> bool;

    #[doc(alias = "gtk_tree_sortable_sort_column_changed")]
    fn sort_column_changed(&self);

    #[doc(alias = "sort-column-changed")]
    fn connect_sort_column_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<TreeSortable>> TreeSortableExt for O {
    fn has_default_sort_func(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_sortable_has_default_sort_func(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn sort_column_changed(&self) {
        unsafe {
            ffi::gtk_tree_sortable_sort_column_changed(self.as_ref().to_glib_none().0);
        }
    }

    fn connect_sort_column_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn sort_column_changed_trampoline<
            P: IsA<TreeSortable>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkTreeSortable,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TreeSortable::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"sort-column-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    sort_column_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for TreeSortable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TreeSortable")
    }
}
