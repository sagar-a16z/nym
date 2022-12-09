// Take a look at the license at the top of the repository in the LICENSE file.

use crate::translate::*;
use crate::types::{StaticType, Type};
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::ffi::{CStr, CString, OsStr};
use std::fmt;
use std::hash;
use std::mem;
use std::ops::Deref;
use std::os::raw::{c_char, c_void};
use std::ptr;
use std::slice;
use std::string::String;

pub struct GString(Inner);
enum Inner {
    Native(Option<CString>),
    Foreign {
        ptr: ptr::NonNull<c_char>,
        len: usize,
    },
}

unsafe impl Send for GString {}
unsafe impl Sync for GString {}

impl GString {
    // rustdoc-stripper-ignore-next
    /// Return the `GString` as string slice.
    pub fn as_str(&self) -> &str {
        unsafe {
            let (ptr, len) = match self.0 {
                Inner::Native(ref cstr) => {
                    let cstr = cstr.as_ref().unwrap();
                    (cstr.as_ptr() as *const u8, cstr.to_bytes().len())
                }
                Inner::Foreign { ptr, len } => (ptr.as_ptr() as *const u8, len),
            };
            if len == 0 {
                ""
            } else {
                let slice = slice::from_raw_parts(ptr, len);
                std::str::from_utf8_unchecked(slice)
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Return the underlying pointer of the `GString`.
    pub fn as_ptr(&self) -> *const c_char {
        match self.0 {
            Inner::Native(ref cstr) => cstr.as_ref().unwrap().as_ptr() as *const _,
            Inner::Foreign { ptr, .. } => ptr.as_ptr(),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Transform into a `NUL`-terminated raw C string pointer.
    pub fn into_raw(self) -> *mut c_char {
        match self.0 {
            Inner::Native(ref cstr) => unsafe {
                let cstr = cstr.as_ref().unwrap();

                let ptr = cstr.as_ptr();
                let len = cstr.to_bytes().len();

                let copy = ffi::g_malloc(len + 1) as *mut c_char;
                ptr::copy_nonoverlapping(ptr as *const c_char, copy, len + 1);
                ptr::write(copy.add(len), 0);

                copy
            },
            Inner::Foreign { ptr, .. } => {
                let _s = mem::ManuallyDrop::new(self);
                ptr.as_ptr()
            }
        }
    }
}

impl Clone for GString {
    fn clone(&self) -> GString {
        self.as_str().into()
    }
}

impl fmt::Debug for GString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <&str as fmt::Debug>::fmt(&self.as_str(), f)
    }
}

impl Drop for GString {
    fn drop(&mut self) {
        if let Inner::Foreign { ptr, .. } = self.0 {
            unsafe {
                ffi::g_free(ptr.as_ptr() as *mut _);
            }
        }
    }
}

impl fmt::Display for GString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl hash::Hash for GString {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state)
    }
}

impl Borrow<str> for GString {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl Ord for GString {
    fn cmp(&self, other: &GString) -> Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl PartialOrd for GString {
    fn partial_cmp(&self, other: &GString) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for GString {
    fn eq(&self, other: &GString) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<GString> for String {
    fn eq(&self, other: &GString) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<str> for GString {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl<'a> PartialEq<&'a str> for GString {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_str() == *other
    }
}

impl<'a> PartialEq<GString> for &'a str {
    fn eq(&self, other: &GString) -> bool {
        *self == other.as_str()
    }
}

impl PartialEq<String> for GString {
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<GString> for str {
    fn eq(&self, other: &GString) -> bool {
        self == other.as_str()
    }
}

impl PartialOrd<GString> for String {
    fn partial_cmp(&self, other: &GString) -> Option<Ordering> {
        Some(self.cmp(&String::from(other.as_str())))
    }
}

impl PartialOrd<String> for GString {
    fn partial_cmp(&self, other: &String) -> Option<Ordering> {
        Some(self.as_str().cmp(other.as_str()))
    }
}

impl PartialOrd<GString> for str {
    fn partial_cmp(&self, other: &GString) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd<str> for GString {
    fn partial_cmp(&self, other: &str) -> Option<Ordering> {
        Some(self.as_str().cmp(other))
    }
}

impl Eq for GString {}

impl AsRef<str> for GString {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<OsStr> for GString {
    fn as_ref(&self) -> &OsStr {
        OsStr::new(self.as_str())
    }
}

impl Deref for GString {
    type Target = str;

    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl From<GString> for String {
    #[inline]
    fn from(mut s: GString) -> Self {
        match s.0 {
            Inner::Foreign { len, .. } if len == 0 => String::new(),
            Inner::Foreign { ptr, len } => unsafe {
                // Creates a copy
                let slice = slice::from_raw_parts(ptr.as_ptr() as *const u8, len);
                std::str::from_utf8_unchecked(slice).into()
            },
            Inner::Native(ref mut cstr) => {
                // Moves the underlying string
                cstr.take().unwrap().into_string().unwrap()
            }
        }
    }
}

impl From<GString> for Box<str> {
    #[inline]
    fn from(s: GString) -> Self {
        // Potentially creates a copy
        let st: String = s.into();
        st.into_boxed_str()
    }
}

impl From<String> for GString {
    #[inline]
    fn from(s: String) -> Self {
        // Moves the content of the String
        unsafe {
            // No check for valid UTF-8 here
            let cstr = CString::from_vec_unchecked(s.into_bytes());
            GString(Inner::Native(Some(cstr)))
        }
    }
}

impl From<Box<str>> for GString {
    #[inline]
    fn from(s: Box<str>) -> Self {
        // Moves the content of the String
        s.into_string().into()
    }
}

impl<'a> From<&'a str> for GString {
    #[inline]
    fn from(s: &'a str) -> Self {
        // Allocates with the GLib allocator
        unsafe {
            // No check for valid UTF-8 here
            let copy = ffi::g_malloc(s.len() + 1) as *mut c_char;
            ptr::copy_nonoverlapping(s.as_ptr() as *const c_char, copy, s.len() + 1);
            ptr::write(copy.add(s.len()), 0);

            GString(Inner::Foreign {
                ptr: ptr::NonNull::new_unchecked(copy),
                len: s.len(),
            })
        }
    }
}

impl From<Vec<u8>> for GString {
    #[inline]
    fn from(s: Vec<u8>) -> Self {
        // Moves the content of the Vec
        // Also check if it's valid UTF-8
        let cstring = CString::new(s).expect("CString::new failed");
        cstring.into()
    }
}

impl From<CString> for GString {
    #[inline]
    fn from(s: CString) -> Self {
        // Moves the content of the CString
        // Also check if it's valid UTF-8
        assert!(s.to_str().is_ok());
        Self(Inner::Native(Some(s)))
    }
}

impl<'a> From<&'a CStr> for GString {
    #[inline]
    fn from(c: &'a CStr) -> Self {
        // Creates a copy with the GLib allocator
        // Also check if it's valid UTF-8
        c.to_str().unwrap().into()
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*mut u8> for GString {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut u8) -> Self {
        assert!(!ptr.is_null());

        // Check for valid UTF-8 here
        let cstr = CStr::from_ptr(ptr as *const _);
        assert!(cstr.to_str().is_ok());
        GString(Inner::Foreign {
            ptr: ptr::NonNull::new_unchecked(ptr as *mut _),
            len: cstr.to_bytes().len(),
        })
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*mut i8> for GString {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut i8) -> Self {
        from_glib_full(ptr as *mut u8)
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*const u8> for GString {
    #[inline]
    unsafe fn from_glib_full(ptr: *const u8) -> Self {
        from_glib_full(ptr as *mut u8)
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*const i8> for GString {
    #[inline]
    unsafe fn from_glib_full(ptr: *const i8) -> Self {
        from_glib_full(ptr as *mut u8)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const u8> for GString {
    #[inline]
    unsafe fn from_glib_none(ptr: *const u8) -> Self {
        assert!(!ptr.is_null());
        let cstr = CStr::from_ptr(ptr as *const _);
        // Also check if it's valid UTF-8
        cstr.to_str().unwrap().into()
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const i8> for GString {
    #[inline]
    unsafe fn from_glib_none(ptr: *const i8) -> Self {
        from_glib_none(ptr as *const u8)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut u8> for GString {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut u8) -> Self {
        from_glib_none(ptr as *const u8)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut i8> for GString {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut i8) -> Self {
        from_glib_none(ptr as *const u8)
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*const u8> for GString {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *const u8) -> Borrowed<Self> {
        assert!(!ptr.is_null());

        // Check for valid UTF-8 here
        let cstr = CStr::from_ptr(ptr as *const _);
        assert!(cstr.to_str().is_ok());
        Borrowed::new(GString(Inner::Foreign {
            ptr: ptr::NonNull::new_unchecked(ptr as *mut _),
            len: cstr.to_bytes().len(),
        }))
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*const i8> for GString {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *const i8) -> Borrowed<Self> {
        from_glib_borrow(ptr as *const u8)
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*mut u8> for GString {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut u8) -> Borrowed<Self> {
        from_glib_borrow(ptr as *const u8)
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*mut i8> for GString {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut i8) -> Borrowed<Self> {
        from_glib_borrow(ptr as *const u8)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const u8> for GString {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const u8, Self> {
        Stash(self.as_ptr() as *const u8, self)
    }

    #[inline]
    fn to_glib_full(&self) -> *const u8 {
        self.clone().into_raw() as *const u8
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const i8> for GString {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const i8, Self> {
        Stash(self.as_ptr() as *const i8, self)
    }

    #[inline]
    fn to_glib_full(&self) -> *const i8 {
        self.clone().into_raw() as *const i8
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *mut u8> for GString {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *mut u8, Self> {
        Stash(self.as_ptr() as *mut u8, self)
    }

    #[inline]
    fn to_glib_full(&self) -> *mut u8 {
        self.clone().into_raw() as *mut u8
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *mut i8> for GString {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *mut i8, Self> {
        Stash(self.as_ptr() as *mut i8, self)
    }

    #[inline]
    fn to_glib_full(&self) -> *mut i8 {
        self.clone().into_raw() as *mut i8
    }
}

#[doc(hidden)]
impl<'a> FromGlibContainer<*const c_char, *const i8> for GString {
    unsafe fn from_glib_none_num(ptr: *const i8, num: usize) -> Self {
        if num == 0 || ptr.is_null() {
            return Self::from("");
        }
        let slice = slice::from_raw_parts(ptr as *const u8, num);
        // Also check if it's valid UTF-8
        std::str::from_utf8(slice).unwrap().into()
    }

    unsafe fn from_glib_container_num(ptr: *const i8, num: usize) -> Self {
        if num == 0 || ptr.is_null() {
            return Self::from("");
        }

        // Check if it's valid UTF-8
        let slice = slice::from_raw_parts(ptr as *const u8, num);
        std::str::from_utf8(slice).unwrap();

        GString(Inner::Foreign {
            ptr: ptr::NonNull::new_unchecked(ptr as *mut _),
            len: num,
        })
    }

    unsafe fn from_glib_full_num(ptr: *const i8, num: usize) -> Self {
        if num == 0 || ptr.is_null() {
            return Self::from("");
        }

        // Check if it's valid UTF-8
        let slice = slice::from_raw_parts(ptr as *const u8, num);
        std::str::from_utf8(slice).unwrap();

        GString(Inner::Foreign {
            ptr: ptr::NonNull::new_unchecked(ptr as *mut _),
            len: num,
        })
    }
}

#[doc(hidden)]
impl<'a> FromGlibContainer<*const c_char, *mut i8> for GString {
    unsafe fn from_glib_none_num(ptr: *mut i8, num: usize) -> Self {
        FromGlibContainer::from_glib_none_num(ptr as *const i8, num)
    }

    unsafe fn from_glib_container_num(ptr: *mut i8, num: usize) -> Self {
        FromGlibContainer::from_glib_container_num(ptr as *const i8, num)
    }

    unsafe fn from_glib_full_num(ptr: *mut i8, num: usize) -> Self {
        FromGlibContainer::from_glib_full_num(ptr as *const i8, num)
    }
}

#[doc(hidden)]
impl<'a> FromGlibContainer<*const c_char, *const u8> for GString {
    unsafe fn from_glib_none_num(ptr: *const u8, num: usize) -> Self {
        FromGlibContainer::from_glib_none_num(ptr as *const i8, num)
    }

    unsafe fn from_glib_container_num(ptr: *const u8, num: usize) -> Self {
        FromGlibContainer::from_glib_container_num(ptr as *const i8, num)
    }

    unsafe fn from_glib_full_num(ptr: *const u8, num: usize) -> Self {
        FromGlibContainer::from_glib_full_num(ptr as *const i8, num)
    }
}

#[doc(hidden)]
impl<'a> FromGlibContainer<*const c_char, *mut u8> for GString {
    unsafe fn from_glib_none_num(ptr: *mut u8, num: usize) -> Self {
        FromGlibContainer::from_glib_none_num(ptr as *const i8, num)
    }

    unsafe fn from_glib_container_num(ptr: *mut u8, num: usize) -> Self {
        FromGlibContainer::from_glib_container_num(ptr as *const i8, num)
    }

    unsafe fn from_glib_full_num(ptr: *mut u8, num: usize) -> Self {
        FromGlibContainer::from_glib_full_num(ptr as *const i8, num)
    }
}

impl GlibPtrDefault for GString {
    type GlibType = *const c_char;
}

impl StaticType for GString {
    fn static_type() -> Type {
        String::static_type()
    }
}

impl crate::value::ValueType for GString {
    type Type = String;
}

impl crate::value::ValueTypeOptional for GString {}

unsafe impl<'a> crate::value::FromValue<'a> for GString {
    type Checker = crate::value::GenericValueTypeOrNoneChecker<Self>;

    unsafe fn from_value(value: &'a crate::Value) -> Self {
        Self::from(<&str>::from_value(value))
    }
}

impl crate::value::ToValue for GString {
    fn to_value(&self) -> crate::Value {
        <&str>::to_value(&self.as_str())
    }

    fn value_type(&self) -> Type {
        String::static_type()
    }
}

impl crate::value::ToValueOptional for GString {
    fn to_value_optional(s: Option<&Self>) -> crate::Value {
        <str>::to_value_optional(s.as_ref().map(|s| s.as_str()))
    }
}

impl StaticType for Vec<GString> {
    fn static_type() -> Type {
        <Vec<String>>::static_type()
    }
}

impl crate::value::ValueType for Vec<GString> {
    type Type = Vec<GString>;
}

unsafe impl<'a> crate::value::FromValue<'a> for Vec<GString> {
    type Checker = crate::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a crate::value::Value) -> Self {
        let ptr = gobject_ffi::g_value_get_boxed(value.to_glib_none().0) as *const *const c_char;
        FromGlibPtrContainer::from_glib_none(ptr)
    }
}

impl crate::value::ToValue for Vec<GString> {
    fn to_value(&self) -> crate::value::Value {
        unsafe {
            let mut value = crate::value::Value::for_value_type::<Self>();
            let ptr: *mut *mut c_char = self.to_glib_full();
            gobject_ffi::g_value_take_boxed(value.to_glib_none_mut().0, ptr as *const c_void);
            value
        }
    }

    fn value_type(&self) -> Type {
        <Vec<GString>>::static_type()
    }
}

impl_from_glib_container_as_vec_string!(GString, *const c_char);
impl_from_glib_container_as_vec_string!(GString, *mut c_char);

#[cfg(test)]
#[allow(clippy::blacklisted_name)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_gstring() {
        let data = CString::new("foo").unwrap();
        let ptr = data.as_ptr();

        unsafe {
            let ptr_copy = ffi::g_strdup(ptr);
            let gstring = GString::from_glib_full(ptr_copy);
            assert_eq!(gstring.as_str(), "foo");
            let foo: Box<str> = gstring.into();
            assert_eq!(foo.as_ref(), "foo");
        }
    }

    #[test]
    fn test_owned_glib_string() {
        let data = CString::new("foo").unwrap();
        let ptr = data.as_ptr();
        unsafe {
            let ptr_copy = ffi::g_strdup(ptr);
            let gstr = GString::from_glib_full(ptr_copy);
            assert_eq!(gstr, "foo");
        }
    }

    #[test]
    fn test_gstring_from_str() {
        let gstring: GString = "foo".into();
        assert_eq!(gstring.as_str(), "foo");
        let foo: Box<str> = gstring.into();
        assert_eq!(foo.as_ref(), "foo");
    }

    #[test]
    fn test_string_from_gstring() {
        let gstring = GString::from("foo");
        assert_eq!(gstring.as_str(), "foo");
        let s = String::from(gstring);
        assert_eq!(s, "foo");
    }

    #[test]
    fn test_gstring_from_cstring() {
        let cstr = CString::new("foo").unwrap();
        let gstring = GString::from(cstr);
        assert_eq!(gstring.as_str(), "foo");
        let foo: Box<str> = gstring.into();
        assert_eq!(foo.as_ref(), "foo");
    }

    #[test]
    fn test_string_from_gstring_from_cstring() {
        let cstr = CString::new("foo").unwrap();
        let gstring = GString::from(cstr);
        assert_eq!(gstring.as_str(), "foo");
        let s = String::from(gstring);
        assert_eq!(s, "foo");
    }

    #[test]
    fn test_vec_u8_to_gstring() {
        let v: &[u8] = b"foo";
        let s: GString = Vec::from(v).into();
        assert_eq!(s.as_str(), "foo");
    }

    #[test]
    fn test_from_glib_container() {
        unsafe {
            let test_a: GString = FromGlibContainer::from_glib_container_num(
                ffi::g_strdup("hello_world".as_ptr() as *const _),
                5,
            );
            assert_eq!("hello", test_a.as_str());

            let test_b: GString = FromGlibContainer::from_glib_none_num("hello_world".as_ptr(), 5);
            assert_eq!("hello", test_b.as_str());

            let test_c: GString =
                FromGlibContainer::from_glib_none_num(std::ptr::null::<std::os::raw::c_char>(), 0);
            assert_eq!("", test_c.as_str());

            let test_d: GString = FromGlibContainer::from_glib_none_num("".as_ptr(), 0);
            assert_eq!("", test_d.as_str());

            let test_e: GString =
                FromGlibContainer::from_glib_container_num(ffi::g_strdup(std::ptr::null()), 0);
            assert_eq!("", test_e.as_str());
        }
    }

    #[test]
    fn test_hashmap() {
        use std::collections::HashMap;

        let gstring = GString::from("foo");
        assert_eq!(gstring.as_str(), "foo");
        let mut h: HashMap<GString, i32> = HashMap::new();
        h.insert(gstring, 42);
        let gstring: GString = "foo".into();
        assert!(h.contains_key(&gstring));
    }
}
