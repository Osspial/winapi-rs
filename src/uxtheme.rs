// Copyright © 2015,
// Licensed under the MIT License <LICENSE.md>
pub const MAX_INTLIST_COUNT: usize = 402;
pub type DTT_CALLBACK_PROC = Option<unsafe extern "system" fn(
    hdc: ::HDC, pszText: ::LPWSTR, cchText: ::c_int, prc: ::LPRECT, dwFlags: ::UINT,
    lParam: ::LPARAM
) -> ::c_int>;

pub type HTHEME = ::HANDLE;
pub type HPAINTBUFFER = ::HANDLE;
pub type HANIMATIONBUFFER = ::HANDLE;

ENUM!{enum TA_PROPERTY {
    TAP_FLAGS,
    TAP_TRANSFORMCOUNT,
    TAP_STAGGERDELAY,
    TAP_STAGGERDELAYCAP,
    TAP_STAGGERDELAYFACTOR,
    TAP_ZORDER,
}}
ENUM!{enum TA_PROPERTY_FLAG {
    TAPF_NONE               = 0x0,
    TAPF_HASSTAGGER         = 0x1,
    TAPF_ISRTLAWARE         = 0x2,
    TAPF_ALLOWCOLLECTION    = 0x4,
    TAPF_HASBACKGROUND      = 0x8,
    TAPF_HASPERSPECTIVE     = 0x10,
}}
ENUM!{enum TA_TRANSFORM_TYPE {
    TATT_TRANSLATE_2D,
    TATT_SCALE_2D,
    TATT_OPACITY,
    TATT_CLIP,
}}
ENUM!{enum TA_TRANSFORM_FLAG {
    TATF_NONE                = 0x0,
    TATF_TARGETVALUES_USER   = 0x1,
    TATF_HASINITIALVALUES    = 0x2,
    TATF_HASORIGINVALUES     = 0x4,
}}
ENUM!{enum TA_TIMINGFUNCTION_TYPE {
    TTFT_UNDEFINED,
    TTFT_CUBIC_BEZIER,
}}
ENUM!{enum THEMESIZE {
    TS_MIN,
    TS_TRUE,
    TS_DRAW,
}}
ENUM!{enum PROPERTYORIGIN {
    PO_STATE,
    PO_PART,
    PO_CLASS,
    PO_GLOBAL,
    PO_NOTFOUND,
}}
ENUM!{enum WINDOWTHEMEATTRIBUTETYPE {
    WTA_NONCLIENT = 1,
}}
ENUM!{enum BP_BUFFERFORMAT {
    BPBF_COMPATIBLEBITMAP,
    BPBF_DIB,
    BPBF_TOPDOWNDIB,
    BPBF_TOPDOWNMONODIB,
}}
ENUM!{enum BP_ANIMATIONSTYLE {
    BPAS_NONE,
    BPAS_LINEAR,
    BPAS_CUBIC,
    BPAS_SINE,
}}

STRUCT!{struct TA_TRANSFORM {
    eTransformType: ::TA_TRANSFORM_TYPE,
    dwTimingFunctionId: ::DWORD,
    dwStartTime: ::DWORD,
    dwDurationTime: ::DWORD,
    eFlags: ::TA_TRANSFORM_FLAG,
}}
STRUCT!{struct TA_TRANSFORM_2D {
    header: ::TA_TRANSFORM,
    rX: ::c_float,
    rY: ::c_float,
    rInitialX: ::c_float,
    rInitialY: ::c_float,
    rOriginX: ::c_float,
    rOriginY: ::c_float,
}}
STRUCT!{struct TA_TRANSFORM_OPACITY {
    header: ::TA_TRANSFORM,
    rOpacity: ::c_float,
    rInitialOpacity: ::c_float,
}}
STRUCT!{struct TA_TRANSFORM_CLIP {
    header: ::TA_TRANSFORM,
    rLeft: ::c_float,
    rTop: ::c_float,
    rRight: ::c_float,
    rBottom: ::c_float,
    rInitialLeft: ::c_float,
    rInitialTop: ::c_float,
    rInitialRight: ::c_float,
    rInitialBottom: ::c_float,
}}
STRUCT!{struct TA_TIMINGFUNCTION {
    eTimingFunctionType: ::TA_TIMINGFUNCTION_TYPE,
}}
STRUCT!{struct TA_CUBIC_BEZIER {
    header: ::TA_TIMINGFUNCTION,
    rX0: ::c_float,
    rY0: ::c_float,
    rX1: ::c_float,
    rY1: ::c_float,
}}
STRUCT!{struct DTBGOPTS {
    dwSize: ::DWORD,
    dwFlags: ::DWORD,
    rcClip: ::RECT,
}}
STRUCT!{struct MARGINS {
    cxLeftWidth: ::c_int,
    cxRightWidth: ::c_int,
    cyTopHeight: ::c_int,
    cyBottomHeight: ::c_int,
}}
#[repr(C)]
pub struct INTLIST {
    iValueCount: ::c_int,
    iValues: [::c_int; MAX_INTLIST_COUNT],
}
STRUCT!{struct WTA_OPTIONS {
    dwFlags: ::DWORD,
    dwMask: ::DWORD,

}}
#[repr(C)] #[derive(Copy)]
pub struct DTTOPTS {
    dwSize: ::DWORD,
    dwFlags: ::DWORD,
    crText: ::COLORREF,
    crBorder: ::COLORREF,
    crShadow: ::COLORREF,
    iTextShadowType: ::c_int,
    ptShadowOffset: ::POINT,
    iBorderSize: ::c_int,
    iFontPropId: ::c_int,
    iColorPropId: ::c_int,
    iStateId: ::c_int,
    fApplyOverlay: ::BOOL,
    iGlowSize: ::c_int,
    pfnDrawTextCallback: ::DTT_CALLBACK_PROC,
    lParam: ::LPARAM,
}
impl Clone for DTTOPTS { fn clone(&self) -> DTTOPTS { *self } }
STRUCT!{struct BP_ANIMATIONPARAMS {
    cbSize: ::DWORD,
    dwFlags: ::DWORD,
    style: ::BP_ANIMATIONSTYLE,
    dwDuration: ::DWORD,
}}
STRUCT!{struct BP_PAINTPARAMS {
    cbSize: ::DWORD,
    dwFlags: ::DWORD,
    prcExclude: *const ::RECT,
    pBlendFunction: *const ::BLENDFUNCTION,
}}