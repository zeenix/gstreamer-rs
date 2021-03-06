// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Error;
use Extractable;
use ffi;
#[cfg(feature = "futures")]
use futures_core;
use gio;
use gio_ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Asset(Object<ffi::GESAsset, ffi::GESAssetClass>);

    match fn {
        get_type => || ffi::ges_asset_get_type(),
    }
}

impl Asset {
    pub fn needs_reload(extractable_type: glib::types::Type, id: &str) -> bool {
        assert_initialized_main_thread!();
        unsafe {
            from_glib(ffi::ges_asset_needs_reload(extractable_type.to_glib(), id.to_glib_none().0))
        }
    }

    pub fn request<'a, P: Into<Option<&'a str>>>(extractable_type: glib::types::Type, id: P) -> Result<Option<Asset>, Error> {
        assert_initialized_main_thread!();
        let id = id.into();
        let id = id.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::ges_asset_request(extractable_type.to_glib(), id.0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn request_async<'a, P: Into<Option<&'a gio::Cancellable>>, Q: FnOnce(Result<Asset, Error>) + Send + 'static>(extractable_type: glib::types::Type, id: &str, cancellable: P, callback: Q) {
        assert_initialized_main_thread!();
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn request_async_trampoline<Q: FnOnce(Result<Asset, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut gio_ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut error = ptr::null_mut();
            let ret = ffi::ges_asset_request_finish(res, &mut error);
            let result = if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = request_async_trampoline::<Q>;
        unsafe {
            ffi::ges_asset_request_async(extractable_type.to_glib(), id.to_glib_none().0, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    pub fn request_async_future(extractable_type: glib::types::Type, id: &str) -> Box_<futures_core::Future<Item = Asset, Error = Error>> {
        use gio::GioFuture;
        use fragile::Fragile;

        let id = String::from(id);
        GioFuture::new(&(), move |_obj, send| {
            let cancellable = gio::Cancellable::new();
            let send = Fragile::new(send);
            Self::request_async(
                 extractable_type,
                 &id,
                 Some(&cancellable),
                 move |res| {
                     let _ = send.into_inner().send(res);
                 },
            );

            cancellable
        })
    }
}

pub trait AssetExt {
    fn extract(&self) -> Result<Option<Extractable>, Error>;

    fn get_error(&self) -> Option<Error>;

    fn get_extractable_type(&self) -> glib::types::Type;

    fn get_id(&self) -> Option<String>;

    fn get_proxy(&self) -> Option<Asset>;

    fn get_proxy_target(&self) -> Option<Asset>;

    fn list_proxies(&self) -> Vec<Asset>;

    fn set_proxy<'a, P: IsA<Asset> + 'a, Q: Into<Option<&'a P>>>(&self, proxy: Q) -> bool;

    fn unproxy<P: IsA<Asset>>(&self, proxy: &P) -> bool;

    fn set_property_proxy_target<P: IsA<Asset> + IsA<glib::object::Object> + glib::value::SetValueOptional>(&self, proxy_target: Option<&P>);

    fn connect_property_proxy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_proxy_target_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Asset> + IsA<glib::object::Object>> AssetExt for O {
    fn extract(&self) -> Result<Option<Extractable>, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::ges_asset_extract(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_error(&self) -> Option<Error> {
        unsafe {
            from_glib_none(ffi::ges_asset_get_error(self.to_glib_none().0))
        }
    }

    fn get_extractable_type(&self) -> glib::types::Type {
        unsafe {
            from_glib(ffi::ges_asset_get_extractable_type(self.to_glib_none().0))
        }
    }

    fn get_id(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::ges_asset_get_id(self.to_glib_none().0))
        }
    }

    fn get_proxy(&self) -> Option<Asset> {
        unsafe {
            from_glib_none(ffi::ges_asset_get_proxy(self.to_glib_none().0))
        }
    }

    fn get_proxy_target(&self) -> Option<Asset> {
        unsafe {
            from_glib_none(ffi::ges_asset_get_proxy_target(self.to_glib_none().0))
        }
    }

    fn list_proxies(&self) -> Vec<Asset> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::ges_asset_list_proxies(self.to_glib_none().0))
        }
    }

    fn set_proxy<'a, P: IsA<Asset> + 'a, Q: Into<Option<&'a P>>>(&self, proxy: Q) -> bool {
        let proxy = proxy.into();
        let proxy = proxy.to_glib_none();
        unsafe {
            from_glib(ffi::ges_asset_set_proxy(self.to_glib_none().0, proxy.0))
        }
    }

    fn unproxy<P: IsA<Asset>>(&self, proxy: &P) -> bool {
        unsafe {
            from_glib(ffi::ges_asset_unproxy(self.to_glib_none().0, proxy.to_glib_none().0))
        }
    }

    fn set_property_proxy_target<P: IsA<Asset> + IsA<glib::object::Object> + glib::value::SetValueOptional>(&self, proxy_target: Option<&P>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "proxy-target".to_glib_none().0, Value::from(proxy_target).to_glib_none().0);
        }
    }

    fn connect_property_proxy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::proxy",
                transmute(notify_proxy_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_proxy_target_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::proxy-target",
                transmute(notify_proxy_target_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_proxy_trampoline<P>(this: *mut ffi::GESAsset, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Asset> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Asset::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_proxy_target_trampoline<P>(this: *mut ffi::GESAsset, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Asset> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Asset::from_glib_borrow(this).downcast_unchecked())
}
