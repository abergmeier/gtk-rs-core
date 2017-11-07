// This file was generated by gir (0409d73) from gir-files (469db10)
// DO NOT EDIT

use Bin;
use Container;
use Popover;
use Widget;
use ffi;
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
    pub struct PopoverMenu(Object<ffi::GtkPopoverMenu, ffi::GtkPopoverMenuClass>): Popover, Bin, Container, Widget;

    match fn {
        get_type => || ffi::gtk_popover_menu_get_type(),
    }
}

impl PopoverMenu {
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn new() -> PopoverMenu {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_popover_menu_new()).downcast_unchecked()
        }
    }
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
impl Default for PopoverMenu {
    fn default() -> Self {
        Self::new()
    }
}

pub trait PopoverMenuExt {
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn open_submenu(&self, name: &str);

    fn get_property_visible_submenu(&self) -> Option<String>;

    fn set_property_visible_submenu(&self, visible_submenu: Option<&str>);

    fn get_child_position<T: IsA<Widget>>(&self, item: &T) -> i32;

    fn set_child_position<T: IsA<Widget>>(&self, item: &T, position: i32);

    fn get_child_submenu<T: IsA<Widget>>(&self, item: &T) -> Option<String>;

    fn set_child_submenu<'a, P: Into<Option<&'a str>>, T: IsA<Widget>>(&self, item: &T, submenu: P);

    fn connect_property_visible_submenu_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<PopoverMenu> + IsA<Container> + IsA<glib::object::Object>> PopoverMenuExt for O {
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn open_submenu(&self, name: &str) {
        unsafe {
            ffi::gtk_popover_menu_open_submenu(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    fn get_property_visible_submenu(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "visible-submenu".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn set_property_visible_submenu(&self, visible_submenu: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "visible-submenu".to_glib_none().0, Value::from(visible_submenu).to_glib_none().0);
        }
    }

    fn get_child_position<T: IsA<Widget>>(&self, item: &T) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "position".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_child_position<T: IsA<Widget>>(&self, item: &T, position: i32) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "position".to_glib_none().0, Value::from(&position).to_glib_none().0);
        }
    }

    fn get_child_submenu<T: IsA<Widget>>(&self, item: &T) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "submenu".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn set_child_submenu<'a, P: Into<Option<&'a str>>, T: IsA<Widget>>(&self, item: &T, submenu: P) {
        let submenu = submenu.into();
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "submenu".to_glib_none().0, Value::from(submenu).to_glib_none().0);
        }
    }

    fn connect_property_visible_submenu_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::visible-submenu",
                transmute(notify_visible_submenu_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_visible_submenu_trampoline<P>(this: *mut ffi::GtkPopoverMenu, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<PopoverMenu> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&PopoverMenu::from_glib_borrow(this).downcast_unchecked())
}
