// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use InputHints;
use InputPurpose;
use Widget;
use gdk;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use gtk_sys;
use libc;
use pango;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct IMContext(Object<gtk_sys::GtkIMContext, gtk_sys::GtkIMContextClass, IMContextClass>);

    match fn {
        get_type => || gtk_sys::gtk_im_context_get_type(),
    }
}

pub const NONE_IM_CONTEXT: Option<&IMContext> = None;

pub trait IMContextExt: 'static {
    fn delete_surrounding(&self, offset: i32, n_chars: i32) -> bool;

    //fn filter_keypress(&self, event: /*Ignored*/&mut gdk::EventKey) -> bool;

    fn focus_in(&self);

    fn focus_out(&self);

    fn get_preedit_string(&self) -> (GString, pango::AttrList, i32);

    fn get_surrounding(&self) -> Option<(GString, i32)>;

    fn reset(&self);

    fn set_client_widget<P: IsA<Widget>>(&self, widget: Option<&P>);

    fn set_cursor_location(&self, area: &gdk::Rectangle);

    fn set_surrounding(&self, text: &str, cursor_index: i32);

    fn set_use_preedit(&self, use_preedit: bool);

    fn get_property_input_hints(&self) -> InputHints;

    fn set_property_input_hints(&self, input_hints: InputHints);

    fn get_property_input_purpose(&self) -> InputPurpose;

    fn set_property_input_purpose(&self, input_purpose: InputPurpose);

    fn connect_commit<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_delete_surrounding<F: Fn(&Self, i32, i32) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_preedit_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_preedit_end<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_preedit_start<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_retrieve_surrounding<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_input_hints_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_input_purpose_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<IMContext>> IMContextExt for O {
    fn delete_surrounding(&self, offset: i32, n_chars: i32) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_im_context_delete_surrounding(self.as_ref().to_glib_none().0, offset, n_chars))
        }
    }

    //fn filter_keypress(&self, event: /*Ignored*/&mut gdk::EventKey) -> bool {
    //    unsafe { TODO: call gtk_sys:gtk_im_context_filter_keypress() }
    //}

    fn focus_in(&self) {
        unsafe {
            gtk_sys::gtk_im_context_focus_in(self.as_ref().to_glib_none().0);
        }
    }

    fn focus_out(&self) {
        unsafe {
            gtk_sys::gtk_im_context_focus_out(self.as_ref().to_glib_none().0);
        }
    }

    fn get_preedit_string(&self) -> (GString, pango::AttrList, i32) {
        unsafe {
            let mut str = ptr::null_mut();
            let mut attrs = ptr::null_mut();
            let mut cursor_pos = mem::uninitialized();
            gtk_sys::gtk_im_context_get_preedit_string(self.as_ref().to_glib_none().0, &mut str, &mut attrs, &mut cursor_pos);
            (from_glib_full(str), from_glib_full(attrs), cursor_pos)
        }
    }

    fn get_surrounding(&self) -> Option<(GString, i32)> {
        unsafe {
            let mut text = ptr::null_mut();
            let mut cursor_index = mem::uninitialized();
            let ret = from_glib(gtk_sys::gtk_im_context_get_surrounding(self.as_ref().to_glib_none().0, &mut text, &mut cursor_index));
            if ret { Some((from_glib_full(text), cursor_index)) } else { None }
        }
    }

    fn reset(&self) {
        unsafe {
            gtk_sys::gtk_im_context_reset(self.as_ref().to_glib_none().0);
        }
    }

    fn set_client_widget<P: IsA<Widget>>(&self, widget: Option<&P>) {
        unsafe {
            gtk_sys::gtk_im_context_set_client_widget(self.as_ref().to_glib_none().0, widget.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    fn set_cursor_location(&self, area: &gdk::Rectangle) {
        unsafe {
            gtk_sys::gtk_im_context_set_cursor_location(self.as_ref().to_glib_none().0, area.to_glib_none().0);
        }
    }

    fn set_surrounding(&self, text: &str, cursor_index: i32) {
        let len = text.len() as i32;
        unsafe {
            gtk_sys::gtk_im_context_set_surrounding(self.as_ref().to_glib_none().0, text.to_glib_none().0, len, cursor_index);
        }
    }

    fn set_use_preedit(&self, use_preedit: bool) {
        unsafe {
            gtk_sys::gtk_im_context_set_use_preedit(self.as_ref().to_glib_none().0, use_preedit.to_glib());
        }
    }

    fn get_property_input_hints(&self) -> InputHints {
        unsafe {
            let mut value = Value::from_type(<InputHints as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"input-hints\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_input_hints(&self, input_hints: InputHints) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"input-hints\0".as_ptr() as *const _, Value::from(&input_hints).to_glib_none().0);
        }
    }

    fn get_property_input_purpose(&self) -> InputPurpose {
        unsafe {
            let mut value = Value::from_type(<InputPurpose as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"input-purpose\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_input_purpose(&self, input_purpose: InputPurpose) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"input-purpose\0".as_ptr() as *const _, Value::from(&input_purpose).to_glib_none().0);
        }
    }

    fn connect_commit<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"commit\0".as_ptr() as *const _,
                Some(transmute(commit_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_delete_surrounding<F: Fn(&Self, i32, i32) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"delete-surrounding\0".as_ptr() as *const _,
                Some(transmute(delete_surrounding_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_preedit_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"preedit-changed\0".as_ptr() as *const _,
                Some(transmute(preedit_changed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_preedit_end<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"preedit-end\0".as_ptr() as *const _,
                Some(transmute(preedit_end_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_preedit_start<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"preedit-start\0".as_ptr() as *const _,
                Some(transmute(preedit_start_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_retrieve_surrounding<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"retrieve-surrounding\0".as_ptr() as *const _,
                Some(transmute(retrieve_surrounding_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_input_hints_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::input-hints\0".as_ptr() as *const _,
                Some(transmute(notify_input_hints_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_input_purpose_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::input-purpose\0".as_ptr() as *const _,
                Some(transmute(notify_input_purpose_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn commit_trampoline<P, F: Fn(&P, &str) + 'static>(this: *mut gtk_sys::GtkIMContext, str: *mut libc::c_char, f: glib_sys::gpointer)
where P: IsA<IMContext> {
    let f: &F = &*(f as *const F);
    f(&IMContext::from_glib_borrow(this).unsafe_cast(), &GString::from_glib_borrow(str))
}

unsafe extern "C" fn delete_surrounding_trampoline<P, F: Fn(&P, i32, i32) -> bool + 'static>(this: *mut gtk_sys::GtkIMContext, offset: libc::c_int, n_chars: libc::c_int, f: glib_sys::gpointer) -> glib_sys::gboolean
where P: IsA<IMContext> {
    let f: &F = &*(f as *const F);
    f(&IMContext::from_glib_borrow(this).unsafe_cast(), offset, n_chars).to_glib()
}

unsafe extern "C" fn preedit_changed_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkIMContext, f: glib_sys::gpointer)
where P: IsA<IMContext> {
    let f: &F = &*(f as *const F);
    f(&IMContext::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn preedit_end_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkIMContext, f: glib_sys::gpointer)
where P: IsA<IMContext> {
    let f: &F = &*(f as *const F);
    f(&IMContext::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn preedit_start_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkIMContext, f: glib_sys::gpointer)
where P: IsA<IMContext> {
    let f: &F = &*(f as *const F);
    f(&IMContext::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn retrieve_surrounding_trampoline<P, F: Fn(&P) -> bool + 'static>(this: *mut gtk_sys::GtkIMContext, f: glib_sys::gpointer) -> glib_sys::gboolean
where P: IsA<IMContext> {
    let f: &F = &*(f as *const F);
    f(&IMContext::from_glib_borrow(this).unsafe_cast()).to_glib()
}

unsafe extern "C" fn notify_input_hints_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkIMContext, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<IMContext> {
    let f: &F = &*(f as *const F);
    f(&IMContext::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_input_purpose_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkIMContext, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<IMContext> {
    let f: &F = &*(f as *const F);
    f(&IMContext::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for IMContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IMContext")
    }
}
