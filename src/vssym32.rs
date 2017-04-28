ENUM!{enum BGTYPE {
    BT_IMAGEFILE = 0,
    BT_BORDERFILL = 1,
    BT_NONE = 2,
}}
ENUM!{enum IMAGELAYOUT {
    IL_VERTICAL = 0,
    IL_HORIZONTAL = 1,
}}
ENUM!{enum BORDERTYPE {
    BT_RECT = 0,
    BT_ROUNDRECT = 1,
    BT_ELLIPSE = 2,
}}
ENUM!{enum FILLTYPE {
    FT_SOLID = 0,
    FT_VERTGRADIENT = 1,
    FT_HORZGRADIENT = 2,
    FT_RADIALGRADIENT = 3,
    FT_TILEIMAGE = 4,
}}
ENUM!{enum SIZINGTYPE {
    ST_TRUESIZE = 0,
    ST_STRETCH = 1,
    ST_TILE = 2,
}}
ENUM!{enum HALIGN {
    HA_LEFT = 0,
    HA_CENTER = 1,
    HA_RIGHT = 2,
}}
ENUM!{enum CONTENTALIGNMENT {
    CA_LEFT = 0,
    CA_CENTER = 1,
    CA_RIGHT = 2,
}}
ENUM!{enum VALIGN {
    VA_TOP = 0,
    VA_CENTER = 1,
    VA_BOTTOM = 2,
}}
ENUM!{enum OFFSETTYPE {
    OT_TOPLEFT = 0,
    OT_TOPRIGHT = 1,
    OT_TOPMIDDLE = 2,
    OT_BOTTOMLEFT = 3,
    OT_BOTTOMRIGHT = 4,
    OT_BOTTOMMIDDLE = 5,
    OT_MIDDLELEFT = 6,
    OT_MIDDLERIGHT = 7,
    OT_LEFTOFCAPTION = 8,
    OT_RIGHTOFCAPTION = 9,
    OT_LEFTOFLASTBUTTON = 10,
    OT_RIGHTOFLASTBUTTON = 11,
    OT_ABOVELASTBUTTON = 12,
    OT_BELOWLASTBUTTON = 13,
}}
ENUM!{enum ICONEFFECT {
    ICE_NONE = 0,
    ICE_GLOW = 1,
    ICE_SHADOW = 2,
    ICE_PULSE = 3,
    ICE_ALPHA = 4,
}}
ENUM!{enum TEXTSHADOWTYPE {
    TST_NONE = 0,
    TST_SINGLE = 1,
    TST_CONTINUOUS = 2,
}}
ENUM!{enum GLYPHTYPE {
    GT_NONE = 0,
    GT_IMAGEGLYPH = 1,
    GT_FONTGLYPH = 2,
}}
ENUM!{enum IMAGESELECTTYPE {
    IST_NONE = 0,
    IST_SIZE = 1,
    IST_DPI = 2,
}}
ENUM!{enum TRUESIZESCALINGTYPE {
    TSST_NONE = 0,
    TSST_SIZE = 1,
    TSST_DPI = 2,
}}
ENUM!{enum GLYPHFONTSIZINGTYPE {
    GFST_NONE = 0,
    GFST_SIZE = 1,
    GFST_DPI = 2,
}}
ENUM!{enum LINKPARTS {
    LP_HYPERLINK = 1,
}}
ENUM!{enum HYPERLINKSTATES {
    HLS_NORMALTEXT = 1,
    HLS_LINKTEXT = 2,
}}
ENUM!{enum EMPTYMARKUPPARTS {
    EMP_MARKUPTEXT = 1,
}}
ENUM!{enum MARKUPTEXTSTATES {
    EMT_NORMALTEXT = 1,
    EMT_LINKTEXT = 2,
}}
ENUM!{enum STATICPARTS {
    STAT_TEXT = 1,
}}
ENUM!{enum PAGEPARTS {
    PGRP_UP = 1,
    PGRP_DOWN = 2,
    PGRP_UPHORZ = 3,
    PGRP_DOWNHORZ = 4,
}}
ENUM!{enum MONTHCALPARTS {
    MC_BACKGROUND = 1,
    MC_BORDERS = 2,
    MC_GRIDBACKGROUND = 3,
    MC_COLHEADERSPLITTER = 4,
    MC_GRIDCELLBACKGROUND = 5,
    MC_GRIDCELL = 6,
    MC_GRIDCELLUPPER = 7,
    MC_TRAILINGGRIDCELL = 8,
    MC_TRAILINGGRIDCELLUPPER = 9,
    MC_NAVNEXT = 10,
    MC_NAVPREV = 11,
}}
ENUM!{enum GRIDCELLBACKGROUNDSTATES {
    MCGCB_SELECTED = 1,
    MCGCB_HOT = 2,
    MCGCB_SELECTEDHOT = 3,
    MCGCB_SELECTEDNOTFOCUSED = 4,
    MCGCB_TODAY = 5,
    MCGCB_TODAYSELECTED = 6,
}}
ENUM!{enum GRIDCELLSTATES {
    MCGC_HOT = 1,
    MCGC_HASSTATE = 2,
    MCGC_HASSTATEHOT = 3,
    MCGC_TODAY = 4,
    MCGC_TODAYSELECTED = 5,
    MCGC_SELECTED = 6,
    MCGC_SELECTEDHOT = 7,
}}
ENUM!{enum GRIDCELLUPPERSTATES {
    MCGCU_HOT = 1,
    MCGCU_HASSTATE = 2,
    MCGCU_HASSTATEHOT = 3,
    MCGCU_SELECTED = 4,
    MCGCU_SELECTEDHOT = 5,
}}
ENUM!{enum TRAILINGGRIDCELLSTATES {
    MCTGC_HOT = 1,
    MCTGC_HASSTATE = 2,
    MCTGC_HASSTATEHOT = 3,
    MCTGC_TODAY = 4,
    MCTGC_TODAYSELECTED = 5,
    MCTGC_SELECTED = 6,
    MCTGC_SELECTEDHOT = 7,
}}
ENUM!{enum TRAILINGGRIDCELLUPPERSTATES {
    MCTGCU_HOT = 1,
    MCTGCU_HASSTATE = 2,
    MCTGCU_HASSTATEHOT = 3,
    MCTGCU_SELECTED = 4,
    MCTGCU_SELECTEDHOT = 5,
}}
ENUM!{enum NAVNEXTSTATES {
    MCNN_NORMAL = 1,
    MCNN_HOT = 2,
    MCNN_PRESSED = 3,
    MCNN_DISABLED = 4,
}}
ENUM!{enum NAVPREVSTATES {
    MCNP_NORMAL = 1,
    MCNP_HOT = 2,
    MCNP_PRESSED = 3,
    MCNP_DISABLED = 4,
}}
ENUM!{enum CLOCKPARTS {
    CLP_TIME = 1,
}}
ENUM!{enum CLOCKSTATES {
    CLS_NORMAL = 1,
    CLS_HOT = 2,
    CLS_PRESSED = 3,
}}
ENUM!{enum TRAYNOTIFYPARTS {
    TNP_BACKGROUND = 1,
    TNP_ANIMBACKGROUND = 2,
}}
ENUM!{enum TASKBARPARTS {
    TBP_BACKGROUNDBOTTOM = 1,
    TBP_BACKGROUNDRIGHT = 2,
    TBP_BACKGROUNDTOP = 3,
    TBP_BACKGROUNDLEFT = 4,
    TBP_SIZINGBARBOTTOM = 5,
    TBP_SIZINGBARRIGHT = 6,
    TBP_SIZINGBARTOP = 7,
    TBP_SIZINGBARLEFT = 8,
}}
ENUM!{enum TASKBANDPARTS {
    TDP_GROUPCOUNT = 1,
    TDP_FLASHBUTTON = 2,
    TDP_FLASHBUTTONGROUPMENU = 3,
}}
ENUM!{enum STARTPANELPARTS {
    SPP_USERPANE = 1,
    SPP_MOREPROGRAMS = 2,
    SPP_MOREPROGRAMSARROW = 3,
    SPP_PROGLIST = 4,
    SPP_PROGLISTSEPARATOR = 5,
    SPP_PLACESLIST = 6,
    SPP_PLACESLISTSEPARATOR = 7,
    SPP_LOGOFF = 8,
    SPP_LOGOFFBUTTONS = 9,
    SPP_USERPICTURE = 10,
    SPP_PREVIEW = 11,
    SPP_MOREPROGRAMSTAB = 12,
    SPP_NSCHOST = 13,
    SPP_SOFTWAREEXPLORER = 14,
    SPP_OPENBOX = 15,
    SPP_SEARCHVIEW = 16,
    SPP_MOREPROGRAMSARROWBACK = 17,
    SPP_TOPMATCH = 18,
    SPP_LOGOFFSPLITBUTTONDROPDOWN = 19,
}}
ENUM!{enum MOREPROGRAMSTABSTATES {
    SPMPT_NORMAL = 1,
    SPMPT_HOT = 2,
    SPMPT_SELECTED = 3,
    SPMPT_DISABLED = 4,
    SPMPT_FOCUSED = 5,
}}
ENUM!{enum SOFTWAREEXPLORERSTATES {
    SPSE_NORMAL = 1,
    SPSE_HOT = 2,
    SPSE_SELECTED = 3,
    SPSE_DISABLED = 4,
    SPSE_FOCUSED = 5,
}}
ENUM!{enum OPENBOXSTATES {
    SPOB_NORMAL = 1,
    SPOB_HOT = 2,
    SPOB_SELECTED = 3,
    SPOB_DISABLED = 4,
    SPOB_FOCUSED = 5,
}}
ENUM!{enum MOREPROGRAMSARROWSTATES {
    SPS_NORMAL = 1,
    SPS_HOT = 2,
    SPS_PRESSED = 3,
}}
ENUM!{enum MOREPROGRAMSARROWBACKSTATES {
    SPSB_NORMAL = 1,
    SPSB_HOT = 2,
    SPSB_PRESSED = 3,
}}
ENUM!{enum LOGOFFBUTTONSSTATES {
    SPLS_NORMAL = 1,
    SPLS_HOT = 2,
    SPLS_PRESSED = 3,
}}
ENUM!{enum MENUBANDPARTS {
    MDP_NEWAPPBUTTON = 1,
    MDP_SEPERATOR = 2,
}}
ENUM!{enum MENUBANDSTATES {
    MDS_NORMAL = 1,
    MDS_HOT = 2,
    MDS_PRESSED = 3,
    MDS_DISABLED = 4,
    MDS_CHECKED = 5,
    MDS_HOTCHECKED = 6,
}}

pub const TMT_DIBDATA: ::c_int = 2;
pub const TMT_GLYPHDIBDATA: ::c_int = 8;
pub const TMT_ENUM: ::c_int = 200;
pub const TMT_STRING: ::c_int = 201;
pub const TMT_INT: ::c_int = 202;
pub const TMT_BOOL: ::c_int = 203;
pub const TMT_COLOR: ::c_int = 204;
pub const TMT_MARGINS: ::c_int = 205;
pub const TMT_FILENAME: ::c_int = 206;
pub const TMT_SIZE: ::c_int = 207;
pub const TMT_POSITION: ::c_int = 208;
pub const TMT_RECT: ::c_int = 209;
pub const TMT_FONT: ::c_int = 210;
pub const TMT_INTLIST: ::c_int = 211;
pub const TMT_HBITMAP: ::c_int = 212;
pub const TMT_DISKSTREAM: ::c_int = 213;
pub const TMT_STREAM: ::c_int = 214;
pub const TMT_BITMAPREF: ::c_int = 215;
pub const TMT_FLOAT: ::c_int = 216;
pub const TMT_FLOATLIST: ::c_int = 217;
pub const TMT_COLORSCHEMES: ::c_int = 401;
pub const TMT_SIZES: ::c_int = 402;
pub const TMT_CHARSET: ::c_int = 403;
pub const TMT_NAME: ::c_int = 600;
pub const TMT_DISPLAYNAME: ::c_int = 601;
pub const TMT_TOOLTIP: ::c_int = 602;
pub const TMT_COMPANY: ::c_int = 603;
pub const TMT_AUTHOR: ::c_int = 604;
pub const TMT_COPYRIGHT: ::c_int = 605;
pub const TMT_URL: ::c_int = 606;
pub const TMT_VERSION: ::c_int = 607;
pub const TMT_DESCRIPTION: ::c_int = 608;
pub const TMT_FIRST_RCSTRING_NAME: ::c_int = TMT_DISPLAYNAME;
pub const TMT_LAST_RCSTRING_NAME: ::c_int = TMT_DESCRIPTION;
pub const TMT_CAPTIONFONT: ::c_int = 801;
pub const TMT_SMALLCAPTIONFONT: ::c_int = 802;
pub const TMT_MENUFONT: ::c_int = 803;
pub const TMT_STATUSFONT: ::c_int = 804;
pub const TMT_MSGBOXFONT: ::c_int = 805;
pub const TMT_ICONTITLEFONT: ::c_int = 806;
pub const TMT_HEADING1FONT: ::c_int = 807;
pub const TMT_HEADING2FONT: ::c_int = 808;
pub const TMT_BODYFONT: ::c_int = 809;
pub const TMT_FIRSTFONT: ::c_int = TMT_CAPTIONFONT;
pub const TMT_LASTFONT: ::c_int = TMT_BODYFONT;
pub const TMT_FLATMENUS: ::c_int = 1001;
pub const TMT_FIRSTBOOL: ::c_int = TMT_FLATMENUS;
pub const TMT_LASTBOOL: ::c_int = TMT_FLATMENUS;
pub const TMT_SIZINGBORDERWIDTH: ::c_int = 1201;
pub const TMT_SCROLLBARWIDTH: ::c_int = 1202;
pub const TMT_SCROLLBARHEIGHT: ::c_int = 1203;
pub const TMT_CAPTIONBARWIDTH: ::c_int = 1204;
pub const TMT_CAPTIONBARHEIGHT: ::c_int = 1205;
pub const TMT_SMCAPTIONBARWIDTH: ::c_int = 1206;
pub const TMT_SMCAPTIONBARHEIGHT: ::c_int = 1207;
pub const TMT_MENUBARWIDTH: ::c_int = 1208;
pub const TMT_MENUBARHEIGHT: ::c_int = 1209;
pub const TMT_PADDEDBORDERWIDTH: ::c_int = 1210;
pub const TMT_FIRSTSIZE: ::c_int = TMT_SIZINGBORDERWIDTH;
pub const TMT_LASTSIZE: ::c_int = TMT_PADDEDBORDERWIDTH;
pub const TMT_MINCOLORDEPTH: ::c_int = 1301;
pub const TMT_FIRSTINT: ::c_int = TMT_MINCOLORDEPTH;
pub const TMT_LASTINT: ::c_int = TMT_MINCOLORDEPTH;
pub const TMT_CSSNAME: ::c_int = 1401;
pub const TMT_XMLNAME: ::c_int = 1402;
pub const TMT_LASTUPDATED: ::c_int = 1403;
pub const TMT_ALIAS: ::c_int = 1404;
pub const TMT_FIRSTSTRING: ::c_int = TMT_CSSNAME;
pub const TMT_LASTSTRING: ::c_int = TMT_ALIAS;
pub const TMT_SCROLLBAR: ::c_int = 1601;
pub const TMT_BACKGROUND: ::c_int = 1602;
pub const TMT_ACTIVECAPTION: ::c_int = 1603;
pub const TMT_INACTIVECAPTION: ::c_int = 1604;
pub const TMT_MENU: ::c_int = 1605;
pub const TMT_WINDOW: ::c_int = 1606;
pub const TMT_WINDOWFRAME: ::c_int = 1607;
pub const TMT_MENUTEXT: ::c_int = 1608;
pub const TMT_WINDOWTEXT: ::c_int = 1609;
pub const TMT_CAPTIONTEXT: ::c_int = 1610;
pub const TMT_ACTIVEBORDER: ::c_int = 1611;
pub const TMT_INACTIVEBORDER: ::c_int = 1612;
pub const TMT_APPWORKSPACE: ::c_int = 1613;
pub const TMT_HIGHLIGHT: ::c_int = 1614;
pub const TMT_HIGHLIGHTTEXT: ::c_int = 1615;
pub const TMT_BTNFACE: ::c_int = 1616;
pub const TMT_BTNSHADOW: ::c_int = 1617;
pub const TMT_GRAYTEXT: ::c_int = 1618;
pub const TMT_BTNTEXT: ::c_int = 1619;
pub const TMT_INACTIVECAPTIONTEXT: ::c_int = 1620;
pub const TMT_BTNHIGHLIGHT: ::c_int = 1621;
pub const TMT_DKSHADOW3D: ::c_int = 1622;
pub const TMT_LIGHT3D: ::c_int = 1623;
pub const TMT_INFOTEXT: ::c_int = 1624;
pub const TMT_INFOBK: ::c_int = 1625;
pub const TMT_BUTTONALTERNATEFACE: ::c_int = 1626;
pub const TMT_HOTTRACKING: ::c_int = 1627;
pub const TMT_GRADIENTACTIVECAPTION: ::c_int = 1628;
pub const TMT_GRADIENTINACTIVECAPTION: ::c_int = 1629;
pub const TMT_MENUHILIGHT: ::c_int = 1630;
pub const TMT_MENUBAR: ::c_int = 1631;
pub const TMT_FIRSTCOLOR: ::c_int = TMT_SCROLLBAR;
pub const TMT_LASTCOLOR: ::c_int = TMT_MENUBAR;
pub const TMT_FROMHUE1: ::c_int = 1801;
pub const TMT_FROMHUE2: ::c_int = 1802;
pub const TMT_FROMHUE3: ::c_int = 1803;
pub const TMT_FROMHUE4: ::c_int = 1804;
pub const TMT_FROMHUE5: ::c_int = 1805;
pub const TMT_TOHUE1: ::c_int = 1806;
pub const TMT_TOHUE2: ::c_int = 1807;
pub const TMT_TOHUE3: ::c_int = 1808;
pub const TMT_TOHUE4: ::c_int = 1809;
pub const TMT_TOHUE5: ::c_int = 1810;
pub const TMT_FROMCOLOR1: ::c_int = 2001;
pub const TMT_FROMCOLOR2: ::c_int = 2002;
pub const TMT_FROMCOLOR3: ::c_int = 2003;
pub const TMT_FROMCOLOR4: ::c_int = 2004;
pub const TMT_FROMCOLOR5: ::c_int = 2005;
pub const TMT_TOCOLOR1: ::c_int = 2006;
pub const TMT_TOCOLOR2: ::c_int = 2007;
pub const TMT_TOCOLOR3: ::c_int = 2008;
pub const TMT_TOCOLOR4: ::c_int = 2009;
pub const TMT_TOCOLOR5: ::c_int = 2010;
pub const TMT_TRANSPARENT: ::c_int = 2201;
pub const TMT_AUTOSIZE: ::c_int = 2202;
pub const TMT_BORDERONLY: ::c_int = 2203;
pub const TMT_COMPOSITED: ::c_int = 2204;
pub const TMT_BGFILL: ::c_int = 2205;
pub const TMT_GLYPHTRANSPARENT: ::c_int = 2206;
pub const TMT_GLYPHONLY: ::c_int = 2207;
pub const TMT_ALWAYSSHOWSIZINGBAR: ::c_int = 2208;
pub const TMT_MIRRORIMAGE: ::c_int = 2209;
pub const TMT_UNIFORMSIZING: ::c_int = 2210;
pub const TMT_INTEGRALSIZING: ::c_int = 2211;
pub const TMT_SOURCEGROW: ::c_int = 2212;
pub const TMT_SOURCESHRINK: ::c_int = 2213;
pub const TMT_DRAWBORDERS: ::c_int = 2214;
pub const TMT_NOETCHEDEFFECT: ::c_int = 2215;
pub const TMT_TEXTAPPLYOVERLAY: ::c_int = 2216;
pub const TMT_TEXTGLOW: ::c_int = 2217;
pub const TMT_TEXTITALIC: ::c_int = 2218;
pub const TMT_COMPOSITEDOPAQUE: ::c_int = 2219;
pub const TMT_LOCALIZEDMIRRORIMAGE: ::c_int = 2220;
pub const TMT_IMAGECOUNT: ::c_int = 2401;
pub const TMT_ALPHALEVEL: ::c_int = 2402;
pub const TMT_BORDERSIZE: ::c_int = 2403;
pub const TMT_ROUNDCORNERWIDTH: ::c_int = 2404;
pub const TMT_ROUNDCORNERHEIGHT: ::c_int = 2405;
pub const TMT_GRADIENTRATIO1: ::c_int = 2406;
pub const TMT_GRADIENTRATIO2: ::c_int = 2407;
pub const TMT_GRADIENTRATIO3: ::c_int = 2408;
pub const TMT_GRADIENTRATIO4: ::c_int = 2409;
pub const TMT_GRADIENTRATIO5: ::c_int = 2410;
pub const TMT_PROGRESSCHUNKSIZE: ::c_int = 2411;
pub const TMT_PROGRESSSPACESIZE: ::c_int = 2412;
pub const TMT_SATURATION: ::c_int = 2413;
pub const TMT_TEXTBORDERSIZE: ::c_int = 2414;
pub const TMT_ALPHATHRESHOLD: ::c_int = 2415;
pub const TMT_WIDTH: ::c_int = 2416;
pub const TMT_HEIGHT: ::c_int = 2417;
pub const TMT_GLYPHINDEX: ::c_int = 2418;
pub const TMT_TRUESIZESTRETCHMARK: ::c_int = 2419;
pub const TMT_MINDPI1: ::c_int = 2420;
pub const TMT_MINDPI2: ::c_int = 2421;
pub const TMT_MINDPI3: ::c_int = 2422;
pub const TMT_MINDPI4: ::c_int = 2423;
pub const TMT_MINDPI5: ::c_int = 2424;
pub const TMT_TEXTGLOWSIZE: ::c_int = 2425;
pub const TMT_FRAMESPERSECOND: ::c_int = 2426;
pub const TMT_PIXELSPERFRAME: ::c_int = 2427;
pub const TMT_ANIMATIONDELAY: ::c_int = 2428;
pub const TMT_GLOWINTENSITY: ::c_int = 2429;
pub const TMT_OPACITY: ::c_int = 2430;
pub const TMT_COLORIZATIONCOLOR: ::c_int = 2431;
pub const TMT_COLORIZATIONOPACITY: ::c_int = 2432;
pub const TMT_MINDPI6: ::c_int = 2433;
pub const TMT_MINDPI7: ::c_int = 2434;
pub const TMT_GLYPHFONT: ::c_int = 2601;
pub const TMT_IMAGEFILE: ::c_int = 3001;
pub const TMT_IMAGEFILE1: ::c_int = 3002;
pub const TMT_IMAGEFILE2: ::c_int = 3003;
pub const TMT_IMAGEFILE3: ::c_int = 3004;
pub const TMT_IMAGEFILE4: ::c_int = 3005;
pub const TMT_IMAGEFILE5: ::c_int = 3006;
pub const TMT_GLYPHIMAGEFILE: ::c_int = 3008;
pub const TMT_IMAGEFILE6: ::c_int = 3009;
pub const TMT_IMAGEFILE7: ::c_int = 3010;
pub const TMT_TEXT: ::c_int = 3201;
pub const TMT_CLASSICVALUE: ::c_int = 3202;
pub const TMT_OFFSET: ::c_int = 3401;
pub const TMT_TEXTSHADOWOFFSET: ::c_int = 3402;
pub const TMT_MINSIZE: ::c_int = 3403;
pub const TMT_MINSIZE1: ::c_int = 3404;
pub const TMT_MINSIZE2: ::c_int = 3405;
pub const TMT_MINSIZE3: ::c_int = 3406;
pub const TMT_MINSIZE4: ::c_int = 3407;
pub const TMT_MINSIZE5: ::c_int = 3408;
pub const TMT_NORMALSIZE: ::c_int = 3409;
pub const TMT_MINSIZE6: ::c_int = 3410;
pub const TMT_MINSIZE7: ::c_int = 3411;
pub const TMT_SIZINGMARGINS: ::c_int = 3601;
pub const TMT_CONTENTMARGINS: ::c_int = 3602;
pub const TMT_CAPTIONMARGINS: ::c_int = 3603;
pub const TMT_BORDERCOLOR: ::c_int = 3801;
pub const TMT_FILLCOLOR: ::c_int = 3802;
pub const TMT_TEXTCOLOR: ::c_int = 3803;
pub const TMT_EDGELIGHTCOLOR: ::c_int = 3804;
pub const TMT_EDGEHIGHLIGHTCOLOR: ::c_int = 3805;
pub const TMT_EDGESHADOWCOLOR: ::c_int = 3806;
pub const TMT_EDGEDKSHADOWCOLOR: ::c_int = 3807;
pub const TMT_EDGEFILLCOLOR: ::c_int = 3808;
pub const TMT_TRANSPARENTCOLOR: ::c_int = 3809;
pub const TMT_GRADIENTCOLOR1: ::c_int = 3810;
pub const TMT_GRADIENTCOLOR2: ::c_int = 3811;
pub const TMT_GRADIENTCOLOR3: ::c_int = 3812;
pub const TMT_GRADIENTCOLOR4: ::c_int = 3813;
pub const TMT_GRADIENTCOLOR5: ::c_int = 3814;
pub const TMT_SHADOWCOLOR: ::c_int = 3815;
pub const TMT_GLOWCOLOR: ::c_int = 3816;
pub const TMT_TEXTBORDERCOLOR: ::c_int = 3817;
pub const TMT_TEXTSHADOWCOLOR: ::c_int = 3818;
pub const TMT_GLYPHTEXTCOLOR: ::c_int = 3819;
pub const TMT_GLYPHTRANSPARENTCOLOR: ::c_int = 3820;
pub const TMT_FILLCOLORHINT: ::c_int = 3821;
pub const TMT_BORDERCOLORHINT: ::c_int = 3822;
pub const TMT_ACCENTCOLORHINT: ::c_int = 3823;
pub const TMT_TEXTCOLORHINT: ::c_int = 3824;
pub const TMT_HEADING1TEXTCOLOR: ::c_int = 3825;
pub const TMT_HEADING2TEXTCOLOR: ::c_int = 3826;
pub const TMT_BODYTEXTCOLOR: ::c_int = 3827;
pub const TMT_BGTYPE: ::c_int = 4001;
pub const TMT_BORDERTYPE: ::c_int = 4002;
pub const TMT_FILLTYPE: ::c_int = 4003;
pub const TMT_SIZINGTYPE: ::c_int = 4004;
pub const TMT_HALIGN: ::c_int = 4005;
pub const TMT_CONTENTALIGNMENT: ::c_int = 4006;
pub const TMT_VALIGN: ::c_int = 4007;
pub const TMT_OFFSETTYPE: ::c_int = 4008;
pub const TMT_ICONEFFECT: ::c_int = 4009;
pub const TMT_TEXTSHADOWTYPE: ::c_int = 4010;
pub const TMT_IMAGELAYOUT: ::c_int = 4011;
pub const TMT_GLYPHTYPE: ::c_int = 4012;
pub const TMT_IMAGESELECTTYPE: ::c_int = 4013;
pub const TMT_GLYPHFONTSIZINGTYPE: ::c_int = 4014;
pub const TMT_TRUESIZESCALINGTYPE: ::c_int = 4015;
pub const TMT_USERPICTURE: ::c_int = 5001;
pub const TMT_DEFAULTPANESIZE: ::c_int = 5002;
pub const TMT_BLENDCOLOR: ::c_int = 5003;
pub const TMT_CUSTOMSPLITRECT: ::c_int = 5004;
pub const TMT_ANIMATIONBUTTONRECT: ::c_int = 5005;
pub const TMT_ANIMATIONDURATION: ::c_int = 5006;
pub const TMT_TRANSITIONDURATIONS: ::c_int = 6000;
pub const TMT_SCALEDBACKGROUND: ::c_int = 7001;
pub const TMT_ATLASIMAGE: ::c_int = 8000;
pub const TMT_ATLASINPUTIMAGE: ::c_int = 8001;
pub const TMT_ATLASRECT: ::c_int = 8002;
