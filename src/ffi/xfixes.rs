/*
 * This file generated automatically from xfixes.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std;
use libc::*;
use ffi;
use ffi::xproto;
use ffi::render;
use ffi::shape;

pub static XFIXES_MAJOR_VERSION : c_uint = 5;
pub static XFIXES_MINOR_VERSION : c_uint = 0;

pub struct query_version_cookie {
    sequence : c_uint
}


pub struct query_version_request {
    major_opcode :           u8,
    minor_opcode :           u8,
    length :                 u16,
    client_major_version :   u32,
    client_minor_version :   u32
}


pub struct query_version_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    major_version :   u32,
    minor_version :   u32,
    pad1 :            [u8,..16]
}



pub struct change_save_set_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    mode :           u8,
    target :         u8,
    map :            u8,
    pad0 :           u8,
    window :         ffi::xproto::window
}



pub struct selection_notify_event {
    response_type :         u8,
    subtype :               u8,
    sequence :              u16,
    window :                ffi::xproto::window,
    owner :                 ffi::xproto::window,
    selection :             ffi::xproto::atom,
    timestamp :             ffi::xproto::timestamp,
    selection_timestamp :   ffi::xproto::timestamp,
    pad0 :                  [u8,..8]
}



pub struct select_selection_input_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         ffi::xproto::window,
    selection :      ffi::xproto::atom,
    event_mask :     u32
}



pub struct cursor_notify_event {
    response_type :   u8,
    subtype :         u8,
    sequence :        u16,
    window :          ffi::xproto::window,
    cursor_serial :   u32,
    timestamp :       ffi::xproto::timestamp,
    name :            ffi::xproto::atom,
    pad0 :            [u8,..12]
}



pub struct select_cursor_input_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         ffi::xproto::window,
    event_mask :     u32
}


pub struct get_cursor_image_cookie {
    sequence : c_uint
}


pub struct get_cursor_image_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}


pub struct get_cursor_image_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    x :               i16,
    y :               i16,
    width :           u16,
    height :          u16,
    xhot :            u16,
    yhot :            u16,
    cursor_serial :   u32,
    pad1 :            [u8,..8]
}


pub type region = u32;
/**
 * @brief region_iterator
 **/
pub struct region_iterator {
    data : *mut region,
    rem  : c_int,
    index: c_int
}



pub struct bad_region_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16
}



pub struct create_region_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    region :         region
}



pub struct create_region_from_bitmap_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    region :         region,
    bitmap :         ffi::xproto::pixmap
}



pub struct create_region_from_window_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    region :         region,
    window :         ffi::xproto::window,
    kind :           ffi::shape::kind,
    pad0 :           [u8,..3]
}



pub struct create_region_from_gc_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    region :         region,
    gc :             ffi::xproto::gcontext
}



pub struct create_region_from_picture_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    region :         region,
    picture :        ffi::render::picture
}



pub struct destroy_region_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    region :         region
}



pub struct set_region_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    region :         region
}



pub struct copy_region_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    source :         region,
    destination :    region
}



pub struct union_region_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    source1 :        region,
    source2 :        region,
    destination :    region
}



pub struct intersect_region_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    source1 :        region,
    source2 :        region,
    destination :    region
}



pub struct subtract_region_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    source1 :        region,
    source2 :        region,
    destination :    region
}



pub struct invert_region_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    source :         region,
    bounds :         ffi::xproto::rectangle,
    destination :    region
}



pub struct translate_region_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    region :         region,
    dx :             i16,
    dy :             i16
}



pub struct region_extents_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    source :         region,
    destination :    region
}


pub struct fetch_region_cookie {
    sequence : c_uint
}


pub struct fetch_region_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    region :         region
}


pub struct fetch_region_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    extents :         ffi::xproto::rectangle,
    pad1 :            [u8,..16]
}



pub struct set_gc_clip_region_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    gc :             ffi::xproto::gcontext,
    region :         region,
    x_origin :       i16,
    y_origin :       i16
}



pub struct set_window_shape_region_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    dest :           ffi::xproto::window,
    dest_kind :      ffi::shape::kind,
    pad0 :           [u8,..3],
    x_offset :       i16,
    y_offset :       i16,
    region :         region
}



pub struct set_picture_clip_region_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    picture :        ffi::render::picture,
    region :         region,
    x_origin :       i16,
    y_origin :       i16
}



pub struct set_cursor_name_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    cursor :         ffi::xproto::cursor,
    nbytes :         u16,
    pad0 :           [u8,..2]
}


pub struct get_cursor_name_cookie {
    sequence : c_uint
}


pub struct get_cursor_name_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    cursor :         ffi::xproto::cursor
}


pub struct get_cursor_name_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    atom :            ffi::xproto::atom,
    nbytes :          u16,
    pad1 :            [u8,..18]
}


pub struct get_cursor_image_and_name_cookie {
    sequence : c_uint
}


pub struct get_cursor_image_and_name_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}


pub struct get_cursor_image_and_name_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    x :               i16,
    y :               i16,
    width :           u16,
    height :          u16,
    xhot :            u16,
    yhot :            u16,
    cursor_serial :   u32,
    cursor_atom :     ffi::xproto::atom,
    nbytes :          u16,
    pad1 :            [u8,..2]
}



pub struct change_cursor_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    source :         ffi::xproto::cursor,
    destination :    ffi::xproto::cursor
}



pub struct change_cursor_by_name_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    src :            ffi::xproto::cursor,
    nbytes :         u16,
    pad0 :           [u8,..2]
}



pub struct expand_region_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    source :         region,
    destination :    region,
    left :           u16,
    right :          u16,
    top :            u16,
    bottom :         u16
}



pub struct hide_cursor_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         ffi::xproto::window
}



pub struct show_cursor_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         ffi::xproto::window
}

#[link(name="lxcb-xfixes")]
pub extern "C" {

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xfixes_query_version (c : *mut ffi::base::connection,
                                    client_major_version :  u32,
                                    client_minor_version :  u32) -> query_version_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_xfixes_query_version_unchecked (c : *mut ffi::base::connection,
                                              client_major_version :  u32,
                                              client_minor_version :  u32) -> query_version_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xfixes_query_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xfixes_query_version_reply (c : *mut ffi::base::connection,
                                          cookie : query_version_cookie,
                                          e : *mut *mut ffi::base::generic_error) -> *mut query_version_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_xfixes_change_save_set_checked (c : *mut ffi::base::connection,
                                              mode :  u8,
                                              target :  u8,
                                              map :  u8,
                                              window :  ffi::xproto::window) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xfixes_change_save_set (c : *mut ffi::base::connection,
                                      mode :  u8,
                                      target :  u8,
                                      map :  u8,
                                      window :  ffi::xproto::window) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_xfixes_select_selection_input_checked (c : *mut ffi::base::connection,
                                                     window :  ffi::xproto::window,
                                                     selection :  ffi::xproto::atom,
                                                     event_mask :  u32) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xfixes_select_selection_input (c : *mut ffi::base::connection,
                                             window :  ffi::xproto::window,
                                             selection :  ffi::xproto::atom,
                                             event_mask :  u32) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_xfixes_select_cursor_input_checked (c : *mut ffi::base::connection,
                                                  window :  ffi::xproto::window,
                                                  event_mask :  u32) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xfixes_select_cursor_input (c : *mut ffi::base::connection,
                                          window :  ffi::xproto::window,
                                          event_mask :  u32) -> ffi::base::void_cookie;

pub fn xcb_xfixes_get_cursor_image_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xfixes_get_cursor_image (c : *mut ffi::base::connection) -> get_cursor_image_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_xfixes_get_cursor_image_unchecked (c : *mut ffi::base::connection) -> get_cursor_image_cookie;

pub fn xcb_xfixes_get_cursor_image_cursor_image (R : *mut get_cursor_image_reply) -> *mut u32;


pub fn xcb_xfixes_get_cursor_image_cursor_image_length (R : *mut get_cursor_image_reply) -> c_int;


pub fn xcb_xfixes_get_cursor_image_cursor_image_end (R : *mut get_cursor_image_reply) -> ffi::base::generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xfixes_get_cursor_image_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xfixes_get_cursor_image_reply (c : *mut ffi::base::connection,
                                             cookie : get_cursor_image_cookie,
                                             e : *mut *mut ffi::base::generic_error) -> *mut get_cursor_image_reply;

/**
 * Get the next element of the iterator
 * @param i Pointer to a region_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(region)
 *
 *
 */
pub fn xcb_xfixes_region_next (i:*mut region_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An region_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_xfixes_region_end (i:region_iterator) -> ffi::base::generic_iterator;

pub fn xcb_xfixes_create_region_sizeof (_buffer :  *mut c_void,
                                 rectangles_len :  u32) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_xfixes_create_region_checked (c : *mut ffi::base::connection,
                                            region :  region,
                                            rectangles_len :  u32,
                                            rectangles : *mut ffi::xproto::rectangle) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xfixes_create_region (c : *mut ffi::base::connection,
                                    region :  region,
                                    rectangles_len :  u32,
                                    rectangles : *mut ffi::xproto::rectangle) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_xfixes_create_region_from_bitmap_checked (c : *mut ffi::base::connection,
                                                        region :  region,
                                                        bitmap :  ffi::xproto::pixmap) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xfixes_create_region_from_bitmap (c : *mut ffi::base::connection,
                                                region :  region,
                                                bitmap :  ffi::xproto::pixmap) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_xfixes_create_region_from_window_checked (c : *mut ffi::base::connection,
                                                        region :  region,
                                                        window :  ffi::xproto::window,
                                                        kind :  ffi::shape::kind) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xfixes_create_region_from_window (c : *mut ffi::base::connection,
                                                region :  region,
                                                window :  ffi::xproto::window,
                                                kind :  ffi::shape::kind) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_xfixes_create_region_from_gc_checked (c : *mut ffi::base::connection,
                                                    region :  region,
                                                    gc :  ffi::xproto::gcontext) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xfixes_create_region_from_gc (c : *mut ffi::base::connection,
                                            region :  region,
                                            gc :  ffi::xproto::gcontext) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_xfixes_create_region_from_picture_checked (c : *mut ffi::base::connection,
                                                         region :  region,
                                                         picture :  ffi::render::picture) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xfixes_create_region_from_picture (c : *mut ffi::base::connection,
                                                 region :  region,
                                                 picture :  ffi::render::picture) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_xfixes_destroy_region_checked (c : *mut ffi::base::connection,
                                             region :  region) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xfixes_destroy_region (c : *mut ffi::base::connection,
                                     region :  region) -> ffi::base::void_cookie;

pub fn xcb_xfixes_set_region_sizeof (_buffer :  *mut c_void,
                              rectangles_len :  u32) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_xfixes_set_region_checked (c : *mut ffi::base::connection,
                                         region :  region,
                                         rectangles_len :  u32,
                                         rectangles : *mut ffi::xproto::rectangle) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xfixes_set_region (c : *mut ffi::base::connection,
                                 region :  region,
                                 rectangles_len :  u32,
                                 rectangles : *mut ffi::xproto::rectangle) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_xfixes_copy_region_checked (c : *mut ffi::base::connection,
                                          source :  region,
                                          destination :  region) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xfixes_copy_region (c : *mut ffi::base::connection,
                                  source :  region,
                                  destination :  region) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_xfixes_union_region_checked (c : *mut ffi::base::connection,
                                           source1 :  region,
                                           source2 :  region,
                                           destination :  region) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xfixes_union_region (c : *mut ffi::base::connection,
                                   source1 :  region,
                                   source2 :  region,
                                   destination :  region) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_xfixes_intersect_region_checked (c : *mut ffi::base::connection,
                                               source1 :  region,
                                               source2 :  region,
                                               destination :  region) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xfixes_intersect_region (c : *mut ffi::base::connection,
                                       source1 :  region,
                                       source2 :  region,
                                       destination :  region) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_xfixes_subtract_region_checked (c : *mut ffi::base::connection,
                                              source1 :  region,
                                              source2 :  region,
                                              destination :  region) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xfixes_subtract_region (c : *mut ffi::base::connection,
                                      source1 :  region,
                                      source2 :  region,
                                      destination :  region) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_xfixes_invert_region_checked (c : *mut ffi::base::connection,
                                            source :  region,
                                            bounds :  ffi::xproto::rectangle,
                                            destination :  region) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xfixes_invert_region (c : *mut ffi::base::connection,
                                    source :  region,
                                    bounds :  ffi::xproto::rectangle,
                                    destination :  region) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_xfixes_translate_region_checked (c : *mut ffi::base::connection,
                                               region :  region,
                                               dx :  i16,
                                               dy :  i16) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xfixes_translate_region (c : *mut ffi::base::connection,
                                       region :  region,
                                       dx :  i16,
                                       dy :  i16) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_xfixes_region_extents_checked (c : *mut ffi::base::connection,
                                             source :  region,
                                             destination :  region) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xfixes_region_extents (c : *mut ffi::base::connection,
                                     source :  region,
                                     destination :  region) -> ffi::base::void_cookie;

pub fn xcb_xfixes_fetch_region_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xfixes_fetch_region (c : *mut ffi::base::connection,
                                   region :  region) -> fetch_region_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_xfixes_fetch_region_unchecked (c : *mut ffi::base::connection,
                                             region :  region) -> fetch_region_cookie;

pub fn xcb_xfixes_fetch_region_rectangles (R : *mut fetch_region_reply) -> *mut ffi::xproto::rectangle;


pub fn xcb_xfixes_fetch_region_rectangles_length (R : *mut fetch_region_reply) -> c_int;

pub fn xcb_xfixes_fetch_region_rectangles_iterator (R : *mut fetch_region_reply) -> ffi::xproto::rectangle_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xfixes_fetch_region_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xfixes_fetch_region_reply (c : *mut ffi::base::connection,
                                         cookie : fetch_region_cookie,
                                         e : *mut *mut ffi::base::generic_error) -> *mut fetch_region_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_xfixes_set_gc_clip_region_checked (c : *mut ffi::base::connection,
                                                 gc :  ffi::xproto::gcontext,
                                                 region :  region,
                                                 x_origin :  i16,
                                                 y_origin :  i16) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xfixes_set_gc_clip_region (c : *mut ffi::base::connection,
                                         gc :  ffi::xproto::gcontext,
                                         region :  region,
                                         x_origin :  i16,
                                         y_origin :  i16) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_xfixes_set_window_shape_region_checked (c : *mut ffi::base::connection,
                                                      dest :  ffi::xproto::window,
                                                      dest_kind :  ffi::shape::kind,
                                                      x_offset :  i16,
                                                      y_offset :  i16,
                                                      region :  region) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xfixes_set_window_shape_region (c : *mut ffi::base::connection,
                                              dest :  ffi::xproto::window,
                                              dest_kind :  ffi::shape::kind,
                                              x_offset :  i16,
                                              y_offset :  i16,
                                              region :  region) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_xfixes_set_picture_clip_region_checked (c : *mut ffi::base::connection,
                                                      picture :  ffi::render::picture,
                                                      region :  region,
                                                      x_origin :  i16,
                                                      y_origin :  i16) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xfixes_set_picture_clip_region (c : *mut ffi::base::connection,
                                              picture :  ffi::render::picture,
                                              region :  region,
                                              x_origin :  i16,
                                              y_origin :  i16) -> ffi::base::void_cookie;

pub fn xcb_xfixes_set_cursor_name_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_xfixes_set_cursor_name_checked (c : *mut ffi::base::connection,
                                              cursor :  ffi::xproto::cursor,
                                              nbytes :  u16,
                                              name : *mut c_char) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xfixes_set_cursor_name (c : *mut ffi::base::connection,
                                      cursor :  ffi::xproto::cursor,
                                      nbytes :  u16,
                                      name : *mut c_char) -> ffi::base::void_cookie;

pub fn xcb_xfixes_get_cursor_name_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xfixes_get_cursor_name (c : *mut ffi::base::connection,
                                      cursor :  ffi::xproto::cursor) -> get_cursor_name_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_xfixes_get_cursor_name_unchecked (c : *mut ffi::base::connection,
                                                cursor :  ffi::xproto::cursor) -> get_cursor_name_cookie;

pub fn xcb_xfixes_get_cursor_name_name (R : *mut get_cursor_name_reply) -> *mut c_char;


pub fn xcb_xfixes_get_cursor_name_name_length (R : *mut get_cursor_name_reply) -> c_int;


pub fn xcb_xfixes_get_cursor_name_name_end (R : *mut get_cursor_name_reply) -> ffi::base::generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xfixes_get_cursor_name_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xfixes_get_cursor_name_reply (c : *mut ffi::base::connection,
                                            cookie : get_cursor_name_cookie,
                                            e : *mut *mut ffi::base::generic_error) -> *mut get_cursor_name_reply;

pub fn xcb_xfixes_get_cursor_image_and_name_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xfixes_get_cursor_image_and_name (c : *mut ffi::base::connection) -> get_cursor_image_and_name_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_xfixes_get_cursor_image_and_name_unchecked (c : *mut ffi::base::connection) -> get_cursor_image_and_name_cookie;

pub fn xcb_xfixes_get_cursor_image_and_name_name (R : *mut get_cursor_image_and_name_reply) -> *mut c_char;


pub fn xcb_xfixes_get_cursor_image_and_name_name_length (R : *mut get_cursor_image_and_name_reply) -> c_int;


pub fn xcb_xfixes_get_cursor_image_and_name_name_end (R : *mut get_cursor_image_and_name_reply) -> ffi::base::generic_iterator;

pub fn xcb_xfixes_get_cursor_image_and_name_cursor_image (R : *mut get_cursor_image_and_name_reply) -> *mut u32;


pub fn xcb_xfixes_get_cursor_image_and_name_cursor_image_length (R : *mut get_cursor_image_and_name_reply) -> c_int;


pub fn xcb_xfixes_get_cursor_image_and_name_cursor_image_end (R : *mut get_cursor_image_and_name_reply) -> ffi::base::generic_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xfixes_get_cursor_image_and_name_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_xfixes_get_cursor_image_and_name_reply (c : *mut ffi::base::connection,
                                                      cookie : get_cursor_image_and_name_cookie,
                                                      e : *mut *mut ffi::base::generic_error) -> *mut get_cursor_image_and_name_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_xfixes_change_cursor_checked (c : *mut ffi::base::connection,
                                            source :  ffi::xproto::cursor,
                                            destination :  ffi::xproto::cursor) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xfixes_change_cursor (c : *mut ffi::base::connection,
                                    source :  ffi::xproto::cursor,
                                    destination :  ffi::xproto::cursor) -> ffi::base::void_cookie;

pub fn xcb_xfixes_change_cursor_by_name_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_xfixes_change_cursor_by_name_checked (c : *mut ffi::base::connection,
                                                    src :  ffi::xproto::cursor,
                                                    nbytes :  u16,
                                                    name : *mut c_char) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xfixes_change_cursor_by_name (c : *mut ffi::base::connection,
                                            src :  ffi::xproto::cursor,
                                            nbytes :  u16,
                                            name : *mut c_char) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_xfixes_expand_region_checked (c : *mut ffi::base::connection,
                                            source :  region,
                                            destination :  region,
                                            left :  u16,
                                            right :  u16,
                                            top :  u16,
                                            bottom :  u16) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xfixes_expand_region (c : *mut ffi::base::connection,
                                    source :  region,
                                    destination :  region,
                                    left :  u16,
                                    right :  u16,
                                    top :  u16,
                                    bottom :  u16) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_xfixes_hide_cursor_checked (c : *mut ffi::base::connection,
                                          window :  ffi::xproto::window) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xfixes_hide_cursor (c : *mut ffi::base::connection,
                                  window :  ffi::xproto::window) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_xfixes_show_cursor_checked (c : *mut ffi::base::connection,
                                          window :  ffi::xproto::window) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_xfixes_show_cursor (c : *mut ffi::base::connection,
                                  window :  ffi::xproto::window) -> ffi::base::void_cookie;
}

