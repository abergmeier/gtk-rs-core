// This file was generated by gir (0409d73) from gir-files (469db10)
// DO NOT EDIT

use Bin;
use Box;
use Container;
use Widget;
use Window;
use ffi;
use glib;
#[cfg(any(feature = "v3_12", feature = "dox"))]
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Dialog(Object<ffi::GtkDialog, ffi::GtkDialogClass>): Window, Bin, Container, Widget;

    match fn {
        get_type => || ffi::gtk_dialog_get_type(),
    }
}

impl Dialog {
    pub fn new() -> Dialog {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_dialog_new()).downcast_unchecked()
        }
    }

    //pub fn new_with_buttons<'a, 'b, 'c, P: Into<Option<&'a str>>, Q: IsA<Window> + 'b, R: Into<Option<&'b Q>>, S: Into<Option<&'c str>>>(title: P, parent: R, flags: DialogFlags, first_button_text: S, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Dialog {
    //    unsafe { TODO: call ffi::gtk_dialog_new_with_buttons() }
    //}
}

impl Default for Dialog {
    fn default() -> Self {
        Self::new()
    }
}

pub trait DialogExt {
    fn add_action_widget<P: IsA<Widget>>(&self, child: &P, response_id: i32);

    fn add_button(&self, button_text: &str, response_id: i32) -> Widget;

    //fn add_buttons(&self, first_button_text: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    fn get_action_area(&self) -> Widget;

    fn get_content_area(&self) -> Box;

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_header_bar(&self) -> Option<Widget>;

    fn get_response_for_widget<P: IsA<Widget>>(&self, widget: &P) -> i32;

    fn get_widget_for_response(&self, response_id: i32) -> Option<Widget>;

    fn response(&self, response_id: i32);

    fn run(&self) -> i32;

    //fn set_alternative_button_order(&self, first_response_id: i32, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    fn set_alternative_button_order_from_array(&self, new_order: &[i32]);

    fn set_default_response(&self, response_id: i32);

    fn set_response_sensitive(&self, response_id: i32, setting: bool);

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_property_use_header_bar(&self) -> i32;

    fn connect_close<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_close(&self);

    fn connect_response<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn connect_property_use_header_bar_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Dialog> + IsA<glib::object::Object> + glib::object::ObjectExt> DialogExt for O {
    fn add_action_widget<P: IsA<Widget>>(&self, child: &P, response_id: i32) {
        unsafe {
            ffi::gtk_dialog_add_action_widget(self.to_glib_none().0, child.to_glib_none().0, response_id);
        }
    }

    fn add_button(&self, button_text: &str, response_id: i32) -> Widget {
        unsafe {
            from_glib_none(ffi::gtk_dialog_add_button(self.to_glib_none().0, button_text.to_glib_none().0, response_id))
        }
    }

    //fn add_buttons(&self, first_button_text: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_dialog_add_buttons() }
    //}

    fn get_action_area(&self) -> Widget {
        unsafe {
            from_glib_none(ffi::gtk_dialog_get_action_area(self.to_glib_none().0))
        }
    }

    fn get_content_area(&self) -> Box {
        unsafe {
            from_glib_none(ffi::gtk_dialog_get_content_area(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_header_bar(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_dialog_get_header_bar(self.to_glib_none().0))
        }
    }

    fn get_response_for_widget<P: IsA<Widget>>(&self, widget: &P) -> i32 {
        unsafe {
            ffi::gtk_dialog_get_response_for_widget(self.to_glib_none().0, widget.to_glib_none().0)
        }
    }

    fn get_widget_for_response(&self, response_id: i32) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_dialog_get_widget_for_response(self.to_glib_none().0, response_id))
        }
    }

    fn response(&self, response_id: i32) {
        unsafe {
            ffi::gtk_dialog_response(self.to_glib_none().0, response_id);
        }
    }

    fn run(&self) -> i32 {
        unsafe {
            ffi::gtk_dialog_run(self.to_glib_none().0)
        }
    }

    //fn set_alternative_button_order(&self, first_response_id: i32, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_dialog_set_alternative_button_order() }
    //}

    fn set_alternative_button_order_from_array(&self, new_order: &[i32]) {
        let n_params = new_order.len() as i32;
        unsafe {
            ffi::gtk_dialog_set_alternative_button_order_from_array(self.to_glib_none().0, n_params, new_order.to_glib_none().0);
        }
    }

    fn set_default_response(&self, response_id: i32) {
        unsafe {
            ffi::gtk_dialog_set_default_response(self.to_glib_none().0, response_id);
        }
    }

    fn set_response_sensitive(&self, response_id: i32, setting: bool) {
        unsafe {
            ffi::gtk_dialog_set_response_sensitive(self.to_glib_none().0, response_id, setting.to_glib());
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_property_use_header_bar(&self) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "use-header-bar".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn connect_close<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "close",
                transmute(close_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_close(&self) {
        let _ = self.emit("close", &[]).unwrap();
    }

    fn connect_response<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, i32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "response",
                transmute(response_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn connect_property_use_header_bar_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::use-header-bar",
                transmute(notify_use_header_bar_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn close_trampoline<P>(this: *mut ffi::GtkDialog, f: glib_ffi::gpointer)
where P: IsA<Dialog> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Dialog::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn response_trampoline<P>(this: *mut ffi::GtkDialog, response_id: libc::c_int, f: glib_ffi::gpointer)
where P: IsA<Dialog> {
    callback_guard!();
    let f: &&(Fn(&P, i32) + 'static) = transmute(f);
    f(&Dialog::from_glib_borrow(this).downcast_unchecked(), response_id)
}

#[cfg(any(feature = "v3_12", feature = "dox"))]
unsafe extern "C" fn notify_use_header_bar_trampoline<P>(this: *mut ffi::GtkDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Dialog> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Dialog::from_glib_borrow(this).downcast_unchecked())
}
