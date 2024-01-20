// Generated by gir (https://github.com/gtk-rs/gir @ 1c7a6b57a5fc)
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 21b29d0e0c1a)
// from packagekit-gir-files
// DO NOT EDIT

use crate::{Source};
#[cfg(feature = "v0_5_4")]
#[cfg_attr(docsrs, doc(cfg(feature = "v0_5_4")))]
use crate::{RestartEnum};
use glib::{prelude::*};
#[cfg(feature = "v0_5_4")]
#[cfg_attr(docsrs, doc(cfg(feature = "v0_5_4")))]
use glib::{signal::{connect_raw, SignalHandlerId},translate::*};
#[cfg(feature = "v0_5_4")]
#[cfg_attr(docsrs, doc(cfg(feature = "v0_5_4")))]
use std::{boxed::Box as Box_};

glib::wrapper! {
    #[doc(alias = "PkRequireRestart")]
    pub struct RequireRestart(Object<ffi::PkRequireRestart, ffi::PkRequireRestartClass>) @extends Source;

    match fn {
        type_ => || ffi::pk_require_restart_get_type(),
    }
}

impl RequireRestart {
        pub const NONE: Option<&'static RequireRestart> = None;
    

    ///
    /// # Returns
    ///
    /// a new [`RequireRestart`][crate::RequireRestart] object.
    #[cfg(feature = "v0_5_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v0_5_4")))]
    #[doc(alias = "pk_require_restart_new")]
    pub fn new() -> RequireRestart {
        unsafe {
            from_glib_full(ffi::pk_require_restart_new())
        }
    }
}

#[cfg(feature = "v0_5_4")]
#[cfg_attr(docsrs, doc(cfg(feature = "v0_5_4")))]
impl Default for RequireRestart {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::RequireRestart>> Sealed for T {}
}

/// Trait containing all [`struct@RequireRestart`] methods.
///
/// # Implementors
///
/// [`RequireRestart`][struct@crate::RequireRestart]
pub trait RequireRestartExt: IsA<RequireRestart> + sealed::Sealed + 'static {
    #[cfg(feature = "v0_5_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v0_5_4")))]
    #[doc(alias = "package-id")]
    fn package_id(&self) -> Option<glib::GString> {
        ObjectExt::property(self.as_ref(), "package-id")
    }

    #[cfg(feature = "v0_5_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v0_5_4")))]
    #[doc(alias = "package-id")]
    fn set_package_id(&self, package_id: Option<&str>) {
        ObjectExt::set_property(self.as_ref(),"package-id", package_id)
    }

    #[cfg(feature = "v0_5_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v0_5_4")))]
    fn restart(&self) -> RestartEnum {
        ObjectExt::property(self.as_ref(), "restart")
    }

    #[cfg(feature = "v0_5_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v0_5_4")))]
    fn set_restart(&self, restart: RestartEnum) {
        ObjectExt::set_property(self.as_ref(),"restart", restart)
    }

    #[cfg(feature = "v0_5_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v0_5_4")))]
    #[doc(alias = "package-id")]
    fn connect_package_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_package_id_trampoline<P: IsA<RequireRestart>, F: Fn(&P) + 'static>(this: *mut ffi::PkRequireRestart, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(RequireRestart::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::package-id\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_package_id_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[cfg(feature = "v0_5_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v0_5_4")))]
    #[doc(alias = "restart")]
    fn connect_restart_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_restart_trampoline<P: IsA<RequireRestart>, F: Fn(&P) + 'static>(this: *mut ffi::PkRequireRestart, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(RequireRestart::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::restart\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_restart_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl<O: IsA<RequireRestart>> RequireRestartExt for O {}