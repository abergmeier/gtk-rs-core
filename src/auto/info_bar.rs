// This file was generated by gir (d121f7e) from gir-files (71d73f0)
// DO NOT EDIT

use Box;
use Button;
use Container;
use MessageType;
use Orientable;
use Widget;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct InfoBar(Object<ffi::GtkInfoBar>): Box, Container, Widget, Orientable;

    match fn {
        get_type => || ffi::gtk_info_bar_get_type(),
    }
}

impl InfoBar {
    pub fn new() -> InfoBar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_info_bar_new()).downcast_unchecked()
        }
    }

    //pub fn new_with_buttons<'a, P: Into<Option<&'a str>>>(first_button_text: P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> InfoBar {
    //    unsafe { TODO: call ffi::gtk_info_bar_new_with_buttons() }
    //}
}

pub trait InfoBarExt {
    fn add_action_widget<P: IsA<Widget>>(&self, child: &P, response_id: i32);

    fn add_button(&self, button_text: &str, response_id: i32) -> Option<Button>;

    //fn add_buttons(&self, first_button_text: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    fn get_action_area(&self) -> Option<Widget>;

    fn get_content_area(&self) -> Option<Widget>;

    fn get_message_type(&self) -> MessageType;

    #[cfg(feature = "v3_10")]
    fn get_show_close_button(&self) -> bool;

    fn response(&self, response_id: i32);

    fn set_default_response(&self, response_id: i32);

    fn set_message_type(&self, message_type: MessageType);

    fn set_response_sensitive(&self, response_id: i32, setting: bool);

    #[cfg(feature = "v3_10")]
    fn set_show_close_button(&self, setting: bool);

    fn connect_close<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_response<F: Fn(&Self, i32) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<InfoBar> + IsA<glib::object::Object>> InfoBarExt for O {
    fn add_action_widget<P: IsA<Widget>>(&self, child: &P, response_id: i32) {
        unsafe {
            ffi::gtk_info_bar_add_action_widget(self.to_glib_none().0, child.to_glib_none().0, response_id);
        }
    }

    fn add_button(&self, button_text: &str, response_id: i32) -> Option<Button> {
        unsafe {
            from_glib_none(ffi::gtk_info_bar_add_button(self.to_glib_none().0, button_text.to_glib_none().0, response_id))
        }
    }

    //fn add_buttons(&self, first_button_text: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_info_bar_add_buttons() }
    //}

    fn get_action_area(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_info_bar_get_action_area(self.to_glib_none().0))
        }
    }

    fn get_content_area(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_info_bar_get_content_area(self.to_glib_none().0))
        }
    }

    fn get_message_type(&self) -> MessageType {
        unsafe {
            from_glib(ffi::gtk_info_bar_get_message_type(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    fn get_show_close_button(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_info_bar_get_show_close_button(self.to_glib_none().0))
        }
    }

    fn response(&self, response_id: i32) {
        unsafe {
            ffi::gtk_info_bar_response(self.to_glib_none().0, response_id);
        }
    }

    fn set_default_response(&self, response_id: i32) {
        unsafe {
            ffi::gtk_info_bar_set_default_response(self.to_glib_none().0, response_id);
        }
    }

    fn set_message_type(&self, message_type: MessageType) {
        unsafe {
            ffi::gtk_info_bar_set_message_type(self.to_glib_none().0, message_type.to_glib());
        }
    }

    fn set_response_sensitive(&self, response_id: i32, setting: bool) {
        unsafe {
            ffi::gtk_info_bar_set_response_sensitive(self.to_glib_none().0, response_id, setting.to_glib());
        }
    }

    #[cfg(feature = "v3_10")]
    fn set_show_close_button(&self, setting: bool) {
        unsafe {
            ffi::gtk_info_bar_set_show_close_button(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn connect_close<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "close",
                transmute(close_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_response<F: Fn(&Self, i32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, i32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "response",
                transmute(response_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn close_trampoline<P>(this: *mut ffi::GtkInfoBar, f: glib_ffi::gpointer)
where P: IsA<InfoBar> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&InfoBar::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn response_trampoline<P>(this: *mut ffi::GtkInfoBar, response_id: libc::c_int, f: glib_ffi::gpointer)
where P: IsA<InfoBar> {
    callback_guard!();
    let f: &Box_<Fn(&P, i32) + 'static> = transmute(f);
    f(&InfoBar::from_glib_none(this).downcast_unchecked(), response_id)
}
