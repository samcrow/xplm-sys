/* automatically generated by rust-bindgen */

use defs::XPLMPluginID;

#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed1 {
    pub structSize: ::libc::c_int,
    pub gearPosition: ::libc::c_float,
    pub flapRatio: ::libc::c_float,
    pub spoilerRatio: ::libc::c_float,
    pub speedBrakeRatio: ::libc::c_float,
    pub slatRatio: ::libc::c_float,
    pub wingSweep: ::libc::c_float,
    pub thrust: ::libc::c_float,
    pub yokePitch: ::libc::c_float,
    pub yokeHeading: ::libc::c_float,
    pub yokeRoll: ::libc::c_float,
}
impl ::std::clone::Clone for Struct_Unnamed1 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed1 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type XPLMPlaneDrawState_t = Struct_Unnamed1;
pub type XPLMPlanesAvailable_f =
    ::std::option::Option<unsafe extern "C" fn(inRefcon: *mut ::libc::c_void)
                              -> ()>;
extern "C" {
    pub fn XPLMSetUsersAircraft(inAircraftPath: *const ::libc::c_char) -> ();
    pub fn XPLMPlaceUserAtAirport(inAirportCode: *const ::libc::c_char) -> ();
    pub fn XPLMCountAircraft(outTotalAircraft: *mut ::libc::c_int,
                             outActiveAircraft: *mut ::libc::c_int,
                             outController: *mut XPLMPluginID) -> ();
    pub fn XPLMGetNthAircraftModel(inIndex: ::libc::c_int,
                                   outFileName: *mut ::libc::c_char,
                                   outPath: *mut ::libc::c_char) -> ();
    pub fn XPLMAcquirePlanes(inAircraft: *mut *mut ::libc::c_char,
                             inCallback: XPLMPlanesAvailable_f,
                             inRefcon: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn XPLMReleasePlanes() -> ();
    pub fn XPLMSetActiveAircraftCount(inCount: ::libc::c_int) -> ();
    pub fn XPLMSetAircraftModel(inIndex: ::libc::c_int,
                                inAircraftPath: *const ::libc::c_char) -> ();
    pub fn XPLMDisableAIForPlane(inPlaneIndex: ::libc::c_int) -> ();
    pub fn XPLMDrawAircraft(inPlaneIndex: ::libc::c_int, inX: ::libc::c_float,
                            inY: ::libc::c_float, inZ: ::libc::c_float,
                            inPitch: ::libc::c_float, inRoll: ::libc::c_float,
                            inYaw: ::libc::c_float, inFullDraw: ::libc::c_int,
                            inDrawStateInfo: *mut XPLMPlaneDrawState_t) -> ();
    pub fn XPLMReinitUsersPlane() -> ();
}