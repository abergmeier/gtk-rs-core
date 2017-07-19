// This file was generated by gir (a4dcfea) from gir-files (0bcaef9)
// DO NOT EDIT

use Actionable;
use Bin;
use Container;
use ToolButton;
use ToolItem;
use Widget;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct ToggleToolButton(Object<ffi::GtkToggleToolButton>): ToolButton, ToolItem, Bin, Container, Widget, Actionable;

    match fn {
        get_type => || ffi::gtk_toggle_tool_button_get_type(),
    }
}

impl ToggleToolButton {
    pub fn new() -> ToggleToolButton {
        assert_initialized_main_thread!();
        unsafe {
            ToolItem::from_glib_none(ffi::gtk_toggle_tool_button_new()).downcast_unchecked()
        }
    }

    pub fn new_from_stock(stock_id: &str) -> ToggleToolButton {
        assert_initialized_main_thread!();
        unsafe {
            ToolItem::from_glib_none(ffi::gtk_toggle_tool_button_new_from_stock(stock_id.to_glib_none().0)).downcast_unchecked()
        }
    }
}

pub trait ToggleToolButtonExt {
    fn get_active(&self) -> bool;

    fn set_active(&self, is_active: bool);

    fn connect_toggled<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<ToggleToolButton> + IsA<glib::object::Object>> ToggleToolButtonExt for O {
    fn get_active(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_toggle_tool_button_get_active(self.to_glib_none().0))
        }
    }

    fn set_active(&self, is_active: bool) {
        unsafe {
            ffi::gtk_toggle_tool_button_set_active(self.to_glib_none().0, is_active.to_glib());
        }
    }

    fn connect_toggled<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "toggled",
                transmute(toggled_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn toggled_trampoline<P>(this: *mut ffi::GtkToggleToolButton, f: glib_ffi::gpointer)
where P: IsA<ToggleToolButton> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&ToggleToolButton::from_glib_none(this).downcast_unchecked())
}
