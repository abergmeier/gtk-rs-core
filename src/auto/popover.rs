// This file was generated by gir (a4dcfea) from gir-files (0bcaef9)
// DO NOT EDIT

use Bin;
use Container;
#[cfg(feature = "v3_20")]
use PopoverConstraint;
use PositionType;
use Widget;
use ffi;
use gdk;
#[cfg(feature = "v3_12")]
use gio;
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
    pub struct Popover(Object<ffi::GtkPopover>): Bin, Container, Widget;

    match fn {
        get_type => || ffi::gtk_popover_get_type(),
    }
}

impl Popover {
    #[cfg(feature = "v3_12")]
    pub fn new<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(relative_to: Q) -> Popover {
        assert_initialized_main_thread!();
        let relative_to = relative_to.into();
        let relative_to = relative_to.to_glib_none();
        unsafe {
            Widget::from_glib_none(ffi::gtk_popover_new(relative_to.0)).downcast_unchecked()
        }
    }

    #[cfg(feature = "v3_12")]
    pub fn new_from_model<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>, R: IsA<gio::MenuModel>>(relative_to: Q, model: &R) -> Popover {
        assert_initialized_main_thread!();
        let relative_to = relative_to.into();
        let relative_to = relative_to.to_glib_none();
        unsafe {
            Widget::from_glib_none(ffi::gtk_popover_new_from_model(relative_to.0, model.to_glib_none().0)).downcast_unchecked()
        }
    }
}

pub trait PopoverExt {
    #[cfg(feature = "v3_12")]
    fn bind_model<'a, 'b, P: IsA<gio::MenuModel> + 'a, Q: Into<Option<&'a P>>, R: Into<Option<&'b str>>>(&self, model: Q, action_namespace: R);

    #[cfg(feature = "v3_20")]
    fn get_constrain_to(&self) -> PopoverConstraint;

    #[cfg(feature = "v3_18")]
    fn get_default_widget(&self) -> Option<Widget>;

    #[cfg(feature = "v3_12")]
    fn get_modal(&self) -> bool;

    fn get_pointing_to(&self) -> Option<gdk::Rectangle>;

    fn get_position(&self) -> PositionType;

    #[cfg(feature = "v3_12")]
    fn get_relative_to(&self) -> Option<Widget>;

    #[cfg(feature = "v3_16")]
    fn get_transitions_enabled(&self) -> bool;

    #[cfg(feature = "v3_22")]
    fn popdown(&self);

    #[cfg(feature = "v3_22")]
    fn popup(&self);

    #[cfg(feature = "v3_20")]
    fn set_constrain_to(&self, constraint: PopoverConstraint);

    #[cfg(feature = "v3_18")]
    fn set_default_widget<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, widget: Q);

    #[cfg(feature = "v3_12")]
    fn set_modal(&self, modal: bool);

    #[cfg(feature = "v3_12")]
    fn set_pointing_to(&self, rect: &gdk::Rectangle);

    #[cfg(feature = "v3_12")]
    fn set_position(&self, position: PositionType);

    #[cfg(feature = "v3_12")]
    fn set_relative_to<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, relative_to: Q);

    #[cfg(feature = "v3_16")]
    fn set_transitions_enabled(&self, transitions_enabled: bool);

    fn connect_closed<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<Popover> + IsA<glib::object::Object>> PopoverExt for O {
    #[cfg(feature = "v3_12")]
    fn bind_model<'a, 'b, P: IsA<gio::MenuModel> + 'a, Q: Into<Option<&'a P>>, R: Into<Option<&'b str>>>(&self, model: Q, action_namespace: R) {
        let model = model.into();
        let model = model.to_glib_none();
        let action_namespace = action_namespace.into();
        let action_namespace = action_namespace.to_glib_none();
        unsafe {
            ffi::gtk_popover_bind_model(self.to_glib_none().0, model.0, action_namespace.0);
        }
    }

    #[cfg(feature = "v3_20")]
    fn get_constrain_to(&self) -> PopoverConstraint {
        unsafe {
            from_glib(ffi::gtk_popover_get_constrain_to(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_18")]
    fn get_default_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_popover_get_default_widget(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_12")]
    fn get_modal(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_popover_get_modal(self.to_glib_none().0))
        }
    }

    fn get_pointing_to(&self) -> Option<gdk::Rectangle> {
        unsafe {
            let mut rect = gdk::Rectangle::uninitialized();
            let ret = from_glib(ffi::gtk_popover_get_pointing_to(self.to_glib_none().0, rect.to_glib_none_mut().0));
            if ret { Some(rect) } else { None }
        }
    }

    fn get_position(&self) -> PositionType {
        unsafe {
            from_glib(ffi::gtk_popover_get_position(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_12")]
    fn get_relative_to(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_popover_get_relative_to(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_16")]
    fn get_transitions_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_popover_get_transitions_enabled(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_22")]
    fn popdown(&self) {
        unsafe {
            ffi::gtk_popover_popdown(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_22")]
    fn popup(&self) {
        unsafe {
            ffi::gtk_popover_popup(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_20")]
    fn set_constrain_to(&self, constraint: PopoverConstraint) {
        unsafe {
            ffi::gtk_popover_set_constrain_to(self.to_glib_none().0, constraint.to_glib());
        }
    }

    #[cfg(feature = "v3_18")]
    fn set_default_widget<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, widget: Q) {
        let widget = widget.into();
        let widget = widget.to_glib_none();
        unsafe {
            ffi::gtk_popover_set_default_widget(self.to_glib_none().0, widget.0);
        }
    }

    #[cfg(feature = "v3_12")]
    fn set_modal(&self, modal: bool) {
        unsafe {
            ffi::gtk_popover_set_modal(self.to_glib_none().0, modal.to_glib());
        }
    }

    #[cfg(feature = "v3_12")]
    fn set_pointing_to(&self, rect: &gdk::Rectangle) {
        unsafe {
            ffi::gtk_popover_set_pointing_to(self.to_glib_none().0, rect.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_12")]
    fn set_position(&self, position: PositionType) {
        unsafe {
            ffi::gtk_popover_set_position(self.to_glib_none().0, position.to_glib());
        }
    }

    #[cfg(feature = "v3_12")]
    fn set_relative_to<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, relative_to: Q) {
        let relative_to = relative_to.into();
        let relative_to = relative_to.to_glib_none();
        unsafe {
            ffi::gtk_popover_set_relative_to(self.to_glib_none().0, relative_to.0);
        }
    }

    #[cfg(feature = "v3_16")]
    fn set_transitions_enabled(&self, transitions_enabled: bool) {
        unsafe {
            ffi::gtk_popover_set_transitions_enabled(self.to_glib_none().0, transitions_enabled.to_glib());
        }
    }

    fn connect_closed<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "closed",
                transmute(closed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn closed_trampoline<P>(this: *mut ffi::GtkPopover, f: glib_ffi::gpointer)
where P: IsA<Popover> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&Popover::from_glib_none(this).downcast_unchecked())
}
