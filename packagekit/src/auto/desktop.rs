// Generated by gir (https://github.com/gtk-rs/gir @ 1c7a6b57a5fc)
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 21b29d0e0c1a)
// from packagekit-gir-files
// DO NOT EDIT

use glib::{prelude::*};
#[cfg(feature = "v0_5_3")]
#[cfg_attr(docsrs, doc(cfg(feature = "v0_5_3")))]
use glib::{translate::*};

glib::wrapper! {
    #[doc(alias = "PkDesktop")]
    pub struct Desktop(Object<ffi::PkDesktop, ffi::PkDesktopClass>);

    match fn {
        type_ => || ffi::pk_desktop_get_type(),
    }
}

impl Desktop {
        pub const NONE: Option<&'static Desktop> = None;
    

    /// NOTE: This method is unused and will be removed next time the library
    /// soname changes!
    #[cfg(feature = "v0_5_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v0_5_3")))]
    #[doc(alias = "pk_desktop_new")]
    pub fn new() -> Desktop {
        unsafe {
            from_glib_full(ffi::pk_desktop_new())
        }
    }
}

#[cfg(feature = "v0_5_3")]
#[cfg_attr(docsrs, doc(cfg(feature = "v0_5_3")))]
impl Default for Desktop {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Desktop>> Sealed for T {}
}

/// Trait containing all [`struct@Desktop`] methods.
///
/// # Implementors
///
/// [`Desktop`][struct@crate::Desktop]
pub trait DesktopExt: IsA<Desktop> + sealed::Sealed + 'static {
    /// Return all desktop files owned by a package, regardless if they are shown
    /// in the main menu or not.
    /// ## `package`
    /// the package name, e.g. "gnome-power-manager"
    ///
    /// # Returns
    ///
    /// string array of results, free with `g_ptr_array_unref()`
    ///
    /// NOTE: This method is unused and will be removed next time the library
    /// soname changes!
    #[cfg(feature = "v0_5_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v0_5_3")))]
    #[doc(alias = "pk_desktop_get_files_for_package")]
    #[doc(alias = "get_files_for_package")]
    fn files_for_package(&self, package: &str) -> Result<Vec<glib::GString>, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::pk_desktop_get_files_for_package(self.as_ref().to_glib_none().0, package.to_glib_none().0, &mut error);
            if error.is_null() { Ok(FromGlibPtrContainer::from_glib_container(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    /// Returns the package name that owns the desktop file. Fast.
    /// ## `filename`
    /// a fully qualified filename
    ///
    /// # Returns
    ///
    /// package name, or [`None`]
    ///
    /// NOTE: This method is unused and will be removed next time the library
    /// soname changes!
    #[cfg(feature = "v0_5_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v0_5_3")))]
    #[doc(alias = "pk_desktop_get_package_for_file")]
    #[doc(alias = "get_package_for_file")]
    fn package_for_file(&self, filename: &str) -> Result<glib::GString, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::pk_desktop_get_package_for_file(self.as_ref().to_glib_none().0, filename.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    /// Return all desktop files owned by a package that would be shown in a menu,
    /// i.e are an application
    /// ## `package`
    /// the package name, e.g. "gnome-power-manager"
    ///
    /// # Returns
    ///
    /// string array of results, free with `g_ptr_array_unref()`
    ///
    /// NOTE: This method is unused and will be removed next time the library
    /// soname changes!
    #[cfg(feature = "v0_5_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v0_5_3")))]
    #[doc(alias = "pk_desktop_get_shown_for_package")]
    #[doc(alias = "get_shown_for_package")]
    fn shown_for_package(&self, package: &str) -> Result<Vec<glib::GString>, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::pk_desktop_get_shown_for_package(self.as_ref().to_glib_none().0, package.to_glib_none().0, &mut error);
            if error.is_null() { Ok(FromGlibPtrContainer::from_glib_container(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    /// This method is unused and will be removed next time the library
    /// soname changes!
    ///
    /// # Returns
    ///
    /// [`true`] if opened correctly
    #[cfg(feature = "v0_5_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v0_5_3")))]
    #[doc(alias = "pk_desktop_open_database")]
    fn open_database(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::pk_desktop_open_database(self.as_ref().to_glib_none().0, &mut error);
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }
}

impl<O: IsA<Desktop>> DesktopExt for O {}