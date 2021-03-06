// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use gtk_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use EventController;
use Gesture;
use GestureSingle;

glib_wrapper! {
    pub struct GestureLongPress(Object<gtk_sys::GtkGestureLongPress, gtk_sys::GtkGestureLongPressClass, GestureLongPressClass>) @extends GestureSingle, Gesture, EventController;

    match fn {
        get_type => || gtk_sys::gtk_gesture_long_press_get_type(),
    }
}

impl GestureLongPress {
    pub fn new() -> GestureLongPress {
        assert_initialized_main_thread!();
        unsafe { Gesture::from_glib_full(gtk_sys::gtk_gesture_long_press_new()).unsafe_cast() }
    }

    pub fn get_property_delay_factor(&self) -> f64 {
        unsafe {
            let mut value = Value::from_type(<f64 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"delay-factor\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `delay-factor` getter")
                .unwrap()
        }
    }

    pub fn set_property_delay_factor(&self, delay_factor: f64) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"delay-factor\0".as_ptr() as *const _,
                Value::from(&delay_factor).to_glib_none().0,
            );
        }
    }

    pub fn connect_cancelled<F: Fn(&GestureLongPress) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn cancelled_trampoline<F: Fn(&GestureLongPress) + 'static>(
            this: *mut gtk_sys::GtkGestureLongPress,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"cancelled\0".as_ptr() as *const _,
                Some(transmute(cancelled_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_pressed<F: Fn(&GestureLongPress, f64, f64) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn pressed_trampoline<F: Fn(&GestureLongPress, f64, f64) + 'static>(
            this: *mut gtk_sys::GtkGestureLongPress,
            x: libc::c_double,
            y: libc::c_double,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), x, y)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"pressed\0".as_ptr() as *const _,
                Some(transmute(pressed_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_delay_factor_notify<F: Fn(&GestureLongPress) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_delay_factor_trampoline<F: Fn(&GestureLongPress) + 'static>(
            this: *mut gtk_sys::GtkGestureLongPress,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::delay-factor\0".as_ptr() as *const _,
                Some(transmute(notify_delay_factor_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for GestureLongPress {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for GestureLongPress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GestureLongPress")
    }
}
