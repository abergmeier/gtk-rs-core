// This file was generated by gir (a4dcfea) from gir-files (0bcaef9)
// DO NOT EDIT

use Bin;
use Container;
use Stack;
use Widget;
use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct StackSidebar(Object<ffi::GtkStackSidebar>): Bin, Container, Widget;

    match fn {
        get_type => || ffi::gtk_stack_sidebar_get_type(),
    }
}

impl StackSidebar {
    #[cfg(feature = "v3_16")]
    pub fn new() -> StackSidebar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_stack_sidebar_new()).downcast_unchecked()
        }
    }
}

pub trait StackSidebarExt {
    #[cfg(feature = "v3_16")]
    fn get_stack(&self) -> Option<Stack>;

    #[cfg(feature = "v3_16")]
    fn set_stack(&self, stack: &Stack);

    fn get_property_stack(&self) -> Option<Stack>;

    fn set_property_stack(&self, stack: Option<&Stack>);
}

impl<O: IsA<StackSidebar> + IsA<glib::object::Object>> StackSidebarExt for O {
    #[cfg(feature = "v3_16")]
    fn get_stack(&self) -> Option<Stack> {
        unsafe {
            from_glib_none(ffi::gtk_stack_sidebar_get_stack(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_16")]
    fn set_stack(&self, stack: &Stack) {
        unsafe {
            ffi::gtk_stack_sidebar_set_stack(self.to_glib_none().0, stack.to_glib_none().0);
        }
    }

    fn get_property_stack(&self) -> Option<Stack> {
        let mut value = Value::from(None::<&Stack>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "stack".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn set_property_stack(&self, stack: Option<&Stack>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "stack".to_glib_none().0, Value::from(stack).to_glib_none().0);
        }
    }
}
