/* automatically generated by rust-bindgen 0.64.0 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Widget {
    _unused: [u8; 0],
}
pub type Fl_Callback = ::core::option::Option<
    unsafe extern "C" fn(arg1: *mut Fl_Widget, arg2: *mut ::core::ffi::c_void),
>;
pub type custom_handler_callback = ::core::option::Option<
    unsafe extern "C" fn(
        arg1: *mut Fl_Widget,
        arg2: ::core::ffi::c_int,
        arg3: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type custom_draw_callback = ::core::option::Option<
    unsafe extern "C" fn(arg1: *mut Fl_Widget, arg2: *mut ::core::ffi::c_void),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Flow {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Flow_new(
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        title: *const ::core::ffi::c_char,
    ) -> *mut Fl_Flow;
}
extern "C" {
    pub fn Fl_Flow_x(arg1: *mut Fl_Flow) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_Flow_y(arg1: *mut Fl_Flow) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_Flow_width(arg1: *mut Fl_Flow) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_Flow_height(arg1: *mut Fl_Flow) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_Flow_label(arg1: *mut Fl_Flow) -> *const ::core::ffi::c_char;
}
extern "C" {
    pub fn Fl_Flow_set_label(arg1: *mut Fl_Flow, title: *const ::core::ffi::c_char);
}
extern "C" {
    pub fn Fl_Flow_redraw(arg1: *mut Fl_Flow);
}
extern "C" {
    pub fn Fl_Flow_show(arg1: *mut Fl_Flow);
}
extern "C" {
    pub fn Fl_Flow_hide(arg1: *mut Fl_Flow);
}
extern "C" {
    pub fn Fl_Flow_activate(arg1: *mut Fl_Flow);
}
extern "C" {
    pub fn Fl_Flow_deactivate(arg1: *mut Fl_Flow);
}
extern "C" {
    pub fn Fl_Flow_redraw_label(arg1: *mut Fl_Flow);
}
extern "C" {
    pub fn Fl_Flow_resize(
        arg1: *mut Fl_Flow,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn Fl_Flow_widget_resize(
        arg1: *mut Fl_Flow,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn Fl_Flow_tooltip(arg1: *mut Fl_Flow) -> *const ::core::ffi::c_char;
}
extern "C" {
    pub fn Fl_Flow_set_tooltip(arg1: *mut Fl_Flow, txt: *const ::core::ffi::c_char);
}
extern "C" {
    pub fn Fl_Flow_get_type(arg1: *mut Fl_Flow) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_Flow_set_type(arg1: *mut Fl_Flow, typ: ::core::ffi::c_int);
}
extern "C" {
    pub fn Fl_Flow_color(arg1: *mut Fl_Flow) -> ::core::ffi::c_uint;
}
extern "C" {
    pub fn Fl_Flow_set_color(arg1: *mut Fl_Flow, color: ::core::ffi::c_uint);
}
extern "C" {
    pub fn Fl_Flow_measure_label(
        arg1: *const Fl_Flow,
        arg2: *mut ::core::ffi::c_int,
        arg3: *mut ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn Fl_Flow_label_color(arg1: *mut Fl_Flow) -> ::core::ffi::c_uint;
}
extern "C" {
    pub fn Fl_Flow_set_label_color(arg1: *mut Fl_Flow, color: ::core::ffi::c_uint);
}
extern "C" {
    pub fn Fl_Flow_label_font(arg1: *mut Fl_Flow) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_Flow_set_label_font(arg1: *mut Fl_Flow, font: ::core::ffi::c_int);
}
extern "C" {
    pub fn Fl_Flow_label_size(arg1: *mut Fl_Flow) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_Flow_set_label_size(arg1: *mut Fl_Flow, sz: ::core::ffi::c_int);
}
extern "C" {
    pub fn Fl_Flow_label_type(arg1: *mut Fl_Flow) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_Flow_set_label_type(arg1: *mut Fl_Flow, typ: ::core::ffi::c_int);
}
extern "C" {
    pub fn Fl_Flow_box(arg1: *mut Fl_Flow) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_Flow_set_box(arg1: *mut Fl_Flow, typ: ::core::ffi::c_int);
}
extern "C" {
    pub fn Fl_Flow_changed(arg1: *mut Fl_Flow) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_Flow_set_changed(arg1: *mut Fl_Flow);
}
extern "C" {
    pub fn Fl_Flow_clear_changed(arg1: *mut Fl_Flow);
}
extern "C" {
    pub fn Fl_Flow_align(arg1: *mut Fl_Flow) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_Flow_set_align(arg1: *mut Fl_Flow, typ: ::core::ffi::c_int);
}
extern "C" {
    pub fn Fl_Flow_delete(arg1: *mut Fl_Flow);
}
extern "C" {
    pub fn Fl_Flow_set_image(arg1: *mut Fl_Flow, arg2: *mut ::core::ffi::c_void);
}
extern "C" {
    pub fn Fl_Flow_handle(
        self_: *mut Fl_Flow,
        cb: custom_handler_callback,
        data: *mut ::core::ffi::c_void,
    );
}
extern "C" {
    pub fn Fl_Flow_handle_event(self_: *mut Fl_Flow, event: ::core::ffi::c_int);
}
extern "C" {
    pub fn Fl_Flow_draw(
        self_: *mut Fl_Flow,
        cb: custom_draw_callback,
        data: *mut ::core::ffi::c_void,
    );
}
extern "C" {
    pub fn Fl_Flow_resize_callback(
        self_: *mut Fl_Flow,
        cb: ::core::option::Option<
            unsafe extern "C" fn(
                arg1: *mut Fl_Widget,
                x: ::core::ffi::c_int,
                y: ::core::ffi::c_int,
                w: ::core::ffi::c_int,
                h: ::core::ffi::c_int,
                arg2: *mut ::core::ffi::c_void,
            ),
        >,
        data: *mut ::core::ffi::c_void,
    );
}
extern "C" {
    pub fn Fl_Flow_set_when(arg1: *mut Fl_Flow, arg2: ::core::ffi::c_int);
}
extern "C" {
    pub fn Fl_Flow_when(arg1: *const Fl_Flow) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_Flow_image(arg1: *const Fl_Flow) -> *const ::core::ffi::c_void;
}
extern "C" {
    pub fn Fl_Flow_parent(self_: *const Fl_Flow) -> *mut ::core::ffi::c_void;
}
extern "C" {
    pub fn Fl_Flow_selection_color(arg1: *mut Fl_Flow) -> ::core::ffi::c_uint;
}
extern "C" {
    pub fn Fl_Flow_set_selection_color(arg1: *mut Fl_Flow, color: ::core::ffi::c_uint);
}
extern "C" {
    pub fn Fl_Flow_do_callback(arg1: *mut Fl_Flow);
}
extern "C" {
    pub fn Fl_Flow_inside(
        self_: *const Fl_Flow,
        arg1: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_Flow_window(arg1: *const Fl_Flow) -> *mut ::core::ffi::c_void;
}
extern "C" {
    pub fn Fl_Flow_top_window(arg1: *const Fl_Flow) -> *mut ::core::ffi::c_void;
}
extern "C" {
    pub fn Fl_Flow_takes_events(arg1: *const Fl_Flow) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_Flow_user_data(arg1: *const Fl_Flow) -> *mut ::core::ffi::c_void;
}
extern "C" {
    pub fn Fl_Flow_take_focus(self_: *mut Fl_Flow) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_Flow_set_visible_focus(self_: *mut Fl_Flow);
}
extern "C" {
    pub fn Fl_Flow_clear_visible_focus(self_: *mut Fl_Flow);
}
extern "C" {
    pub fn Fl_Flow_visible_focus(self_: *mut Fl_Flow, v: ::core::ffi::c_int);
}
extern "C" {
    pub fn Fl_Flow_has_visible_focus(self_: *mut Fl_Flow) -> ::core::ffi::c_uint;
}
extern "C" {
    pub fn Fl_Flow_set_user_data(arg1: *mut Fl_Flow, data: *mut ::core::ffi::c_void);
}
extern "C" {
    pub fn Fl_Flow_draw_data(self_: *const Fl_Flow) -> *mut ::core::ffi::c_void;
}
extern "C" {
    pub fn Fl_Flow_handle_data(self_: *const Fl_Flow) -> *mut ::core::ffi::c_void;
}
extern "C" {
    pub fn Fl_Flow_set_draw_data(self_: *mut Fl_Flow, data: *mut ::core::ffi::c_void);
}
extern "C" {
    pub fn Fl_Flow_set_handle_data(self_: *mut Fl_Flow, data: *mut ::core::ffi::c_void);
}
extern "C" {
    pub fn Fl_Flow_damage(self_: *const Fl_Flow) -> ::core::ffi::c_uchar;
}
extern "C" {
    pub fn Fl_Flow_set_damage(self_: *mut Fl_Flow, flag: ::core::ffi::c_uchar);
}
extern "C" {
    pub fn Fl_Flow_clear_damage(self_: *mut Fl_Flow);
}
extern "C" {
    pub fn Fl_Flow_as_window(self_: *mut Fl_Flow) -> *mut ::core::ffi::c_void;
}
extern "C" {
    pub fn Fl_Flow_as_group(self_: *mut Fl_Flow) -> *mut ::core::ffi::c_void;
}
extern "C" {
    pub fn Fl_Flow_set_deimage(arg1: *mut Fl_Flow, arg2: *mut ::core::ffi::c_void);
}
extern "C" {
    pub fn Fl_Flow_deimage(arg1: *const Fl_Flow) -> *const ::core::ffi::c_void;
}
extern "C" {
    pub fn Fl_Flow_set_callback(
        arg1: *mut Fl_Flow,
        arg2: Fl_Callback,
        arg3: *mut ::core::ffi::c_void,
    );
}
extern "C" {
    pub fn Fl_Flow_set_deleter(
        arg1: *mut Fl_Flow,
        arg2: ::core::option::Option<unsafe extern "C" fn(arg1: *mut ::core::ffi::c_void)>,
    );
}
extern "C" {
    pub fn Fl_Flow_visible(self_: *const Fl_Flow) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_Flow_visible_r(self_: *const Fl_Flow) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_Flow_active(self_: *const Fl_Flow) -> ::core::ffi::c_uint;
}
extern "C" {
    pub fn Fl_Flow_active_r(self_: *const Fl_Flow) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_Flow_callback(self_: *const Fl_Flow) -> Fl_Callback;
}
extern "C" {
    pub fn Fl_Flow_set_deletion_callback(
        self_: *mut Fl_Flow,
        arg1: ::core::option::Option<
            unsafe extern "C" fn(arg1: *mut Fl_Widget, arg2: *mut ::core::ffi::c_void),
        >,
        data: *mut ::core::ffi::c_void,
    );
}
extern "C" {
    pub fn Fl_Flow_from_dyn_ptr(ptr: *mut Fl_Widget) -> *mut Fl_Flow;
}
extern "C" {
    pub fn Fl_Flow_rule(self_: *mut Fl_Flow, wid: *mut Fl_Widget, inst: *const ::core::ffi::c_char);
}
extern "C" {
    pub fn Fl_Flow_begin(self_: *mut Fl_Flow);
}
extern "C" {
    pub fn Fl_Flow_end(self_: *mut Fl_Flow);
}
extern "C" {
    pub fn Fl_Flow_find(
        self_: *mut Fl_Flow,
        arg1: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_Flow_add(self_: *mut Fl_Flow, arg1: *mut ::core::ffi::c_void);
}
extern "C" {
    pub fn Fl_Flow_insert(
        self_: *mut Fl_Flow,
        arg1: *mut ::core::ffi::c_void,
        pos: ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn Fl_Flow_remove(self_: *mut Fl_Flow, wid: *mut ::core::ffi::c_void);
}
extern "C" {
    pub fn Fl_Flow_remove_by_index(self_: *mut Fl_Flow, idx: ::core::ffi::c_int);
}
extern "C" {
    pub fn Fl_Flow_clear(self_: *mut Fl_Flow);
}
extern "C" {
    pub fn Fl_Flow_children(self_: *mut Fl_Flow) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_Flow_child(arg1: *mut Fl_Flow, index: ::core::ffi::c_int) -> *mut Fl_Widget;
}
extern "C" {
    pub fn Fl_Flow_resizable(self_: *mut Fl_Flow, arg1: *mut ::core::ffi::c_void);
}
extern "C" {
    pub fn Fl_Flow_set_clip_children(self_: *mut Fl_Flow, c: ::core::ffi::c_int);
}
extern "C" {
    pub fn Fl_Flow_clip_children(self_: *mut Fl_Flow) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_Flow_init_sizes(self_: *mut Fl_Flow);
}
extern "C" {
    pub fn Fl_Flow_draw_child(self_: *const Fl_Flow, w: *mut Fl_Widget);
}
extern "C" {
    pub fn Fl_Flow_update_child(self_: *const Fl_Flow, w: *mut Fl_Widget);
}
extern "C" {
    pub fn Fl_Flow_draw_outside_label(self_: *const Fl_Flow, w: *const Fl_Widget);
}
extern "C" {
    pub fn Fl_Flow_draw_children(self_: *mut Fl_Flow);
}
