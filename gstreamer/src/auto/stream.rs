// This file was generated by gir (5c71144) from gir-files (???)
// DO NOT EDIT

#[cfg(feature = "v1_10")]
use Caps;
use Object;
use StreamFlags;
use StreamType;
use ffi;
use glib;
use glib::Value;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Stream(Object<ffi::GstStream>): Object;

    match fn {
        get_type => || ffi::gst_stream_get_type(),
    }
}

impl Stream {
    #[cfg(feature = "v1_10")]
    pub fn new<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b Caps>>>(stream_id: P, caps: Q, type_: StreamType, flags: StreamFlags) -> Stream {
        assert_initialized_main_thread!();
        let stream_id = stream_id.into();
        let stream_id = stream_id.to_glib_none();
        let caps = caps.into();
        let caps = caps.to_glib_none();
        unsafe {
            from_glib_full(ffi::gst_stream_new(stream_id.0, caps.0, type_.to_glib(), flags.to_glib()))
        }
    }
}

unsafe impl Send for Stream {}
unsafe impl Sync for Stream {}

pub trait StreamExt {
    #[cfg(feature = "v1_10")]
    fn get_caps(&self) -> Option<Caps>;

    #[cfg(feature = "v1_10")]
    fn get_stream_flags(&self) -> StreamFlags;

    #[cfg(feature = "v1_10")]
    fn get_stream_id(&self) -> Option<String>;

    #[cfg(feature = "v1_10")]
    fn get_stream_type(&self) -> StreamType;

    //#[cfg(feature = "v1_10")]
    //fn get_tags(&self) -> /*Ignored*/Option<TagList>;

    #[cfg(feature = "v1_10")]
    fn set_caps<'a, P: Into<Option<&'a Caps>>>(&self, caps: P);

    #[cfg(feature = "v1_10")]
    fn set_stream_flags(&self, flags: StreamFlags);

    #[cfg(feature = "v1_10")]
    fn set_stream_type(&self, stream_type: StreamType);

    //#[cfg(feature = "v1_10")]
    //fn set_tags<'a, P: Into<Option<&'a /*Ignored*/TagList>>>(&self, tags: P);

    fn get_property_stream_flags(&self) -> StreamFlags;

    fn set_property_stream_flags(&self, stream_flags: StreamFlags);

    fn get_property_stream_id(&self) -> Option<String>;

    fn get_property_stream_type(&self) -> StreamType;

    fn set_property_stream_type(&self, stream_type: StreamType);

    //fn get_property_tags(&self) -> /*Ignored*/Option<TagList>;

    //fn set_property_tags(&self, tags: /*Ignored*/Option<&TagList>);
}

impl<O: IsA<Stream> + IsA<glib::object::Object>> StreamExt for O {
    #[cfg(feature = "v1_10")]
    fn get_caps(&self) -> Option<Caps> {
        unsafe {
            from_glib_full(ffi::gst_stream_get_caps(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v1_10")]
    fn get_stream_flags(&self) -> StreamFlags {
        unsafe {
            from_glib(ffi::gst_stream_get_stream_flags(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v1_10")]
    fn get_stream_id(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_stream_get_stream_id(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v1_10")]
    fn get_stream_type(&self) -> StreamType {
        unsafe {
            from_glib(ffi::gst_stream_get_stream_type(self.to_glib_none().0))
        }
    }

    //#[cfg(feature = "v1_10")]
    //fn get_tags(&self) -> /*Ignored*/Option<TagList> {
    //    unsafe { TODO: call ffi::gst_stream_get_tags() }
    //}

    #[cfg(feature = "v1_10")]
    fn set_caps<'a, P: Into<Option<&'a Caps>>>(&self, caps: P) {
        let caps = caps.into();
        let caps = caps.to_glib_none();
        unsafe {
            ffi::gst_stream_set_caps(self.to_glib_none().0, caps.0);
        }
    }

    #[cfg(feature = "v1_10")]
    fn set_stream_flags(&self, flags: StreamFlags) {
        unsafe {
            ffi::gst_stream_set_stream_flags(self.to_glib_none().0, flags.to_glib());
        }
    }

    #[cfg(feature = "v1_10")]
    fn set_stream_type(&self, stream_type: StreamType) {
        unsafe {
            ffi::gst_stream_set_stream_type(self.to_glib_none().0, stream_type.to_glib());
        }
    }

    //#[cfg(feature = "v1_10")]
    //fn set_tags<'a, P: Into<Option<&'a /*Ignored*/TagList>>>(&self, tags: P) {
    //    unsafe { TODO: call ffi::gst_stream_set_tags() }
    //}

    fn get_property_stream_flags(&self) -> StreamFlags {
        let mut value = Value::from(&0u32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "stream-flags".to_glib_none().0, value.to_glib_none_mut().0);
            from_glib(transmute(value.get::<u32>().unwrap()))
        }
    }

    fn set_property_stream_flags(&self, stream_flags: StreamFlags) {
        let stream_flags = stream_flags.to_glib().bits() as u32;
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "stream-flags".to_glib_none().0, Value::from(&stream_flags).to_glib_none().0);
        }
    }

    fn get_property_stream_id(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "stream-id".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn get_property_stream_type(&self) -> StreamType {
        let mut value = Value::from(&0u32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "stream-type".to_glib_none().0, value.to_glib_none_mut().0);
            from_glib(transmute(value.get::<u32>().unwrap()))
        }
    }

    fn set_property_stream_type(&self, stream_type: StreamType) {
        let stream_type = stream_type.to_glib().bits() as u32;
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "stream-type".to_glib_none().0, Value::from(&stream_type).to_glib_none().0);
        }
    }

    //fn get_property_tags(&self) -> /*Ignored*/Option<TagList> {
    //    let mut value = Value::from(None::<&/*Ignored*/TagList>);
    //    unsafe {
    //        gobject_ffi::g_object_get_property(self.to_glib_none().0, "tags".to_glib_none().0, value.to_glib_none_mut().0);
    //    }
    //    value.get()
    //}

    //fn set_property_tags(&self, tags: /*Ignored*/Option<&TagList>) {
    //    unsafe {
    //        gobject_ffi::g_object_set_property(self.to_glib_none().0, "tags".to_glib_none().0, Value::from(tags).to_glib_none().0);
    //    }
    //}
}
