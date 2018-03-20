// This file was generated by gir (https://github.com/gtk-rs/gir @ fe7a6ff)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use RTSPMediaFactory;
use ffi;
use glib;
use glib::StaticType;
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
    pub struct RTSPMediaFactoryURI(Object<ffi::GstRTSPMediaFactoryURI, ffi::GstRTSPMediaFactoryURIClass>): RTSPMediaFactory;

    match fn {
        get_type => || ffi::gst_rtsp_media_factory_uri_get_type(),
    }
}

impl RTSPMediaFactoryURI {
    pub fn new() -> RTSPMediaFactoryURI {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_rtsp_media_factory_uri_new())
        }
    }
}

impl Default for RTSPMediaFactoryURI {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for RTSPMediaFactoryURI {}
unsafe impl Sync for RTSPMediaFactoryURI {}

pub trait RTSPMediaFactoryURIExt {
    fn get_uri(&self) -> Option<String>;

    fn set_uri(&self, uri: &str);

    fn get_property_use_gstpay(&self) -> bool;

    fn set_property_use_gstpay(&self, use_gstpay: bool);

    fn connect_property_uri_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_use_gstpay_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<RTSPMediaFactoryURI> + IsA<glib::object::Object>> RTSPMediaFactoryURIExt for O {
    fn get_uri(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_media_factory_uri_get_uri(self.to_glib_none().0))
        }
    }

    fn set_uri(&self, uri: &str) {
        unsafe {
            ffi::gst_rtsp_media_factory_uri_set_uri(self.to_glib_none().0, uri.to_glib_none().0);
        }
    }

    fn get_property_use_gstpay(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "use-gstpay".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_use_gstpay(&self, use_gstpay: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "use-gstpay".to_glib_none().0, Value::from(&use_gstpay).to_glib_none().0);
        }
    }

    fn connect_property_uri_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::uri",
                transmute(notify_uri_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_use_gstpay_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::use-gstpay",
                transmute(notify_use_gstpay_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_uri_trampoline<P>(this: *mut ffi::GstRTSPMediaFactoryURI, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RTSPMediaFactoryURI> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&RTSPMediaFactoryURI::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_use_gstpay_trampoline<P>(this: *mut ffi::GstRTSPMediaFactoryURI, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RTSPMediaFactoryURI> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&RTSPMediaFactoryURI::from_glib_borrow(this).downcast_unchecked())
}
