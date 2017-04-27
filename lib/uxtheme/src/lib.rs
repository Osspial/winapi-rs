// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to uxtheme.
#![no_std]
extern crate winapi;
use winapi::*;
extern "system" {
    pub fn BeginPanningFeedback(hwnd: HWND) -> BOOL;
    pub fn UpdatePanningFeedback(
        hwnd: HWND, lTotalOverpanOffsetX: LONG, lTotalOverpanOffsetY: LONG, fInInertia: BOOL
    ) -> BOOL;
    pub fn EndPanningFeedback(hwnd: HWND, fAnimateBack: BOOL) -> BOOL;
    pub fn GetThemeAnimationProperty(
        hTheme: HTHEME, iStoryboardId: c_int, iTargetId: c_int, eProperty: TA_PROPERTY,
        pvProperty: *mut VOID, cbSize: DWORD, pcbSizeOut: *mut DWORD
    ) -> HRESULT;
    pub fn GetThemeAnimationTransform(
        hTheme: HTHEME, iStoryboardId: c_int, iTargetId: c_int, dwTransformIndex: DWORD,
        pTransform: *mut TA_TRANSFORM, cbSize: DWORD, pcbSizeOut: *mut DWORD
    ) -> HRESULT;
    pub fn GetThemeTimingFunction(
        hTheme: HTHEME, iTimingFunctionId: c_int, pTimingFunction: *mut TA_TIMINGFUNCTION,
        cbSize: DWORD, pcbSizeOut: *mut DWORD
    ) -> HRESULT;
    pub fn OpenThemeData(hwnd: HWND, pszClassList: LPCWSTR) -> HTHEME;
    pub fn OpenThemeDataEx(hwnd: HWND, pszClassList: LPCWSTR, dwFlags: DWORD) -> HTHEME;
    pub fn CloseThemeData(hTheme: HTHEME) -> HRESULT;
    pub fn DrawThemeBackground(
        hTheme: HTHEME, hdc: HDC, iPartId: c_int, iStateId: c_int, pRect: LPCRECT,
        pClipRect: LPCRECT
    ) -> HRESULT;
    pub fn DrawThemeBackgroundEx(
        hTheme: HTHEME, hdc: HDC, iPartId: c_int, iStateId: c_int, pRect: LPCRECT,
        pOptions: *const DTBGOPTS
    ) -> HRESULT;
    pub fn DrawThemeText(
        hTheme: HTHEME, hdc: HDC, iPartId: c_int, iStateId: c_int, pszText: LPCWSTR, cchText: c_int,
        dwTextFlags: DWORD, dwTextFlags2: DWORD, pRect: LPCRECT
    ) -> HRESULT;
    pub fn GetThemeBackgroundContentRect(
        hTheme: HTHEME, hdc: HDC, iPartId: c_int, iStateId: c_int, pBoundingRect: LPCRECT,
        pContentRect: LPRECT
    ) -> HRESULT;
    pub fn GetThemeBackgroundExtent(
        hTheme: HTHEME, hdc: HDC, iPartId: c_int, iStateId: c_int, pContentRect: LPCRECT,
        pExtentRect: LPRECT
    ) -> HRESULT;
    pub fn GetThemeBackgroundRegion(
        hTheme: HTHEME, hdc: HDC, iPartId: c_int, iStateId: c_int, pRect: LPCRECT,
        pRegion: *mut HRGN
    ) -> HRESULT;
    pub fn GetThemePartSize(
        hTheme: HTHEME, hdc: HDC, iPartId: c_int, iStateId: c_int, prc: LPCRECT, eSize: THEMESIZE,
        psz: *mut SIZE
    ) -> HRESULT;
    pub fn GetThemeTextExtent(
        hTheme: HTHEME, hdc: HDC, iPartId: c_int, iStateId: c_int, pszText: LPCWSTR,
        cchCharCount: c_int, dwTextFlags: DWORD, pBoundingRect: LPCRECT, pExtentRect: LPRECT
    ) -> HRESULT;
    pub fn GetThemeTextMetrics(
        hTheme: HTHEME, hdc: HDC, iPartId: c_int, iStateId: c_int, ptm: *mut TEXTMETRICW
    ) -> HRESULT;
    pub fn HitTestThemeBackground(
        hTheme: HTHEME, hdc: HDC, iPartId: c_int, iStateId: c_int, dwOptions: DWORD, pRect: LPCRECT,
        hrgn: HRGN, ptTest: POINT, pwHitTestCode: *mut WORD
    ) -> HRESULT;
    pub fn DrawThemeEdge(
        hTheme: HTHEME, hdc: HDC, iPartId: c_int, iStateId: c_int, pDestRect: LPCRECT, uEdge: UINT,
        uFlags: UINT, pContentRect: LPRECT
    )-> HRESULT;
    pub fn DrawThemeIcon(
        hTheme: HTHEME, hdc: HDC, iPartId: c_int, iStateId: c_int, pRect: LPCRECT, himl: HIMAGELIST,
        iImageIndex: c_int
    ) -> HRESULT;
    pub fn IsThemePartDefined(hTheme: HTHEME, iPartId: c_int, iStateId: c_int) -> BOOL;
    pub fn IsThemeBackgroundPartiallyTransparent(
        hTheme: HTHEME, iPartId: c_int, iStateId: c_int
    ) -> BOOL;
    pub fn GetThemeColor(
        hTheme: HTHEME, iPartId: c_int, iStateId: c_int, iPropId: c_int, pColor: *mut COLORREF
    ) -> HRESULT;
    pub fn GetThemeMetric(
        hTheme: HTHEME, hdc: HDC, iPartId: c_int, iStateId: c_int, iPropId: c_int,
        piVal: *mut c_int
    ) -> HRESULT;
    pub fn GetThemeString(
        hTheme: HTHEME, iPartId: c_int, iStateId: c_int, iPropId: c_int, pszBuff: LPWSTR,
        cchMaxBuffChars: c_int
    ) -> HRESULT;
    pub fn GetThemeBool(
        hTheme: HTHEME, iPartId: c_int, iStateId: c_int, iPropId: c_int, pfVal: *mut BOOL
    ) -> HRESULT;
    pub fn GetThemeInt(
        hTheme: HTHEME, iPartId: c_int, iStateId: c_int, iPropId: c_int, piVal: *mut c_int
    ) -> HRESULT;
    pub fn GetThemeEnumValue(
        hTheme: HTHEME, iPartId: c_int, iStateId: c_int, iPropId: c_int, piVal: *mut c_int
    ) -> HRESULT;
    pub fn GetThemePosition(
        hTheme: HTHEME, iPartId: c_int, iStateId: c_int, iPropId: c_int, pPoint: *mut POINT
    ) -> HRESULT;
    pub fn GetThemeFont(
        hTheme: HTHEME, hdc: HDC, iPartId: c_int, iStateId: c_int, iPropId: c_int,
        pFont: *mut LOGFONTW
    ) -> HRESULT;
    pub fn GetThemeRect(
        hTheme: HTHEME, iPartId: c_int, iStateId: c_int, iPropId: c_int, pRect: LPRECT
    ) -> HRESULT;
    pub fn GetThemeMargins(
        hTheme: HTHEME, hdc: HDC, iPartId: c_int, iStateId: c_int, iPropId: c_int, prc: LPCRECT,
        pMargins: *mut MARGINS
    ) -> HRESULT;
    pub fn GetThemeIntList(
        hTheme: HTHEME, iPartId: c_int, iStateId: c_int, iPropId: c_int, pIntList: *mut INTLIST
    ) -> HRESULT;
    pub fn GetThemePropertyOrigin(
        hTheme: HTHEME, iPartId: c_int, iStateId: c_int, iPropId: c_int,
        pOrigin: *mut PROPERTYORIGIN
    ) -> HRESULT;
    pub fn SetWindowTheme(hwnd: HWND, pszSubAppName: LPCWSTR, pszSubIdList: LPCWSTR) -> HRESULT;
    pub fn GetThemeFilename(
        hTheme: HTHEME, iPartId: c_int, iStateId: c_int, iPropId: c_int, pszThemeFileName: LPWSTR,
        cchMaxBuffChars: c_int
    ) -> HRESULT;
    pub fn GetThemeSysColor(hTheme: HTHEME, iColorId: c_int) -> COLORREF;
    pub fn GetThemeSysColorBrush(hTheme: HTHEME, iColorId: c_int) -> HBRUSH;
    pub fn GetThemeSysBool(hTheme: HTHEME, iBoolId: c_int) -> BOOL;
    pub fn GetThemeSysSize(hTheme: HTHEME, iSizeId: c_int) -> c_int;
    pub fn GetThemeSysFont(hTheme: HTHEME, iFontId: c_int, plf: *mut LOGFONTW) -> HRESULT;
    pub fn GetThemeSysString(
        hTheme: HTHEME, iStringId: c_int, pszStringBuff: LPWSTR, cchMaxStringChars: c_int
    ) -> HRESULT;
    pub fn GetThemeSysInt(hTheme: HTHEME, iIntId: c_int, piValue: *mut c_int) -> HRESULT;
    pub fn IsThemeActive() -> BOOL;
    pub fn IsAppThemed() -> BOOL;
    pub fn GetWindowTheme(hwnd: HWND) -> HTHEME;
    pub fn EnableThemeDialogTexture(hwnd: HWND, dwFlags: DWORD) -> HRESULT;
    pub fn IsThemeDialogTextureEnabled(hwnd: HWND) -> BOOL;
    pub fn GetThemeAppProperties() -> DWORD;
    pub fn SetThemeAppProperties(dwFlags: DWORD);
    pub fn GetCurrentThemeName(
        pszThemeFileName: LPWSTR, cchMaxNameChars: c_int, pszColorBuff: LPWSTR,
        cchMaxColorChars: c_int, pszSizeBuff: LPWSTR, cchMaxSizeChars: c_int
    ) -> HRESULT;
    pub fn GetThemeDocumentationProperty(
        pszThemeName: LPCWSTR, pszPropertyName: LPCWSTR, pszValueBuff: LPWSTR,
        cchMaxValChars: c_int
    ) -> HRESULT;
    pub fn DrawThemeParentBackground(hwnd: HWND, hdc: HDC, prc: *const RECT) -> HRESULT;
    pub fn EnableTheming(fEnable: BOOL) -> HRESULT;
    pub fn DrawThemeParentBackgroundEx(
        hwnd: HWND, hdc: HDC, dwFlags: DWORD, prc: *const RECT
    ) -> HRESULT;
    pub fn SetWindowThemeAttribute(
        hwnd: HWND, eAttribute: WINDOWTHEMEATTRIBUTETYPE, pvAttribute: PVOID, cbAttribute: DWORD
    ) -> HRESULT;
    pub fn DrawThemeTextEx(
        hTheme: HTHEME, hdc: HDC, iPartId: c_int, iStateId: c_int, pszText: LPCWSTR, cchText: c_int,
        dwTextFlags: DWORD, pRect: LPRECT, pOptions: *const DTTOPTS
    ) -> HRESULT;
    pub fn GetThemeBitmap(
        hTheme: HTHEME, iPartId: c_int, iStateId: c_int, iPropId: c_int, dwFlags: ULONG,
        phBitmap: *mut HBITMAP
    ) -> HRESULT;
    pub fn GetThemeStream(
        hTheme: HTHEME, iPartId: c_int, iStateId: c_int, iPropId: c_int, ppvStream: *mut *mut VOID,
        pcbStream: *mut DWORD, hInst: HINSTANCE
    ) -> HRESULT;
    pub fn BufferedPaintInit() -> HRESULT;
    pub fn BufferedPaintUnInit() -> HRESULT;
    pub fn BeginBufferedPaint(
        hdcTarget: HDC, prcTarget: *const RECT, dwFormat: BP_BUFFERFORMAT,
        pPaintParams: *mut BP_PAINTPARAMS, phdc: *mut HDC
    ) -> HPAINTBUFFER;
    pub fn EndBufferedPaint(hBufferedPaint: HPAINTBUFFER, fUpdateTarget: BOOL) -> HRESULT;
    pub fn GetBufferedPaintTargetRect(hBufferedPaint: HPAINTBUFFER, prc: *mut RECT) -> HRESULT;
    pub fn GetBufferedPaintTargetDC(hBufferedPaint: HPAINTBUFFER) -> HDC;
    pub fn GetBufferedPaintDC(hBufferedPaint: HPAINTBUFFER) -> HDC;
    pub fn GetBufferedPaintBits(
        hBufferedPaint: HPAINTBUFFER, ppbBuffer: *mut *mut RGBQUAD, pcxRow: *mut c_int
    ) -> HRESULT;
    pub fn BufferedPaintClear(hBufferedPaint: HPAINTBUFFER, prc: *const RECT) -> HRESULT;
    pub fn BufferedPaintSetAlpha(
        hBufferedPaint: HPAINTBUFFER, prc: *const RECT, alpha: BYTE
    ) -> HRESULT;
    pub fn BufferedPaintStopAllAnimations(hwnd: HWND) -> HRESULT;
    pub fn BeginBufferedAnimation(
        hwnd: HWND, hdcTarget: HDC, prcTarget: *const RECT, dwFormat: BP_BUFFERFORMAT,
        pPaintParams: *mut BP_PAINTPARAMS, pAnimationParams: *mut BP_ANIMATIONPARAMS,
        phdcFrom: *mut HDC, phdcTo: *mut HDC
    ) -> HANIMATIONBUFFER;
    pub fn EndBufferedAnimation(hbpAnimation: HANIMATIONBUFFER, fUpdateTarget: BOOL) -> HRESULT;
    pub fn BufferedPaintRenderAnimation(hwnd: HWND, hdcTarget: HDC) -> BOOL;
    pub fn IsCompositionActive() -> BOOL;
    pub fn GetThemeTransitionDuration(
        hTheme: HTHEME, iPartId: c_int, iStateIdFrom: c_int, iStateIdTo: c_int, iPropId: c_int,
        pdwDuration: *mut DWORD
    ) -> HRESULT;
}
