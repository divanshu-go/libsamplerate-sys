/* automatically generated by rust-bindgen 0.55.1 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SRC_STATE_tag {
    _unused: [u8; 0],
}
pub type SRC_STATE = SRC_STATE_tag;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SRC_DATA {
    pub data_in: *const f32,
    pub data_out: *mut f32,
    pub input_frames: ::std::os::raw::c_long,
    pub output_frames: ::std::os::raw::c_long,
    pub input_frames_used: ::std::os::raw::c_long,
    pub output_frames_gen: ::std::os::raw::c_long,
    pub end_of_input: ::std::os::raw::c_int,
    pub src_ratio: f64,
}
#[test]
fn bindgen_test_layout_SRC_DATA() {
    assert_eq!(
        ::std::mem::size_of::<SRC_DATA>(),
        64usize,
        concat!("Size of: ", stringify!(SRC_DATA))
    );
    assert_eq!(
        ::std::mem::align_of::<SRC_DATA>(),
        8usize,
        concat!("Alignment of ", stringify!(SRC_DATA))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SRC_DATA>())).data_in as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SRC_DATA),
            "::",
            stringify!(data_in)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SRC_DATA>())).data_out as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(SRC_DATA),
            "::",
            stringify!(data_out)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SRC_DATA>())).input_frames as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(SRC_DATA),
            "::",
            stringify!(input_frames)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SRC_DATA>())).output_frames as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(SRC_DATA),
            "::",
            stringify!(output_frames)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SRC_DATA>())).input_frames_used as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(SRC_DATA),
            "::",
            stringify!(input_frames_used)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SRC_DATA>())).output_frames_gen as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(SRC_DATA),
            "::",
            stringify!(output_frames_gen)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SRC_DATA>())).end_of_input as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(SRC_DATA),
            "::",
            stringify!(end_of_input)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SRC_DATA>())).src_ratio as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(SRC_DATA),
            "::",
            stringify!(src_ratio)
        )
    );
}
pub type src_callback_t = ::std::option::Option<
    unsafe extern "C" fn(
        cb_data: *mut ::std::os::raw::c_void,
        data: *mut *mut f32,
    ) -> ::std::os::raw::c_long,
>;
extern "C" {
    pub fn src_new(
        converter_type: ::std::os::raw::c_int,
        channels: ::std::os::raw::c_int,
        error: *mut ::std::os::raw::c_int,
    ) -> *mut SRC_STATE;
}
extern "C" {
    pub fn src_clone(orig: *mut SRC_STATE, error: *mut ::std::os::raw::c_int) -> *mut SRC_STATE;
}
extern "C" {
    pub fn src_callback_new(
        func: src_callback_t,
        converter_type: ::std::os::raw::c_int,
        channels: ::std::os::raw::c_int,
        error: *mut ::std::os::raw::c_int,
        cb_data: *mut ::std::os::raw::c_void,
    ) -> *mut SRC_STATE;
}
extern "C" {
    pub fn src_delete(state: *mut SRC_STATE) -> *mut SRC_STATE;
}
extern "C" {
    pub fn src_process(state: *mut SRC_STATE, data: *mut SRC_DATA) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn src_callback_read(
        state: *mut SRC_STATE,
        src_ratio: f64,
        frames: ::std::os::raw::c_long,
        data: *mut f32,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn src_simple(
        data: *mut SRC_DATA,
        converter_type: ::std::os::raw::c_int,
        channels: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn src_get_name(converter_type: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn src_get_description(
        converter_type: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn src_get_version() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn src_set_ratio(state: *mut SRC_STATE, new_ratio: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn src_get_channels(state: *mut SRC_STATE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn src_reset(state: *mut SRC_STATE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn src_is_valid_ratio(ratio: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn src_error(state: *mut SRC_STATE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn src_strerror(error: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}
pub const SRC_SINC_BEST_QUALITY: ::std::os::raw::c_uint = 0;
pub const SRC_SINC_MEDIUM_QUALITY: ::std::os::raw::c_uint = 1;
pub const SRC_SINC_FASTEST: ::std::os::raw::c_uint = 2;
pub const SRC_ZERO_ORDER_HOLD: ::std::os::raw::c_uint = 3;
pub const SRC_LINEAR: ::std::os::raw::c_uint = 4;
pub type _bindgen_ty_1 = ::std::os::raw::c_uint;
extern "C" {
    pub fn src_short_to_float_array(
        in_: *const ::std::os::raw::c_short,
        out: *mut f32,
        len: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn src_float_to_short_array(
        in_: *const f32,
        out: *mut ::std::os::raw::c_short,
        len: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn src_int_to_float_array(
        in_: *const ::std::os::raw::c_int,
        out: *mut f32,
        len: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn src_float_to_int_array(
        in_: *const f32,
        out: *mut ::std::os::raw::c_int,
        len: ::std::os::raw::c_int,
    );
}
