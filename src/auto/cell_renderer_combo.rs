// This file was generated by gir (e48471c) from gir-files (71d73f0)
// DO NOT EDIT

use CellRenderer;
use CellRendererText;
use TreeIter;
use TreePath;
use ffi;
use glib::object::Downcast;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct CellRendererCombo(Object<ffi::GtkCellRendererCombo>): CellRendererText, CellRenderer;

    match fn {
        get_type => || ffi::gtk_cell_renderer_combo_get_type(),
    }
}

impl CellRendererCombo {
    pub fn new() -> CellRendererCombo {
        assert_initialized_main_thread!();
        unsafe {
            CellRenderer::from_glib_none(ffi::gtk_cell_renderer_combo_new()).downcast_unchecked()
        }
    }

    pub fn connect_changed<F: Fn(&CellRendererCombo, TreePath, &TreeIter) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&CellRendererCombo, TreePath, &TreeIter) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "changed",
                transmute(changed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn changed_trampoline(this: *mut ffi::GtkCellRendererCombo, path_string: *mut libc::c_char, new_iter: *mut ffi::GtkTreeIter, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&CellRendererCombo, TreePath, &TreeIter) + 'static> = transmute(f);
    let path = from_glib_full(ffi::gtk_tree_path_new_from_string(path_string));
    f(&from_glib_none(this), path, &from_glib_borrow(new_iter))
}
