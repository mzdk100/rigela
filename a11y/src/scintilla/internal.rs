/*
 * Copyright (c) 2024. The RigelA open source project team and
 * its contributors reserve all rights.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 * http://www.apache.org/licenses/LICENSE-2.0
 * Unless required by applicable law or agreed to in writing, software distributed under the
 * License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and limitations under the License.
 */

use std::{ffi::c_char, os::raw::c_void, ptr::slice_from_raw_parts};

use scintilla_sys::{
    Sci_CharacterRange, Sci_PositionCR, Sci_RangeToFormat, Sci_Rectangle, Sci_TextRange,
    Sci_TextToFind,
};
pub use scintilla_sys::{
    CARETSTYLE_BLOCK, CARETSTYLE_INVISIBLE, CARETSTYLE_LINE, CARET_EVEN, CARET_JUMPS, CARET_SLOP,
    CARET_STRICT, INDIC0_MASK, INDIC1_MASK, INDIC2_MASK, INDICS_MASK, KEYWORDSET_MAX,
    SCFIND_CXX11REGEX, SCFIND_MATCHCASE, SCFIND_POSIX, SCFIND_REGEXP, SCFIND_WHOLEWORD,
    SCFIND_WORDSTART, SCI_ADDREFDOCUMENT, SCI_ADDSELECTION, SCI_ADDSTYLEDTEXT, SCI_ADDTABSTOP,
    SCI_ADDTEXT, SCI_ADDUNDOACTION, SCI_ALLOCATE, SCI_ALLOCATEEXTENDEDSTYLES,
    SCI_ALLOCATESUBSTYLES, SCI_ANNOTATIONCLEARALL, SCI_ANNOTATIONGETLINES, SCI_ANNOTATIONGETSTYLE,
    SCI_ANNOTATIONGETSTYLEOFFSET, SCI_ANNOTATIONGETSTYLES, SCI_ANNOTATIONGETTEXT,
    SCI_ANNOTATIONGETVISIBLE, SCI_ANNOTATIONSETSTYLE, SCI_ANNOTATIONSETSTYLEOFFSET,
    SCI_ANNOTATIONSETSTYLES, SCI_ANNOTATIONSETTEXT, SCI_ANNOTATIONSETVISIBLE, SCI_APPENDTEXT,
    SCI_ASSIGNCMDKEY, SCI_AUTOCACTIVE, SCI_AUTOCCANCEL, SCI_AUTOCCOMPLETE, SCI_AUTOCGETAUTOHIDE,
    SCI_AUTOCGETCANCELATSTART, SCI_AUTOCGETCASEINSENSITIVEBEHAVIOUR, SCI_AUTOCGETCHOOSESINGLE,
    SCI_AUTOCGETCURRENT, SCI_AUTOCGETCURRENTTEXT, SCI_AUTOCGETDROPRESTOFWORD,
    SCI_AUTOCGETIGNORECASE, SCI_AUTOCGETMAXHEIGHT, SCI_AUTOCGETMAXWIDTH, SCI_AUTOCGETMULTI,
    SCI_AUTOCGETORDER, SCI_AUTOCGETSEPARATOR, SCI_AUTOCGETTYPESEPARATOR, SCI_AUTOCPOSSTART,
    SCI_AUTOCSELECT, SCI_AUTOCSETAUTOHIDE, SCI_AUTOCSETCANCELATSTART,
    SCI_AUTOCSETCASEINSENSITIVEBEHAVIOUR, SCI_AUTOCSETCHOOSESINGLE, SCI_AUTOCSETDROPRESTOFWORD,
    SCI_AUTOCSETFILLUPS, SCI_AUTOCSETIGNORECASE, SCI_AUTOCSETMAXHEIGHT, SCI_AUTOCSETMAXWIDTH,
    SCI_AUTOCSETMULTI, SCI_AUTOCSETORDER, SCI_AUTOCSETSEPARATOR, SCI_AUTOCSETTYPESEPARATOR,
    SCI_AUTOCSHOW, SCI_AUTOCSTOPS, SCI_BACKTAB, SCI_BEGINUNDOACTION, SCI_BRACEBADLIGHT,
    SCI_BRACEBADLIGHTINDICATOR, SCI_BRACEHIGHLIGHT, SCI_BRACEHIGHLIGHTINDICATOR, SCI_BRACEMATCH,
    SCI_CALLTIPACTIVE, SCI_CALLTIPCANCEL, SCI_CALLTIPPOSSTART, SCI_CALLTIPSETBACK,
    SCI_CALLTIPSETFORE, SCI_CALLTIPSETFOREHLT, SCI_CALLTIPSETHLT, SCI_CALLTIPSETPOSITION,
    SCI_CALLTIPSETPOSSTART, SCI_CALLTIPSHOW, SCI_CALLTIPUSESTYLE, SCI_CANCEL, SCI_CANPASTE,
    SCI_CANREDO, SCI_CANUNDO, SCI_CHANGEINSERTION, SCI_CHANGELEXERSTATE, SCI_CHARLEFT,
    SCI_CHARLEFTEXTEND, SCI_CHARLEFTRECTEXTEND, SCI_CHARRIGHT, SCI_CHARRIGHTEXTEND,
    SCI_CHARRIGHTRECTEXTEND, SCI_CHOOSECARETX, SCI_CLEAR, SCI_CLEARALL, SCI_CLEARALLCMDKEYS,
    SCI_CLEARCMDKEY, SCI_CLEARDOCUMENTSTYLE, SCI_CLEARREGISTEREDIMAGES, SCI_CLEARREPRESENTATION,
    SCI_CLEARSELECTIONS, SCI_CLEARTABSTOPS, SCI_COLOURISE, SCI_CONTRACTEDFOLDNEXT, SCI_CONVERTEOLS,
    SCI_COPY, SCI_COPYALLOWLINE, SCI_COPYRANGE, SCI_COPYTEXT, SCI_COUNTCHARACTERS,
    SCI_CREATEDOCUMENT, SCI_CREATELOADER, SCI_CUT, SCI_DELETEBACK, SCI_DELETEBACKNOTLINE,
    SCI_DELETERANGE, SCI_DELLINELEFT, SCI_DELLINERIGHT, SCI_DELWORDLEFT, SCI_DELWORDRIGHT,
    SCI_DELWORDRIGHTEND, SCI_DESCRIBEKEYWORDSETS, SCI_DESCRIBEPROPERTY, SCI_DESCRIPTIONOFSTYLE,
    SCI_DISTANCETOSECONDARYSTYLES, SCI_DOCUMENTEND, SCI_DOCUMENTENDEXTEND, SCI_DOCUMENTSTART,
    SCI_DOCUMENTSTARTEXTEND, SCI_DROPSELECTIONN, SCI_EDITTOGGLEOVERTYPE, SCI_EMPTYUNDOBUFFER,
    SCI_ENCODEDFROMUTF8, SCI_ENDUNDOACTION, SCI_ENSUREVISIBLE, SCI_ENSUREVISIBLEENFORCEPOLICY,
    SCI_EXPANDCHILDREN, SCI_FINDCOLUMN, SCI_FINDINDICATORFLASH, SCI_FINDINDICATORHIDE,
    SCI_FINDINDICATORSHOW, SCI_FINDTEXT, SCI_FOLDALL, SCI_FOLDCHILDREN,
    SCI_FOLDDISPLAYTEXTSETSTYLE, SCI_FOLDLINE, SCI_FORMATRANGE, SCI_FORMFEED, SCI_FREESUBSTYLES,
    SCI_GETACCESSIBILITY, SCI_GETADDITIONALCARETFORE, SCI_GETADDITIONALCARETSBLINK,
    SCI_GETADDITIONALCARETSVISIBLE, SCI_GETADDITIONALSELALPHA, SCI_GETADDITIONALSELECTIONTYPING,
    SCI_GETALLLINESVISIBLE, SCI_GETANCHOR, SCI_GETAUTOMATICFOLD, SCI_GETBACKSPACEUNINDENTS,
    SCI_GETBIDIRECTIONAL, SCI_GETBUFFEREDDRAW, SCI_GETCARETFORE, SCI_GETCARETLINEBACK,
    SCI_GETCARETLINEBACKALPHA, SCI_GETCARETLINEFRAME, SCI_GETCARETLINEVISIBLE,
    SCI_GETCARETLINEVISIBLEALWAYS, SCI_GETCARETPERIOD, SCI_GETCARETSTICKY, SCI_GETCARETSTYLE,
    SCI_GETCARETWIDTH, SCI_GETCHARACTERPOINTER, SCI_GETCHARAT, SCI_GETCODEPAGE, SCI_GETCOLUMN,
    SCI_GETCONTROLCHARSYMBOL, SCI_GETCURLINE, SCI_GETCURRENTPOS, SCI_GETCURSOR,
    SCI_GETDIRECTFUNCTION, SCI_GETDIRECTPOINTER, SCI_GETDOCPOINTER, SCI_GETEDGECOLOUR,
    SCI_GETEDGECOLUMN, SCI_GETEDGEMODE, SCI_GETENDATLASTLINE, SCI_GETENDSTYLED, SCI_GETEOLMODE,
    SCI_GETEXTRAASCENT, SCI_GETEXTRADESCENT, SCI_GETFIRSTVISIBLELINE, SCI_GETFOCUS,
    SCI_GETFOLDEXPANDED, SCI_GETFOLDLEVEL, SCI_GETFOLDPARENT, SCI_GETFONTQUALITY,
    SCI_GETGAPPOSITION, SCI_GETHIGHLIGHTGUIDE, SCI_GETHOTSPOTACTIVEBACK, SCI_GETHOTSPOTACTIVEFORE,
    SCI_GETHOTSPOTACTIVEUNDERLINE, SCI_GETHOTSPOTSINGLELINE, SCI_GETHSCROLLBAR, SCI_GETIDLESTYLING,
    SCI_GETIMEINTERACTION, SCI_GETINDENT, SCI_GETINDENTATIONGUIDES, SCI_GETINDICATORCURRENT,
    SCI_GETINDICATORVALUE, SCI_GETLASTCHILD, SCI_GETLAYOUTCACHE, SCI_GETLENGTH, SCI_GETLEXER,
    SCI_GETLEXERLANGUAGE, SCI_GETLINE, SCI_GETLINECOUNT, SCI_GETLINEENDPOSITION,
    SCI_GETLINEENDTYPESACTIVE, SCI_GETLINEENDTYPESALLOWED, SCI_GETLINEENDTYPESSUPPORTED,
    SCI_GETLINEINDENTATION, SCI_GETLINEINDENTPOSITION, SCI_GETLINESELENDPOSITION,
    SCI_GETLINESELSTARTPOSITION, SCI_GETLINESTATE, SCI_GETLINEVISIBLE, SCI_GETMAINSELECTION,
    SCI_GETMARGINBACKN, SCI_GETMARGINCURSORN, SCI_GETMARGINLEFT, SCI_GETMARGINMASKN,
    SCI_GETMARGINOPTIONS, SCI_GETMARGINRIGHT, SCI_GETMARGINS, SCI_GETMARGINSENSITIVEN,
    SCI_GETMARGINTYPEN, SCI_GETMARGINWIDTHN, SCI_GETMAXLINESTATE, SCI_GETMODIFY,
    SCI_GETMOUSEDOWNCAPTURES, SCI_GETMOUSESELECTIONRECTANGULARSWITCH, SCI_GETMOUSEWHEELCAPTURES,
    SCI_GETMOVEEXTENDSSELECTION, SCI_GETMULTIPASTE, SCI_GETMULTIPLESELECTION, SCI_GETNAMEDSTYLES,
    SCI_GETNEXTTABSTOP, SCI_GETOVERTYPE, SCI_GETPASTECONVERTENDINGS, SCI_GETPHASESDRAW,
    SCI_GETPOSITIONCACHE, SCI_GETPRIMARYSTYLEFROMSTYLE, SCI_GETPRINTCOLOURMODE,
    SCI_GETPRINTMAGNIFICATION, SCI_GETPRINTWRAPMODE, SCI_GETPROPERTY, SCI_GETPROPERTYEXPANDED,
    SCI_GETPROPERTYINT, SCI_GETPUNCTUATIONCHARS, SCI_GETRANGEPOINTER, SCI_GETREADONLY,
    SCI_GETRECTANGULARSELECTIONANCHOR, SCI_GETRECTANGULARSELECTIONANCHORVIRTUALSPACE,
    SCI_GETRECTANGULARSELECTIONCARET, SCI_GETRECTANGULARSELECTIONCARETVIRTUALSPACE,
    SCI_GETRECTANGULARSELECTIONMODIFIER, SCI_GETREPRESENTATION, SCI_GETSCROLLWIDTH,
    SCI_GETSCROLLWIDTHTRACKING, SCI_GETSEARCHFLAGS, SCI_GETSELALPHA, SCI_GETSELECTIONEMPTY,
    SCI_GETSELECTIONEND, SCI_GETSELECTIONMODE, SCI_GETSELECTIONNANCHOR,
    SCI_GETSELECTIONNANCHORVIRTUALSPACE, SCI_GETSELECTIONNCARET,
    SCI_GETSELECTIONNCARETVIRTUALSPACE, SCI_GETSELECTIONNEND, SCI_GETSELECTIONNSTART,
    SCI_GETSELECTIONS, SCI_GETSELECTIONSTART, SCI_GETSELEOLFILLED, SCI_GETSELTEXT, SCI_GETSTATUS,
    SCI_GETSTYLEAT, SCI_GETSTYLEDTEXT, SCI_GETSTYLEFROMSUBSTYLE, SCI_GETSUBSTYLEBASES,
    SCI_GETSUBSTYLESLENGTH, SCI_GETSUBSTYLESSTART, SCI_GETTABDRAWMODE, SCI_GETTABINDENTS,
    SCI_GETTABWIDTH, SCI_GETTAG, SCI_GETTARGETEND, SCI_GETTARGETSTART, SCI_GETTARGETTEXT,
    SCI_GETTECHNOLOGY, SCI_GETTEXT, SCI_GETTEXTLENGTH, SCI_GETTEXTRANGE, SCI_GETUNDOCOLLECTION,
    SCI_GETUSETABS, SCI_GETVIEWEOL, SCI_GETVIEWWS, SCI_GETVIRTUALSPACEOPTIONS, SCI_GETVSCROLLBAR,
    SCI_GETWHITESPACECHARS, SCI_GETWHITESPACESIZE, SCI_GETWORDCHARS, SCI_GETWRAPINDENTMODE,
    SCI_GETWRAPMODE, SCI_GETWRAPSTARTINDENT, SCI_GETWRAPVISUALFLAGS,
    SCI_GETWRAPVISUALFLAGSLOCATION, SCI_GETXOFFSET, SCI_GETZOOM, SCI_GOTOLINE, SCI_GOTOPOS,
    SCI_GRABFOCUS, SCI_HIDELINES, SCI_HIDESELECTION, SCI_HOME, SCI_HOMEDISPLAY,
    SCI_HOMEDISPLAYEXTEND, SCI_HOMEEXTEND, SCI_HOMERECTEXTEND, SCI_HOMEWRAP, SCI_HOMEWRAPEXTEND,
    SCI_INDICATORALLONFOR, SCI_INDICATORCLEARRANGE, SCI_INDICATOREND, SCI_INDICATORFILLRANGE,
    SCI_INDICATORSTART, SCI_INDICATORVALUEAT, SCI_INDICGETALPHA, SCI_INDICGETFLAGS,
    SCI_INDICGETFORE, SCI_INDICGETHOVERFORE, SCI_INDICGETHOVERSTYLE, SCI_INDICGETOUTLINEALPHA,
    SCI_INDICGETSTYLE, SCI_INDICGETUNDER, SCI_INDICSETALPHA, SCI_INDICSETFLAGS, SCI_INDICSETFORE,
    SCI_INDICSETHOVERFORE, SCI_INDICSETHOVERSTYLE, SCI_INDICSETOUTLINEALPHA, SCI_INDICSETSTYLE,
    SCI_INDICSETUNDER, SCI_INSERTTEXT, SCI_ISRANGEWORD, SCI_LINECOPY, SCI_LINECUT, SCI_LINEDELETE,
    SCI_LINEDOWN, SCI_LINEDOWNEXTEND, SCI_LINEDOWNRECTEXTEND, SCI_LINEDUPLICATE, SCI_LINEEND,
    SCI_LINEENDDISPLAY, SCI_LINEENDDISPLAYEXTEND, SCI_LINEENDEXTEND, SCI_LINEENDRECTEXTEND,
    SCI_LINEENDWRAP, SCI_LINEENDWRAPEXTEND, SCI_LINEFROMPOSITION, SCI_LINELENGTH, SCI_LINEREVERSE,
    SCI_LINESCROLL, SCI_LINESCROLLDOWN, SCI_LINESCROLLUP, SCI_LINESJOIN, SCI_LINESONSCREEN,
    SCI_LINESSPLIT, SCI_LINETRANSPOSE, SCI_LINEUP, SCI_LINEUPEXTEND, SCI_LINEUPRECTEXTEND,
    SCI_LOWERCASE, SCI_MARGINGETSTYLE, SCI_MARGINGETSTYLEOFFSET, SCI_MARGINGETSTYLES,
    SCI_MARGINGETTEXT, SCI_MARGINSETSTYLE, SCI_MARGINSETSTYLEOFFSET, SCI_MARGINSETSTYLES,
    SCI_MARGINSETTEXT, SCI_MARGINTEXTCLEARALL, SCI_MARKERADD, SCI_MARKERADDSET, SCI_MARKERDEFINE,
    SCI_MARKERDEFINEPIXMAP, SCI_MARKERDEFINERGBAIMAGE, SCI_MARKERDELETE, SCI_MARKERDELETEALL,
    SCI_MARKERDELETEHANDLE, SCI_MARKERENABLEHIGHLIGHT, SCI_MARKERGET, SCI_MARKERLINEFROMHANDLE,
    SCI_MARKERNEXT, SCI_MARKERPREVIOUS, SCI_MARKERSETALPHA, SCI_MARKERSETBACK,
    SCI_MARKERSETBACKSELECTED, SCI_MARKERSETFORE, SCI_MARKERSYMBOLDEFINED, SCI_MOVECARETINSIDEVIEW,
    SCI_MOVESELECTEDLINESDOWN, SCI_MOVESELECTEDLINESUP, SCI_MULTIEDGEADDLINE,
    SCI_MULTIEDGECLEARALL, SCI_MULTIPLESELECTADDEACH, SCI_MULTIPLESELECTADDNEXT, SCI_NAMEOFSTYLE,
    SCI_NEWLINE, SCI_NULL, SCI_PAGEDOWN, SCI_PAGEDOWNEXTEND, SCI_PAGEDOWNRECTEXTEND, SCI_PAGEUP,
    SCI_PAGEUPEXTEND, SCI_PAGEUPRECTEXTEND, SCI_PARADOWN, SCI_PARADOWNEXTEND, SCI_PARAUP,
    SCI_PARAUPEXTEND, SCI_PASTE, SCI_POINTXFROMPOSITION, SCI_POINTYFROMPOSITION, SCI_POSITIONAFTER,
    SCI_POSITIONBEFORE, SCI_POSITIONFROMPOINT, SCI_POSITIONFROMPOINTCLOSE, SCI_POSITIONRELATIVE,
    SCI_PRIVATELEXERCALL, SCI_PROPERTYNAMES, SCI_PROPERTYTYPE, SCI_REDO, SCI_REGISTERIMAGE,
    SCI_REGISTERRGBAIMAGE, SCI_RELEASEALLEXTENDEDSTYLES, SCI_RELEASEDOCUMENT, SCI_REPLACESEL,
    SCI_REPLACETARGET, SCI_REPLACETARGETRE, SCI_RGBAIMAGESETHEIGHT, SCI_RGBAIMAGESETSCALE,
    SCI_RGBAIMAGESETWIDTH, SCI_ROTATESELECTION, SCI_SCROLLCARET, SCI_SCROLLRANGE, SCI_SCROLLTOEND,
    SCI_SCROLLTOSTART, SCI_SEARCHANCHOR, SCI_SEARCHINTARGET, SCI_SEARCHNEXT, SCI_SEARCHPREV,
    SCI_SELECTALL, SCI_SELECTIONDUPLICATE, SCI_SELECTIONISRECTANGLE, SCI_SETACCESSIBILITY,
    SCI_SETADDITIONALCARETFORE, SCI_SETADDITIONALCARETSBLINK, SCI_SETADDITIONALCARETSVISIBLE,
    SCI_SETADDITIONALSELALPHA, SCI_SETADDITIONALSELBACK, SCI_SETADDITIONALSELECTIONTYPING,
    SCI_SETADDITIONALSELFORE, SCI_SETANCHOR, SCI_SETAUTOMATICFOLD, SCI_SETBACKSPACEUNINDENTS,
    SCI_SETBIDIRECTIONAL, SCI_SETBUFFEREDDRAW, SCI_SETCARETFORE, SCI_SETCARETLINEBACK,
    SCI_SETCARETLINEBACKALPHA, SCI_SETCARETLINEFRAME, SCI_SETCARETLINEVISIBLE,
    SCI_SETCARETLINEVISIBLEALWAYS, SCI_SETCARETPERIOD, SCI_SETCARETSTICKY, SCI_SETCARETSTYLE,
    SCI_SETCARETWIDTH, SCI_SETCHARSDEFAULT, SCI_SETCODEPAGE, SCI_SETCONTROLCHARSYMBOL,
    SCI_SETCURRENTPOS, SCI_SETCURSOR, SCI_SETDOCPOINTER, SCI_SETEDGECOLOUR, SCI_SETEDGECOLUMN,
    SCI_SETEDGEMODE, SCI_SETEMPTYSELECTION, SCI_SETENDATLASTLINE, SCI_SETEOLMODE,
    SCI_SETEXTRAASCENT, SCI_SETEXTRADESCENT, SCI_SETFIRSTVISIBLELINE, SCI_SETFOCUS,
    SCI_SETFOLDEXPANDED, SCI_SETFOLDFLAGS, SCI_SETFOLDLEVEL, SCI_SETFOLDMARGINCOLOUR,
    SCI_SETFOLDMARGINHICOLOUR, SCI_SETFONTQUALITY, SCI_SETHIGHLIGHTGUIDE, SCI_SETHOTSPOTACTIVEBACK,
    SCI_SETHOTSPOTACTIVEFORE, SCI_SETHOTSPOTACTIVEUNDERLINE, SCI_SETHOTSPOTSINGLELINE,
    SCI_SETHSCROLLBAR, SCI_SETIDENTIFIERS, SCI_SETIDLESTYLING, SCI_SETIMEINTERACTION,
    SCI_SETINDENT, SCI_SETINDENTATIONGUIDES, SCI_SETINDICATORCURRENT, SCI_SETINDICATORVALUE,
    SCI_SETKEYWORDS, SCI_SETLAYOUTCACHE, SCI_SETLENGTHFORENCODE, SCI_SETLINEENDTYPESALLOWED,
    SCI_SETLINEINDENTATION, SCI_SETLINESTATE, SCI_SETMAINSELECTION, SCI_SETMARGINBACKN,
    SCI_SETMARGINCURSORN, SCI_SETMARGINLEFT, SCI_SETMARGINMASKN, SCI_SETMARGINOPTIONS,
    SCI_SETMARGINRIGHT, SCI_SETMARGINS, SCI_SETMARGINSENSITIVEN, SCI_SETMARGINTYPEN,
    SCI_SETMARGINWIDTHN, SCI_SETMOUSEDOWNCAPTURES, SCI_SETMOUSESELECTIONRECTANGULARSWITCH,
    SCI_SETMOUSEWHEELCAPTURES, SCI_SETMULTIPASTE, SCI_SETMULTIPLESELECTION, SCI_SETOVERTYPE,
    SCI_SETPASTECONVERTENDINGS, SCI_SETPHASESDRAW, SCI_SETPOSITIONCACHE, SCI_SETPRINTCOLOURMODE,
    SCI_SETPRINTMAGNIFICATION, SCI_SETPRINTWRAPMODE, SCI_SETPROPERTY, SCI_SETPUNCTUATIONCHARS,
    SCI_SETREADONLY, SCI_SETRECTANGULARSELECTIONANCHOR,
    SCI_SETRECTANGULARSELECTIONANCHORVIRTUALSPACE, SCI_SETRECTANGULARSELECTIONCARET,
    SCI_SETRECTANGULARSELECTIONCARETVIRTUALSPACE, SCI_SETRECTANGULARSELECTIONMODIFIER,
    SCI_SETREPRESENTATION, SCI_SETSAVEPOINT, SCI_SETSCROLLWIDTH, SCI_SETSCROLLWIDTHTRACKING,
    SCI_SETSEARCHFLAGS, SCI_SETSEL, SCI_SETSELALPHA, SCI_SETSELBACK, SCI_SETSELECTION,
    SCI_SETSELECTIONEND, SCI_SETSELECTIONMODE, SCI_SETSELECTIONNANCHOR,
    SCI_SETSELECTIONNANCHORVIRTUALSPACE, SCI_SETSELECTIONNCARET,
    SCI_SETSELECTIONNCARETVIRTUALSPACE, SCI_SETSELECTIONNEND, SCI_SETSELECTIONNSTART,
    SCI_SETSELECTIONSTART, SCI_SETSELEOLFILLED, SCI_SETSELFORE, SCI_SETSTATUS, SCI_SETSTYLING,
    SCI_SETSTYLINGEX, SCI_SETTABDRAWMODE, SCI_SETTABINDENTS, SCI_SETTABWIDTH, SCI_SETTARGETEND,
    SCI_SETTARGETRANGE, SCI_SETTARGETSTART, SCI_SETTECHNOLOGY, SCI_SETTEXT, SCI_SETUNDOCOLLECTION,
    SCI_SETUSETABS, SCI_SETVIEWEOL, SCI_SETVIEWWS, SCI_SETVIRTUALSPACEOPTIONS,
    SCI_SETVISIBLEPOLICY, SCI_SETVSCROLLBAR, SCI_SETWHITESPACEBACK, SCI_SETWHITESPACECHARS,
    SCI_SETWHITESPACEFORE, SCI_SETWHITESPACESIZE, SCI_SETWORDCHARS, SCI_SETWRAPINDENTMODE,
    SCI_SETWRAPMODE, SCI_SETWRAPSTARTINDENT, SCI_SETWRAPVISUALFLAGS,
    SCI_SETWRAPVISUALFLAGSLOCATION, SCI_SETXCARETPOLICY, SCI_SETXOFFSET, SCI_SETYCARETPOLICY,
    SCI_SETZOOM, SCI_SHOWLINES, SCI_STARTRECORD, SCI_STARTSTYLING, SCI_STOPRECORD,
    SCI_STUTTEREDPAGEDOWN, SCI_STUTTEREDPAGEDOWNEXTEND, SCI_STUTTEREDPAGEUP,
    SCI_STUTTEREDPAGEUPEXTEND, SCI_STYLECLEARALL, SCI_STYLEGETBACK, SCI_STYLEGETBOLD,
    SCI_STYLEGETCASE, SCI_STYLEGETCHANGEABLE, SCI_STYLEGETCHARACTERSET, SCI_STYLEGETEOLFILLED,
    SCI_STYLEGETFONT, SCI_STYLEGETFORE, SCI_STYLEGETHOTSPOT, SCI_STYLEGETITALIC, SCI_STYLEGETSIZE,
    SCI_STYLEGETSIZEFRACTIONAL, SCI_STYLEGETUNDERLINE, SCI_STYLEGETVISIBLE, SCI_STYLEGETWEIGHT,
    SCI_STYLERESETDEFAULT, SCI_STYLESETBACK, SCI_STYLESETBOLD, SCI_STYLESETCASE,
    SCI_STYLESETCHANGEABLE, SCI_STYLESETCHARACTERSET, SCI_STYLESETEOLFILLED, SCI_STYLESETFONT,
    SCI_STYLESETFORE, SCI_STYLESETHOTSPOT, SCI_STYLESETITALIC, SCI_STYLESETSIZE,
    SCI_STYLESETSIZEFRACTIONAL, SCI_STYLESETUNDERLINE, SCI_STYLESETVISIBLE, SCI_STYLESETWEIGHT,
    SCI_SWAPMAINANCHORCARET, SCI_TAB, SCI_TAGSOFSTYLE, SCI_TARGETASUTF8, SCI_TARGETFROMSELECTION,
    SCI_TARGETWHOLEDOCUMENT, SCI_TEXTHEIGHT, SCI_TEXTWIDTH, SCI_TOGGLECARETSTICKY, SCI_TOGGLEFOLD,
    SCI_TOGGLEFOLDSHOWTEXT, SCI_UNDO, SCI_UPPERCASE, SCI_USEPOPUP, SCI_USERLISTSHOW, SCI_VCHOME,
    SCI_VCHOMEDISPLAY, SCI_VCHOMEDISPLAYEXTEND, SCI_VCHOMEEXTEND, SCI_VCHOMERECTEXTEND,
    SCI_VCHOMEWRAP, SCI_VCHOMEWRAPEXTEND, SCI_VERTICALCENTRECARET, SCI_VISIBLEFROMDOCLINE,
    SCI_WORDENDPOSITION, SCI_WORDLEFT, SCI_WORDLEFTEND, SCI_WORDLEFTENDEXTEND, SCI_WORDLEFTEXTEND,
    SCI_WORDPARTLEFT, SCI_WORDPARTLEFTEXTEND, SCI_WORDPARTRIGHT, SCI_WORDPARTRIGHTEXTEND,
    SCI_WORDRIGHT, SCI_WORDRIGHTEND, SCI_WORDRIGHTENDEXTEND, SCI_WORDRIGHTEXTEND,
    SCI_WORDSTARTPOSITION, SCI_WRAPCOUNT, SCI_ZOOMIN, SCI_ZOOMOUT, SCMOD_ALT, SCMOD_CTRL,
    SCMOD_META, SCMOD_NORM, SCMOD_SHIFT, SCMOD_SUPER, SCVS_NONE, SCVS_NOWRAPLINESTART,
    SCVS_RECTANGULARSELECTION, SCVS_USERACCESSIBLE, SC_AUTOMATICFOLD_CHANGE,
    SC_AUTOMATICFOLD_CLICK, SC_AUTOMATICFOLD_SHOW, SC_CASEINSENSITIVEBEHAVIOUR_IGNORECASE,
    SC_CASEINSENSITIVEBEHAVIOUR_RESPECTCASE, SC_CP_UTF8, SC_CURSORARROW, SC_CURSORNORMAL,
    SC_CURSORREVERSEARROW, SC_CURSORWAIT, SC_EFF_QUALITY_ANTIALIASED, SC_EFF_QUALITY_DEFAULT,
    SC_EFF_QUALITY_LCD_OPTIMIZED, SC_EFF_QUALITY_MASK, SC_EFF_QUALITY_NON_ANTIALIASED,
    SC_FOLDFLAG_LEVELNUMBERS, SC_FOLDFLAG_LINEAFTER_CONTRACTED, SC_FOLDFLAG_LINEAFTER_EXPANDED,
    SC_FOLDFLAG_LINEBEFORE_CONTRACTED, SC_FOLDFLAG_LINEBEFORE_EXPANDED, SC_FOLDFLAG_LINESTATE,
    SC_FOLDLEVELBASE, SC_FOLDLEVELHEADERFLAG, SC_FOLDLEVELNUMBERMASK, SC_FOLDLEVELWHITEFLAG,
    SC_INDICFLAG_VALUEFORE, SC_INDICVALUEMASK, SC_LINE_END_TYPE_DEFAULT, SC_LINE_END_TYPE_UNICODE,
    SC_MARGIN_BACK, SC_MARGIN_COLOUR, SC_MARGIN_FORE, SC_MARGIN_NUMBER, SC_MARGIN_RTEXT,
    SC_MARGIN_SYMBOL, SC_MARGIN_TEXT, SC_MASK_FOLDERS, SC_WEIGHT_BOLD, SC_WEIGHT_NORMAL,
    SC_WEIGHT_SEMIBOLD, UNDO_MAY_COALESCE, VISIBLE_SLOP, VISIBLE_STRICT,
};

use win_wrap::{
    common::{LPARAM, WPARAM},
    control::WindowControl,
    ext::StringExt,
    memory::InProcessMemory,
};

use crate::scintilla::{
    accessibility::Accessibility,
    annotation::Annotation,
    autoc::MultiAutoc,
    bidirectional::Bidirectional,
    cache::CacheMode,
    caret::CaretSticky,
    character::CharacterSet,
    edge::EdgeMode,
    eol::EolMode,
    ext::VecExt,
    folding::{FoldAction, FoldDisplayText},
    ime::Ime,
    indentation::IndentView,
    indicator::Indicator,
    key::KeyCode,
    margin::MarginOptions,
    marker::Mark,
    order::Order,
    phases::Phases,
    popup::PopUpMode,
    print::PrintMode,
    selection::SelectionMode,
    space::{TabDrawMode, WhiteSpace},
    status::Status,
    style::{Case, IdleStyling},
    technology::Technology,
    typing::PropertyType,
    wrap::{WrapIndent, WrapMode},
    Scintilla,
};

/// 标准行为
pub const SC_DOCUMENTOPTION_DEFAULT: u32 = 0;
/// 停止为样式分配内存并将所有文本视为样式 0。
pub const SC_DOCUMENTOPTION_STYLES_NONE: u32 = 0x1;
/// 允许文档大于 2 GB。
pub const SC_DOCUMENTOPTION_TEXT_LARGE: u32 = 0x100;

pub type SurfaceId = *mut c_void;
pub type Cell = u16;
pub type Rectangle = Sci_Rectangle;

impl Scintilla for WindowControl {
    fn get_text(&self, length: usize) -> Option<String> {
        let mem = InProcessMemory::new(self.get_process_handle(), length).unwrap();
        self.send_message(SCI_GETTEXT, WPARAM(length), LPARAM(mem.as_ptr() as isize));
        mem.read(|buf| (buf as *const u8).to_string())
    }

    fn set_text(&self, text: String) {
        let size = text.as_bytes().len();
        let mem = InProcessMemory::new(self.get_process_handle(), size).unwrap();
        mem.write(text.as_ptr() as *const c_void, size);
        self.send_message(SCI_SETTEXT, WPARAM(size), LPARAM(mem.as_ptr() as isize));
    }

    fn set_save_point(&self) {
        self.send_message(SCI_SETSAVEPOINT, WPARAM::default(), LPARAM::default());
    }

    fn line_length(&self, line: usize) -> usize {
        let (_, res) = self.send_message(SCI_LINELENGTH, WPARAM(line), LPARAM::default());
        res
    }

    fn get_line(&self, line: usize, length: usize) -> Option<String> {
        let mem = InProcessMemory::new(self.get_process_handle(), length).unwrap();
        self.send_message(SCI_GETLINE, WPARAM(line), LPARAM(mem.as_ptr() as isize));
        mem.read(|buf| (buf as *const u8).to_string())
    }

    fn replace_sel(&self, text: String) {
        let size = text.as_bytes().len();
        let mem = InProcessMemory::new(self.get_process_handle(), size).unwrap();
        mem.write(text.as_ptr() as *const c_void, size);
        self.send_message(SCI_REPLACESEL, WPARAM(size), LPARAM(mem.as_ptr() as isize));
    }

    fn get_readonly(&self) -> bool {
        let (_, res) = self.send_message(SCI_GETREADONLY, WPARAM::default(), LPARAM::default());
        res != 0
    }

    fn set_readonly(&self, val: bool) {
        let val = if val { 1 } else { 0 };
        self.send_message(SCI_SETREADONLY, WPARAM(val), LPARAM::default());
    }

    fn get_text_range(&self, min: isize, max: isize) -> Option<String> {
        let mem = InProcessMemory::new(self.get_process_handle(), ((max - min).abs() + 1) as usize)
            .unwrap();
        let param = Sci_TextRange {
            chrg: Sci_CharacterRange {
                cpMax: max as Sci_PositionCR,
                cpMin: min as Sci_PositionCR,
            },
            lpstrText: mem.as_ptr_mut() as *mut c_char,
        };
        let size = std::mem::size_of::<Sci_TextRange>();
        let mem2 = InProcessMemory::new(self.get_process_handle(), size).unwrap();
        mem2.write(&param as *const Sci_TextRange as *const c_void, size);
        self.send_message(
            SCI_GETTEXTRANGE,
            WPARAM::default(),
            LPARAM(mem2.as_ptr() as isize),
        );
        mem.read(|buf| (buf as *const u8).to_string())
    }

    fn get_styled_text(&self, min: isize, max: isize) -> Option<Vec<Cell>> {
        let mem = InProcessMemory::new(
            self.get_process_handle(),
            (2 * (max - min).abs() + 2) as usize,
        )
        .unwrap();
        let param = Sci_TextRange {
            chrg: Sci_CharacterRange {
                cpMax: max as Sci_PositionCR,
                cpMin: min as Sci_PositionCR,
            },
            lpstrText: mem.as_ptr_mut() as *mut c_char,
        };
        let size = std::mem::size_of::<Sci_TextRange>();
        let mem2 = InProcessMemory::new(self.get_process_handle(), size).unwrap();
        mem2.write(&param as *const Sci_TextRange as *const c_void, size);
        let (_, len) = self.send_message(
            SCI_GETSTYLEDTEXT,
            WPARAM::default(),
            LPARAM(mem2.as_ptr() as isize),
        );
        mem.read(|buf| (buf as *const Cell).to_vec(len))
    }

    fn allocate(&self, bytes: usize) {
        self.send_message(SCI_ALLOCATE, WPARAM(bytes), LPARAM::default());
    }

    fn add_text(&self, text: String) {
        let size = text.as_bytes().len();
        let mem = InProcessMemory::new(self.get_process_handle(), size).unwrap();
        mem.write(text.as_ptr() as *const c_void, size);
        self.send_message(SCI_ADDTEXT, WPARAM(size), LPARAM(mem.as_ptr() as isize));
    }

    fn add_styled_text(&self, text: &[Cell]) {
        let size = text.len() * std::mem::size_of::<Cell>();
        let mem = InProcessMemory::new(self.get_process_handle(), size).unwrap();
        mem.write(text.as_ptr() as *const c_void, size);
        self.send_message(
            SCI_ADDSTYLEDTEXT,
            WPARAM(size),
            LPARAM(mem.as_ptr() as isize),
        );
    }

    fn append_text(&self, text: String) {
        let size = text.as_bytes().len();
        let mem = InProcessMemory::new(self.get_process_handle(), size).unwrap();
        mem.write(text.as_ptr() as *const c_void, size);
        self.send_message(SCI_APPENDTEXT, WPARAM(size), LPARAM(mem.as_ptr() as isize));
    }

    fn insert_text(&self, pos: usize, text: String) {
        let size = text.as_bytes().len();
        let mem = InProcessMemory::new(self.get_process_handle(), size).unwrap();
        mem.write(text.as_ptr() as *const c_void, size);
        self.send_message(SCI_INSERTTEXT, WPARAM(pos), LPARAM(mem.as_ptr() as isize));
    }

    fn change_insertion(&self, text: String) {
        let size = text.as_bytes().len();
        let mem = InProcessMemory::new(self.get_process_handle(), size).unwrap();
        mem.write(text.as_ptr() as *const c_void, size);
        self.send_message(
            SCI_CHANGEINSERTION,
            WPARAM(size),
            LPARAM(mem.as_ptr() as isize),
        );
    }

    fn clear_all(&self) {
        self.send_message(SCI_CLEARALL, WPARAM::default(), LPARAM::default());
    }

    fn delete_range(&self, start: usize, length: usize) {
        self.send_message(SCI_DELETERANGE, WPARAM(start), LPARAM(length as isize));
    }

    fn clear_document_style(&self) {
        self.send_message(SCI_CLEARDOCUMENTSTYLE, WPARAM::default(), LPARAM::default());
    }

    fn get_char_at(&self, pos: usize) -> isize {
        let (_, res) = self.send_message(SCI_GETCHARAT, WPARAM(pos), LPARAM::default());
        res as isize
    }

    fn get_style_at(&self, pos: usize) -> isize {
        let (_, res) = self.send_message(SCI_GETSTYLEAT, WPARAM(pos), LPARAM::default());
        res as isize
    }

    fn release_all_extended_styles(&self) {
        self.send_message(
            SCI_RELEASEALLEXTENDEDSTYLES,
            WPARAM::default(),
            LPARAM::default(),
        );
    }

    fn allocate_extended_styles(&self, number: isize) -> isize {
        let (_, res) = self.send_message(
            SCI_ALLOCATEEXTENDEDSTYLES,
            WPARAM(number as usize),
            LPARAM::default(),
        );
        res as isize
    }

    fn target_as_utf8(&self) -> usize {
        let (_, res) = self.send_message(SCI_TARGETASUTF8, WPARAM::default(), LPARAM::default());
        res
    }

    fn encoded_from_utf8(&self, text: String) -> Vec<u8> {
        let size = text.as_bytes().len();
        self.send_message(SCI_SETLENGTHFORENCODE, WPARAM(size), LPARAM::default());
        let mem = InProcessMemory::new(self.get_process_handle(), size + 1).unwrap();
        mem.write(text.as_ptr() as *const c_void, size);
        let (_, len) = self.send_message(
            SCI_ENCODEDFROMUTF8,
            WPARAM(mem.as_ptr() as usize),
            LPARAM::default(),
        );
        let mem2 = InProcessMemory::new(self.get_process_handle(), len + 1).unwrap();
        self.send_message(
            SCI_ENCODEDFROMUTF8,
            WPARAM(mem.as_ptr() as usize),
            LPARAM(mem2.as_ptr() as isize),
        );
        mem2.read(|buf| (buf as *const u8).to_vec(len))
            .unwrap_or(Vec::new())
    }

    fn get_text_length(&self) -> usize {
        let (_, res) = self.send_message(SCI_GETTEXTLENGTH, WPARAM::default(), LPARAM::default());
        res
    }

    fn get_length(&self) -> usize {
        let (_, res) = self.send_message(SCI_GETLENGTH, WPARAM::default(), LPARAM::default());
        res
    }

    fn get_line_count(&self) -> usize {
        let (_, res) = self.send_message(SCI_GETLINECOUNT, WPARAM::default(), LPARAM::default());
        res
    }

    fn lines_on_screen(&self) -> usize {
        let (_, res) = self.send_message(SCI_LINESONSCREEN, WPARAM::default(), LPARAM::default());
        res
    }

    fn get_modify(&self) -> bool {
        let (_, res) = self.send_message(SCI_GETMODIFY, WPARAM::default(), LPARAM::default());
        res != 0
    }

    fn line_from_position(&self, pos: usize) -> usize {
        let (_, res) = self.send_message(SCI_LINEFROMPOSITION, WPARAM(pos), LPARAM::default());
        res
    }

    fn get_line_end_position(&self, line: usize) -> usize {
        let (_, res) = self.send_message(SCI_GETLINEENDPOSITION, WPARAM(line), LPARAM::default());
        res
    }

    fn get_column(&self, pos: usize) -> usize {
        let (_, res) = self.send_message(SCI_GETCOLUMN, WPARAM(pos), LPARAM::default());
        res
    }

    fn find_column(&self, line: usize, column: usize) -> usize {
        let (_, res) = self.send_message(SCI_FINDCOLUMN, WPARAM(line), LPARAM(column as isize));
        res
    }

    fn position_before(&self, pos: usize) -> usize {
        let (_, res) = self.send_message(SCI_POSITIONBEFORE, WPARAM(pos), LPARAM::default());
        res
    }

    fn position_after(&self, pos: usize) -> usize {
        let (_, res) = self.send_message(SCI_POSITIONAFTER, WPARAM(pos), LPARAM::default());
        res
    }

    fn text_width(&self, style: i32, text: String) -> i32 {
        let size = text.as_bytes().len();
        let mem = InProcessMemory::new(self.get_process_handle(), size).unwrap();
        mem.write(text.as_ptr() as *const c_void, size);
        let (_, res) = self.send_message(
            SCI_TEXTWIDTH,
            WPARAM(style as usize),
            LPARAM(mem.as_ptr() as isize),
        );
        res as i32
    }

    fn text_height(&self, line: usize) -> i32 {
        let (_, res) = self.send_message(SCI_TEXTHEIGHT, WPARAM(line), LPARAM::default());
        res as i32
    }

    fn position_from_point(&self, x: i32, y: i32) -> usize {
        let (_, res) = self.send_message(
            SCI_POSITIONFROMPOINT,
            WPARAM(x as usize),
            LPARAM(y as isize),
        );
        res
    }

    fn position_from_point_close(&self, x: i32, y: i32) -> isize {
        let (_, res) = self.send_message(
            SCI_POSITIONFROMPOINTCLOSE,
            WPARAM(x as usize),
            LPARAM(y as isize),
        );
        res as isize
    }

    fn point_x_from_position(&self, pos: usize) -> i32 {
        let (_, res) = self.send_message(
            SCI_POINTXFROMPOSITION,
            WPARAM::default(),
            LPARAM(pos as isize),
        );
        res as i32
    }

    fn point_y_from_position(&self, pos: usize) -> i32 {
        let (_, res) = self.send_message(
            SCI_POINTYFROMPOSITION,
            WPARAM::default(),
            LPARAM(pos as isize),
        );
        res as i32
    }

    fn position_relative(&self, pos: usize, relative: usize) -> usize {
        let (_, res) =
            self.send_message(SCI_POSITIONRELATIVE, WPARAM(pos), LPARAM(relative as isize));
        res
    }

    fn count_characters(&self, start: usize, end: usize) -> usize {
        let (_, res) = self.send_message(SCI_COUNTCHARACTERS, WPARAM(start), LPARAM(end as isize));
        res
    }

    fn set_status(&self, status: Status) {
        self.send_message(
            SCI_SETSTATUS,
            WPARAM(<Status as Into<u32>>::into(status) as usize),
            LPARAM::default(),
        );
    }

    fn get_status(&self) -> Status {
        let (_, res) = self.send_message(SCI_GETSTATUS, WPARAM::default(), LPARAM::default());
        (res as u32).into()
    }

    fn set_sel(&self, anchor: usize, caret: usize) {
        self.send_message(SCI_SETSEL, WPARAM(anchor), LPARAM(caret as isize));
    }

    fn goto_pos(&self, caret: usize) {
        self.send_message(SCI_GOTOPOS, WPARAM(caret), LPARAM::default());
    }

    fn goto_line(&self, line: usize) {
        self.send_message(SCI_GOTOLINE, WPARAM(line), LPARAM::default());
    }

    fn set_current_pos(&self, caret: usize) {
        self.send_message(SCI_SETCURRENTPOS, WPARAM(caret), LPARAM::default());
    }

    fn get_current_pos(&self) -> usize {
        let (_, res) = self.send_message(SCI_GETCURRENTPOS, WPARAM::default(), LPARAM::default());
        res
    }

    fn set_anchor(&self, anchor: usize) {
        self.send_message(SCI_SETANCHOR, WPARAM(anchor), LPARAM::default());
    }

    fn get_anchor(&self) -> usize {
        let (_, res) = self.send_message(SCI_GETANCHOR, WPARAM::default(), LPARAM::default());
        res
    }

    fn set_selection_start(&self, anchor: usize) {
        self.send_message(SCI_SETSELECTIONSTART, WPARAM(anchor), LPARAM::default());
    }

    fn set_selection_end(&self, caret: usize) {
        self.send_message(SCI_SETSELECTIONEND, WPARAM(caret), LPARAM::default());
    }

    fn get_selection_start(&self) -> usize {
        let (_, res) =
            self.send_message(SCI_GETSELECTIONSTART, WPARAM::default(), LPARAM::default());
        res
    }

    fn get_selection_end(&self) -> usize {
        let (_, res) = self.send_message(SCI_GETSELECTIONEND, WPARAM::default(), LPARAM::default());
        res
    }

    fn set_empty_selection(&self, caret: usize) {
        self.send_message(SCI_SETEMPTYSELECTION, WPARAM(caret), LPARAM::default());
    }

    fn select_all(&self) {
        self.send_message(SCI_SELECTALL, WPARAM::default(), LPARAM::default());
    }

    fn hide_selection(&self, hide: bool) {
        let hide = if hide { 1 } else { 0 };
        self.send_message(SCI_HIDESELECTION, WPARAM(hide), LPARAM::default());
    }

    fn get_sel_text(&self) -> Option<String> {
        let (_, len) = self.send_message(SCI_GETSELTEXT, WPARAM::default(), LPARAM::default());
        let mem = InProcessMemory::new(self.get_process_handle(), len + 1).unwrap();
        self.send_message(
            SCI_GETSELTEXT,
            WPARAM::default(),
            LPARAM(mem.as_ptr() as isize),
        );
        mem.read(|buf| (buf as *const u8).to_string())
    }

    fn get_cur_line(&self) -> (Option<String>, usize) {
        let (_, len) = self.send_message(SCI_GETCURLINE, WPARAM::default(), LPARAM::default());
        let mem = InProcessMemory::new(self.get_process_handle(), len + 1).unwrap();
        let (_, caret) =
            self.send_message(SCI_GETCURLINE, WPARAM(len), LPARAM(mem.as_ptr() as isize));
        (mem.read(|buf| (buf as *const u8).to_string()), caret)
    }

    fn selection_is_rectangle(&self) -> bool {
        let (_, res) = self.send_message(
            SCI_SELECTIONISRECTANGLE,
            WPARAM::default(),
            LPARAM::default(),
        );
        res != 0
    }

    fn set_selection_mode(&self, selection_mode: SelectionMode) {
        self.send_message(
            SCI_SETSELECTIONMODE,
            WPARAM(<SelectionMode as Into<u32>>::into(selection_mode) as usize),
            LPARAM::default(),
        );
    }

    fn get_selection_mode(&self) -> SelectionMode {
        let (_, res) =
            self.send_message(SCI_GETSELECTIONMODE, WPARAM::default(), LPARAM::default());
        SelectionMode::from(res as u32)
    }

    fn get_move_extends_selection(&self) -> bool {
        let (_, res) = self.send_message(
            SCI_GETMOVEEXTENDSSELECTION,
            WPARAM::default(),
            LPARAM::default(),
        );
        res != 0
    }

    fn get_line_sel_start_position(&self, line: usize) -> usize {
        let (_, res) =
            self.send_message(SCI_GETLINESELSTARTPOSITION, WPARAM(line), LPARAM::default());
        res
    }

    fn get_line_sel_end_position(&self, line: usize) -> usize {
        let (_, res) =
            self.send_message(SCI_GETLINESELENDPOSITION, WPARAM(line), LPARAM::default());
        res
    }

    fn move_caret_inside_view(&self) {
        self.send_message(
            SCI_MOVECARETINSIDEVIEW,
            WPARAM::default(),
            LPARAM::default(),
        );
    }

    fn choose_caret_x(&self) {
        self.send_message(SCI_CHOOSECARETX, WPARAM::default(), LPARAM::default());
    }

    fn move_selected_lines_up(&self) {
        self.send_message(
            SCI_MOVESELECTEDLINESUP,
            WPARAM::default(),
            LPARAM::default(),
        );
    }

    fn move_selected_lines_down(&self) {
        self.send_message(
            SCI_MOVESELECTEDLINESDOWN,
            WPARAM::default(),
            LPARAM::default(),
        );
    }

    fn set_mouse_selection_rectangular_switch(&self, mouse_selection_rectangular_switch: bool) {
        let mode = if mouse_selection_rectangular_switch {
            1
        } else {
            0
        };
        self.send_message(
            SCI_SETMOUSESELECTIONRECTANGULARSWITCH,
            WPARAM(mode),
            LPARAM::default(),
        );
    }

    fn get_mouse_selection_rectangular_switch(&self) -> bool {
        let (_, res) = self.send_message(
            SCI_GETMOUSESELECTIONRECTANGULARSWITCH,
            WPARAM::default(),
            LPARAM::default(),
        );
        res != 0
    }

    fn set_multiple_selection(&self, multiple_selection: bool) {
        let multiple_selection = if multiple_selection { 1 } else { 0 };
        self.send_message(
            SCI_SETMULTIPLESELECTION,
            WPARAM(multiple_selection),
            LPARAM::default(),
        );
    }

    fn get_multiple_selection(&self) -> bool {
        let (_, res) = self.send_message(
            SCI_GETMULTIPLESELECTION,
            WPARAM::default(),
            LPARAM::default(),
        );
        res != 0
    }

    fn set_additional_selection_typing(&self, additional_selection_typing: bool) {
        let additional_selection_typing = if additional_selection_typing { 1 } else { 0 };
        self.send_message(
            SCI_SETADDITIONALSELECTIONTYPING,
            WPARAM(additional_selection_typing),
            LPARAM::default(),
        );
    }

    fn get_additional_selection_typing(&self) -> bool {
        let (_, res) = self.send_message(
            SCI_GETADDITIONALSELECTIONTYPING,
            WPARAM::default(),
            LPARAM::default(),
        );
        res != 0
    }

    fn set_multi_paste(&self, multi_paste: bool) {
        let multi_paste = if multi_paste { 1 } else { 0 };
        self.send_message(SCI_SETMULTIPASTE, WPARAM(multi_paste), LPARAM::default());
    }

    fn get_multi_paste(&self) -> bool {
        let (_, res) = self.send_message(SCI_GETMULTIPASTE, WPARAM::default(), LPARAM::default());
        res != 0
    }

    fn set_virtual_space_options(&self, virtual_space: u32) {
        self.send_message(
            SCI_SETVIRTUALSPACEOPTIONS,
            WPARAM(virtual_space as usize),
            LPARAM::default(),
        );
    }

    fn get_virtual_space_options(&self) -> u32 {
        let (_, res) = self.send_message(
            SCI_GETVIRTUALSPACEOPTIONS,
            WPARAM::default(),
            LPARAM::default(),
        );
        res as u32
    }

    fn set_rectangular_selection_modifier(&self, modifier: u32) {
        self.send_message(
            SCI_SETRECTANGULARSELECTIONMODIFIER,
            WPARAM(modifier as usize),
            LPARAM::default(),
        );
    }

    fn get_rectangular_selection_modifier(&self) -> u32 {
        let (_, res) = self.send_message(
            SCI_GETRECTANGULARSELECTIONMODIFIER,
            WPARAM::default(),
            LPARAM::default(),
        );
        res as u32
    }

    fn get_selections(&self) -> i32 {
        let (_, res) = self.send_message(SCI_GETSELECTIONS, WPARAM::default(), LPARAM::default());
        res as i32
    }

    fn get_selection_empty(&self) -> bool {
        let (_, res) = self.send_message(SCI_GETSELECTIONS, WPARAM::default(), LPARAM::default());
        res != 0
    }

    fn clear_selections(&self) {
        self.send_message(SCI_CLEARSELECTIONS, WPARAM::default(), LPARAM::default());
    }

    fn set_selection(&self, caret: usize, anchor: usize) {
        self.send_message(SCI_SETSELECTION, WPARAM(caret), LPARAM(anchor as isize));
    }

    fn add_selection(&self, caret: usize, anchor: usize) {
        self.send_message(SCI_ADDSELECTION, WPARAM(caret), LPARAM(anchor as isize));
    }

    fn drop_selection_n(&self, selection: i32) {
        self.send_message(
            SCI_DROPSELECTIONN,
            WPARAM(selection as usize),
            LPARAM::default(),
        );
    }

    fn set_main_selection(&self, selection: i32) {
        self.send_message(
            SCI_SETMAINSELECTION,
            WPARAM(selection as usize),
            LPARAM::default(),
        );
    }

    fn get_main_selection(&self) -> i32 {
        let (_, res) =
            self.send_message(SCI_GETMAINSELECTION, WPARAM::default(), LPARAM::default());
        res as i32
    }

    fn set_selection_n_caret(&self, selection: i32, caret: usize) {
        self.send_message(
            SCI_SETSELECTIONNCARET,
            WPARAM(selection as usize),
            LPARAM(caret as isize),
        );
    }

    fn get_selection_n_caret(&self, selection: i32) -> usize {
        let (_, res) = self.send_message(
            SCI_GETSELECTIONNCARET,
            WPARAM(selection as usize),
            LPARAM::default(),
        );
        res
    }

    fn set_selection_n_caret_virtual_space(&self, selection: i32, space: usize) {
        self.send_message(
            SCI_SETSELECTIONNCARETVIRTUALSPACE,
            WPARAM(selection as usize),
            LPARAM(space as isize),
        );
    }

    fn get_selection_n_caret_virtual_space(&self, selection: i32) -> usize {
        let (_, res) = self.send_message(
            SCI_GETSELECTIONNCARETVIRTUALSPACE,
            WPARAM(selection as usize),
            LPARAM::default(),
        );
        res
    }

    fn set_selection_n_anchor(&self, selection: i32, anchor: usize) {
        self.send_message(
            SCI_SETSELECTIONNANCHOR,
            WPARAM(selection as usize),
            LPARAM(anchor as isize),
        );
    }

    fn get_selection_n_anchor(&self, selection: i32) -> usize {
        let (_, res) = self.send_message(
            SCI_GETSELECTIONNANCHOR,
            WPARAM(selection as usize),
            LPARAM::default(),
        );
        res
    }

    fn set_selection_n_anchor_virtual_space(&self, selection: i32, space: usize) {
        self.send_message(
            SCI_SETSELECTIONNANCHORVIRTUALSPACE,
            WPARAM(selection as usize),
            LPARAM(space as isize),
        );
    }

    fn get_selection_n_anchor_virtual_space(&self, selection: i32) -> usize {
        let (_, res) = self.send_message(
            SCI_GETSELECTIONNANCHORVIRTUALSPACE,
            WPARAM(selection as usize),
            LPARAM::default(),
        );
        res
    }

    fn set_selection_n_start(&self, selection: i32, anchor: usize) {
        self.send_message(
            SCI_SETSELECTIONNSTART,
            WPARAM(selection as usize),
            LPARAM(anchor as isize),
        );
    }

    fn get_selection_n_start(&self, selection: i32) -> usize {
        let (_, res) = self.send_message(
            SCI_GETSELECTIONNSTART,
            WPARAM(selection as usize),
            LPARAM::default(),
        );
        res
    }

    fn set_selection_n_end(&self, selection: i32, caret: usize) {
        self.send_message(
            SCI_SETSELECTIONNEND,
            WPARAM(selection as usize),
            LPARAM(caret as isize),
        );
    }

    fn get_selection_n_end(&self, selection: i32) -> usize {
        let (_, res) = self.send_message(
            SCI_GETSELECTIONNEND,
            WPARAM(selection as usize),
            LPARAM::default(),
        );
        res
    }

    fn set_rectangular_selection_caret(&self, caret: usize) {
        self.send_message(
            SCI_SETRECTANGULARSELECTIONCARET,
            WPARAM(caret),
            LPARAM::default(),
        );
    }

    fn get_rectangular_selection_caret(&self) -> usize {
        let (_, res) = self.send_message(
            SCI_GETRECTANGULARSELECTIONCARET,
            WPARAM::default(),
            LPARAM::default(),
        );
        res
    }

    fn set_rectangular_selection_caret_virtual_space(&self, space: usize) {
        self.send_message(
            SCI_SETRECTANGULARSELECTIONCARETVIRTUALSPACE,
            WPARAM(space),
            LPARAM::default(),
        );
    }

    fn get_rectangular_selection_caret_virtual_space(&self) -> usize {
        let (_, res) = self.send_message(
            SCI_GETRECTANGULARSELECTIONCARETVIRTUALSPACE,
            WPARAM::default(),
            LPARAM::default(),
        );
        res
    }

    fn set_rectangular_selection_anchor(&self, anchor: usize) {
        self.send_message(
            SCI_SETRECTANGULARSELECTIONCARET,
            WPARAM(anchor),
            LPARAM::default(),
        );
    }

    fn get_rectangular_selection_anchor(&self) -> usize {
        let (_, res) = self.send_message(
            SCI_GETRECTANGULARSELECTIONANCHOR,
            WPARAM::default(),
            LPARAM::default(),
        );
        res
    }

    fn set_rectangular_selection_anchor_virtual_space(&self, space: usize) {
        self.send_message(
            SCI_SETRECTANGULARSELECTIONCARET,
            WPARAM(space),
            LPARAM::default(),
        );
    }

    fn get_rectangular_selection_anchor_virtual_space(&self) -> usize {
        let (_, res) = self.send_message(
            SCI_GETRECTANGULARSELECTIONANCHORVIRTUALSPACE,
            WPARAM::default(),
            LPARAM::default(),
        );
        res
    }

    fn set_additional_sel_alpha(&self, alpha: i32) {
        self.send_message(
            SCI_SETADDITIONALSELALPHA,
            WPARAM(alpha as usize),
            LPARAM::default(),
        );
    }

    fn get_additional_sel_alpha(&self) -> i32 {
        let (_, res) = self.send_message(
            SCI_GETADDITIONALSELALPHA,
            WPARAM::default(),
            LPARAM::default(),
        );
        res as i32
    }

    fn set_additional_sel_fore(&self, fore: i32) {
        self.send_message(
            SCI_SETADDITIONALSELFORE,
            WPARAM(fore as usize),
            LPARAM::default(),
        );
    }

    fn set_additional_sel_back(&self, back: i32) {
        self.send_message(
            SCI_SETADDITIONALSELBACK,
            WPARAM(back as usize),
            LPARAM::default(),
        );
    }

    fn set_additional_caret_fore(&self, fore: i32) {
        self.send_message(
            SCI_SETADDITIONALCARETFORE,
            WPARAM(fore as usize),
            LPARAM::default(),
        );
    }

    fn get_additional_caret_fore(&self) -> i32 {
        let (_, res) = self.send_message(
            SCI_GETADDITIONALCARETFORE,
            WPARAM::default(),
            LPARAM::default(),
        );
        res as i32
    }

    fn set_additional_carets_blink(&self, additional_carets_blink: bool) {
        let additional_carets_blink = if additional_carets_blink { 1 } else { 0 };
        self.send_message(
            SCI_SETADDITIONALCARETSBLINK,
            WPARAM(additional_carets_blink as usize),
            LPARAM::default(),
        );
    }

    fn get_additional_carets_blink(&self) -> bool {
        let (_, res) = self.send_message(
            SCI_GETADDITIONALCARETSBLINK,
            WPARAM::default(),
            LPARAM::default(),
        );
        res != 0
    }

    fn set_additional_carets_visible(&self, visible: bool) {
        let visible = if visible { 1 } else { 0 };
        self.send_message(
            SCI_SETADDITIONALCARETSVISIBLE,
            WPARAM(visible as usize),
            LPARAM::default(),
        );
    }

    fn get_additional_carets_visible(&self) -> bool {
        let (_, res) = self.send_message(
            SCI_GETADDITIONALCARETSVISIBLE,
            WPARAM::default(),
            LPARAM::default(),
        );
        res != 0
    }

    fn swap_main_anchor_caret(&self) {
        self.send_message(
            SCI_SWAPMAINANCHORCARET,
            WPARAM::default(),
            LPARAM::default(),
        );
    }

    fn rotate_selection(&self) {
        self.send_message(SCI_ROTATESELECTION, WPARAM::default(), LPARAM::default());
    }

    fn multiple_select_add_next(&self) {
        self.send_message(
            SCI_MULTIPLESELECTADDNEXT,
            WPARAM::default(),
            LPARAM::default(),
        );
    }

    fn multiple_select_add_each(&self) {
        self.send_message(
            SCI_MULTIPLESELECTADDEACH,
            WPARAM::default(),
            LPARAM::default(),
        );
    }

    fn set_over_type(&self, over_type: bool) {
        let over_type = if over_type { 1 } else { 0 };
        self.send_message(SCI_SETOVERTYPE, WPARAM(over_type), LPARAM::default());
    }

    fn get_over_type(&self) -> bool {
        let (_, res) = self.send_message(SCI_GETOVERTYPE, WPARAM::default(), LPARAM::default());
        res != 0
    }

    fn set_target_start(&self, start: usize) {
        self.send_message(SCI_SETTARGETSTART, WPARAM(start), LPARAM::default());
    }

    fn get_target_start(&self) -> usize {
        let (_, res) = self.send_message(SCI_GETTARGETSTART, WPARAM::default(), LPARAM::default());
        res
    }

    fn set_target_end(&self, end: usize) {
        self.send_message(SCI_SETTARGETEND, WPARAM(end), LPARAM::default());
    }

    fn get_target_end(&self) -> usize {
        let (_, res) = self.send_message(SCI_GETTARGETEND, WPARAM::default(), LPARAM::default());
        res
    }

    fn set_target_range(&self, start: usize, end: usize) {
        self.send_message(SCI_SETTARGETRANGE, WPARAM(start), LPARAM(end as isize));
    }

    fn target_from_selection(&self) {
        self.send_message(
            SCI_TARGETFROMSELECTION,
            WPARAM::default(),
            LPARAM::default(),
        );
    }

    fn target_whole_document(&self) {
        self.send_message(
            SCI_TARGETWHOLEDOCUMENT,
            WPARAM::default(),
            LPARAM::default(),
        );
    }

    fn set_search_flags(&self, search_flags: u32) {
        self.send_message(
            SCI_SETSEARCHFLAGS,
            WPARAM(search_flags as usize),
            LPARAM::default(),
        );
    }

    fn get_search_flags(&self) -> u32 {
        let (_, res) = self.send_message(SCI_GETSEARCHFLAGS, WPARAM::default(), LPARAM::default());
        res as u32
    }

    fn search_in_target(&self, text: String) -> usize {
        let length = text.as_bytes().len();
        let mem = InProcessMemory::new(self.get_process_handle(), length + 1).unwrap();
        mem.write(text.as_ptr() as *const c_void, length);
        let (_, res) = self.send_message(
            SCI_SEARCHINTARGET,
            WPARAM(length),
            LPARAM(mem.as_ptr() as isize),
        );
        res
    }

    fn get_target_text(&self, length: usize) -> Option<String> {
        let mem = InProcessMemory::new(self.get_process_handle(), length).unwrap();
        self.send_message(
            SCI_GETTARGETTEXT,
            WPARAM(length),
            LPARAM(mem.as_ptr() as isize),
        );
        mem.read(|buf| (buf as *const u8).to_string())
    }

    fn replace_target(&self, text: String) -> usize {
        let length = text.as_bytes().len();
        let mem = InProcessMemory::new(self.get_process_handle(), length + 1).unwrap();
        mem.write(text.as_ptr() as *const c_void, length);
        let (_, res) = self.send_message(
            SCI_REPLACETARGET,
            WPARAM(length),
            LPARAM(mem.as_ptr() as isize),
        );
        res
    }

    fn replace_target_re(&self, text: String) -> usize {
        let length = text.as_bytes().len();
        let mem = InProcessMemory::new(self.get_process_handle(), length + 1).unwrap();
        mem.write(text.as_ptr() as *const c_void, length);
        let (_, res) = self.send_message(
            SCI_REPLACETARGETRE,
            WPARAM(length),
            LPARAM(mem.as_ptr() as isize),
        );
        res
    }

    fn get_tag(&self, tag_number: i32, length: usize) -> (i32, Option<String>) {
        let mem = InProcessMemory::new(self.get_process_handle(), length).unwrap();
        let (_, res) = self.send_message(
            SCI_GETTAG,
            WPARAM(tag_number as usize),
            LPARAM(mem.as_ptr() as isize),
        );
        (res as i32, mem.read(|buf| (buf as *const u8).to_string()))
    }

    fn find_text(
        &self,
        text: String,
        min: isize,
        max: isize,
        search_flags: u32,
    ) -> (usize, Option<(usize, usize)>) {
        let length = text.as_bytes().len();
        let mem = InProcessMemory::new(self.get_process_handle(), length + 1).unwrap();
        mem.write(text.as_ptr() as *const c_void, length);
        let param = Sci_TextToFind {
            chrg: Sci_CharacterRange {
                cpMax: max as Sci_PositionCR,
                cpMin: min as Sci_PositionCR,
            },
            lpstrText: mem.as_ptr() as *const c_char,
            chrgText: Sci_CharacterRange { cpMin: 0, cpMax: 0 },
        };
        let size = std::mem::size_of::<Sci_TextToFind>();
        let mem2 = InProcessMemory::new(self.get_process_handle(), size).unwrap();
        mem2.write(&param as *const Sci_TextToFind as *const c_void, size);
        let (_, res) = self.send_message(
            SCI_FINDTEXT,
            WPARAM(search_flags as usize),
            LPARAM(mem2.as_ptr() as isize),
        );
        let range = mem2.read(|buf| unsafe { (buf as *const Sci_TextToFind).read().chrgText });
        match range {
            None => (res, None),
            Some(r) => (res, Some((r.cpMin as usize, r.cpMax as usize))),
        }
    }

    fn search_anchor(&self) {
        self.send_message(SCI_SEARCHANCHOR, WPARAM::default(), LPARAM::default());
    }

    fn search_prev(&self, search_flags: u32, text: String) -> usize {
        let length = text.as_bytes().len();
        let mem = InProcessMemory::new(self.get_process_handle(), length + 1).unwrap();
        mem.write(text.as_ptr() as *const c_void, length);
        let (_, res) = self.send_message(
            SCI_SEARCHPREV,
            WPARAM(search_flags as usize),
            LPARAM(mem.as_ptr() as isize),
        );
        res
    }

    fn search_next(&self, search_flags: u32, text: String) -> usize {
        let length = text.as_bytes().len();
        let mem = InProcessMemory::new(self.get_process_handle(), length + 1).unwrap();
        mem.write(text.as_ptr() as *const c_void, length);
        let (_, res) = self.send_message(
            SCI_SEARCHNEXT,
            WPARAM(search_flags as usize),
            LPARAM(mem.as_ptr() as isize),
        );
        res
    }

    fn cut(&self) {
        self.send_message(SCI_CUT, WPARAM::default(), LPARAM::default());
    }

    fn copy(&self) {
        self.send_message(SCI_COPY, WPARAM::default(), LPARAM::default());
    }

    fn paste(&self) {
        self.send_message(SCI_PASTE, WPARAM::default(), LPARAM::default());
    }

    fn clear(&self) {
        self.send_message(SCI_CLEAR, WPARAM::default(), LPARAM::default());
    }

    fn can_paste(&self) -> bool {
        let (_, res) = self.send_message(SCI_CUT, WPARAM::default(), LPARAM::default());
        res != 0
    }

    fn copy_allow_line(&self) {
        self.send_message(SCI_COPYALLOWLINE, WPARAM::default(), LPARAM::default());
    }

    fn copy_range(&self, start: usize, end: usize) {
        self.send_message(SCI_COPYRANGE, WPARAM(start), LPARAM(end as isize));
    }

    fn copy_text(&self, text: String) {
        let lentth = text.as_bytes().len();
        let mem = InProcessMemory::new(self.get_process_handle(), lentth + 1).unwrap();
        mem.write(text.as_ptr() as *const c_void, lentth);
        self.send_message(SCI_COPYTEXT, WPARAM(lentth), LPARAM(mem.as_ptr() as isize));
    }

    fn set_paste_convert_endings(&self, convert: bool) {
        let convert = if convert { 1 } else { 0 };
        self.send_message(
            SCI_SETPASTECONVERTENDINGS,
            WPARAM(convert),
            LPARAM::default(),
        );
    }

    fn get_paste_convert_endings(&self) -> bool {
        let (_, res) = self.send_message(
            SCI_GETPASTECONVERTENDINGS,
            WPARAM::default(),
            LPARAM::default(),
        );
        res != 0
    }

    fn undo(&self) {
        self.send_message(SCI_UNDO, WPARAM::default(), LPARAM::default());
    }

    fn can_undo(&self) -> bool {
        let (_, res) = self.send_message(SCI_CANUNDO, WPARAM::default(), LPARAM::default());
        res != 0
    }

    fn redo(&self) {
        self.send_message(SCI_REDO, WPARAM::default(), LPARAM::default());
    }

    fn can_redo(&self) -> bool {
        let (_, res) = self.send_message(SCI_CANREDO, WPARAM::default(), LPARAM::default());
        res != 0
    }

    fn empty_undo_buffer(&self) {
        self.send_message(SCI_EMPTYUNDOBUFFER, WPARAM::default(), LPARAM::default());
    }

    fn set_undo_collection(&self, collect_undo: bool) {
        let collect_undo = if collect_undo { 1 } else { 0 };
        self.send_message(
            SCI_SETUNDOCOLLECTION,
            WPARAM(collect_undo),
            LPARAM::default(),
        );
    }

    fn get_undo_collection(&self) -> bool {
        let (_, res) =
            self.send_message(SCI_GETUNDOCOLLECTION, WPARAM::default(), LPARAM::default());
        res != 0
    }

    fn begin_undo_action(&self) {
        self.send_message(SCI_BEGINUNDOACTION, WPARAM::default(), LPARAM::default());
    }

    fn end_undo_action(&self) {
        self.send_message(SCI_ENDUNDOACTION, WPARAM::default(), LPARAM::default());
    }

    fn add_undo_action(&self, token: i32, flags: u32) {
        self.send_message(
            SCI_ADDUNDOACTION,
            WPARAM(token as usize),
            LPARAM(flags as isize),
        );
    }

    fn set_first_visible_line(&self, line: usize) {
        self.send_message(SCI_SETFIRSTVISIBLELINE, WPARAM(line), LPARAM::default());
    }

    fn get_first_visible_line(&self) -> usize {
        let (_, res) = self.send_message(
            SCI_GETFIRSTVISIBLELINE,
            WPARAM::default(),
            LPARAM::default(),
        );
        res
    }

    fn set_x_offset(&self, x_offset: i32) {
        self.send_message(SCI_SETXOFFSET, WPARAM(x_offset as usize), LPARAM::default());
    }

    fn get_x_offset(&self) -> i32 {
        let (_, res) = self.send_message(SCI_GETXOFFSET, WPARAM::default(), LPARAM::default());
        res as i32
    }

    fn line_scroll(&self, columns: usize, lines: usize) {
        self.send_message(SCI_LINESCROLL, WPARAM(columns), LPARAM(lines as isize));
    }

    fn scroll_caret(&self) {
        self.send_message(SCI_SCROLLCARET, WPARAM::default(), LPARAM::default());
    }

    fn scroll_range(&self, secondary: usize, primary: usize) {
        self.send_message(SCI_SCROLLRANGE, WPARAM(secondary), LPARAM(primary as isize));
    }

    fn set_x_caret_policy(&self, caret_policy: u32, caret_slop: i32) {
        self.send_message(
            SCI_SETXCARETPOLICY,
            WPARAM(caret_policy as usize),
            LPARAM(caret_slop as isize),
        );
    }

    fn set_y_caret_policy(&self, caret_policy: u32, caret_slop: i32) {
        self.send_message(
            SCI_SETYCARETPOLICY,
            WPARAM(caret_policy as usize),
            LPARAM(caret_slop as isize),
        );
    }

    fn set_visible_policy(&self, visible_policy: u32, visible_slop: i32) {
        self.send_message(
            SCI_SETVISIBLEPOLICY,
            WPARAM(visible_policy as usize),
            LPARAM(visible_slop as isize),
        );
    }

    fn set_h_scroll_bar(&self, visible: bool) {
        let visible = if visible { 1 } else { 0 };
        self.send_message(SCI_SETHSCROLLBAR, WPARAM(visible), LPARAM::default());
    }

    fn get_h_scroll_bar(&self) -> bool {
        let (_, res) = self.send_message(SCI_GETHSCROLLBAR, WPARAM::default(), LPARAM::default());
        res != 0
    }

    fn set_v_scroll_bar(&self, visible: bool) {
        let visible = if visible { 1 } else { 0 };
        self.send_message(SCI_SETVSCROLLBAR, WPARAM(visible), LPARAM::default());
    }

    fn get_v_scroll_bar(&self) -> bool {
        let (_, res) = self.send_message(SCI_GETVSCROLLBAR, WPARAM::default(), LPARAM::default());
        res != 0
    }

    fn set_scroll_width(&self, pixel_width: i32) {
        self.send_message(
            SCI_SETSCROLLWIDTH,
            WPARAM(pixel_width as usize),
            LPARAM::default(),
        );
    }

    fn get_scroll_width(&self) -> i32 {
        let (_, res) = self.send_message(SCI_GETSCROLLWIDTH, WPARAM::default(), LPARAM::default());
        res as i32
    }

    fn set_scroll_width_tracking(&self, tracking: bool) {
        let tracking = if tracking { 1 } else { 0 };
        self.send_message(
            SCI_SETSCROLLWIDTHTRACKING,
            WPARAM(tracking),
            LPARAM::default(),
        );
    }

    fn get_scroll_width_tracking(&self) -> bool {
        let (_, res) = self.send_message(
            SCI_GETSCROLLWIDTHTRACKING,
            WPARAM::default(),
            LPARAM::default(),
        );
        res != 0
    }

    fn set_end_at_last_line(&self, end_at_last_line: bool) {
        let end_at_last_line = if end_at_last_line { 1 } else { 0 };
        self.send_message(
            SCI_SETENDATLASTLINE,
            WPARAM(end_at_last_line),
            LPARAM::default(),
        );
    }

    fn get_end_at_last_line(&self) -> bool {
        let (_, res) =
            self.send_message(SCI_GETENDATLASTLINE, WPARAM::default(), LPARAM::default());
        res != 0
    }

    fn set_view_ws(&self, view_ws: WhiteSpace) {
        self.send_message(
            SCI_SETVIEWWS,
            WPARAM(Into::<u32>::into(view_ws) as usize),
            LPARAM::default(),
        );
    }

    fn get_view_ws(&self) -> WhiteSpace {
        let (_, res) = self.send_message(SCI_GETVIEWWS, WPARAM::default(), LPARAM::default());
        WhiteSpace::from(res as u32)
    }

    fn set_white_space_fore(&self, use_setting: bool, fore: i32) {
        let use_setting = if use_setting { 1 } else { 0 };
        self.send_message(
            SCI_SETWHITESPACEFORE,
            WPARAM(use_setting),
            LPARAM(fore as isize),
        );
    }

    fn set_white_space_back(&self, use_setting: bool, back: i32) {
        let use_setting = if use_setting { 1 } else { 0 };
        self.send_message(
            SCI_SETWHITESPACEBACK,
            WPARAM(use_setting),
            LPARAM(back as isize),
        );
    }

    fn set_white_space_size(&self, size: i32) {
        self.send_message(
            SCI_SETWHITESPACESIZE,
            WPARAM(size as usize),
            LPARAM::default(),
        );
    }

    fn get_white_space_size(&self) -> i32 {
        let (_, res) =
            self.send_message(SCI_GETWHITESPACESIZE, WPARAM::default(), LPARAM::default());
        res as i32
    }

    fn set_tab_draw_mode(&self, tab_draw_mode: TabDrawMode) {
        self.send_message(
            SCI_SETTABDRAWMODE,
            WPARAM(Into::<u32>::into(tab_draw_mode) as usize),
            LPARAM::default(),
        );
    }

    fn get_tab_draw_mode(&self) -> TabDrawMode {
        let (_, res) = self.send_message(SCI_GETTABDRAWMODE, WPARAM::default(), LPARAM::default());
        TabDrawMode::from(res as u32)
    }

    fn set_extra_ascent(&self, ascent: i32) {
        self.send_message(
            SCI_SETEXTRAASCENT,
            WPARAM(ascent as usize),
            LPARAM::default(),
        );
    }

    fn get_extra_ascent(&self) -> i32 {
        let (_, res) = self.send_message(SCI_GETEXTRAASCENT, WPARAM::default(), LPARAM::default());
        res as i32
    }

    fn set_extra_descent(&self, descent: i32) {
        self.send_message(
            SCI_SETEXTRADESCENT,
            WPARAM(descent as usize),
            LPARAM::default(),
        );
    }

    fn get_extra_descent(&self) -> i32 {
        let (_, res) = self.send_message(SCI_GETEXTRADESCENT, WPARAM::default(), LPARAM::default());
        res as i32
    }

    fn set_cursor(&self, cursor_type: u32) {
        self.send_message(
            SCI_SETCURSOR,
            WPARAM(cursor_type as usize),
            LPARAM::default(),
        );
    }

    fn get_cursor(&self) -> u32 {
        let (_, res) = self.send_message(SCI_GETCURSOR, WPARAM::default(), LPARAM::default());
        res as u32
    }

    fn set_mouse_down_captures(&self, captures: bool) {
        let captures = if captures { 1 } else { 0 };
        self.send_message(
            SCI_SETMOUSEDOWNCAPTURES,
            WPARAM(captures),
            LPARAM::default(),
        );
    }

    fn get_mouse_down_captures(&self) -> bool {
        let (_, res) = self.send_message(
            SCI_GETMOUSEDOWNCAPTURES,
            WPARAM::default(),
            LPARAM::default(),
        );
        res != 0
    }

    fn set_mouse_wheel_captures(&self, captures: bool) {
        let captures = if captures { 1 } else { 0 };
        self.send_message(
            SCI_SETMOUSEWHEELCAPTURES,
            WPARAM(captures),
            LPARAM::default(),
        );
    }

    fn get_mouse_wheel_captures(&self) -> bool {
        let (_, res) = self.send_message(
            SCI_GETMOUSEWHEELCAPTURES,
            WPARAM::default(),
            LPARAM::default(),
        );
        res != 0
    }

    fn set_eol_mode(&self, eol_mode: EolMode) {
        self.send_message(
            SCI_SETEOLMODE,
            WPARAM(Into::<u32>::into(eol_mode) as usize),
            LPARAM::default(),
        );
    }

    fn get_eol_mode(&self) -> EolMode {
        let (_, res) = self.send_message(SCI_GETEOLMODE, WPARAM::default(), LPARAM::default());
        EolMode::from(res as u32)
    }

    fn convert_eols(&self, eol_mode: EolMode) {
        self.send_message(
            SCI_CONVERTEOLS,
            WPARAM(Into::<u32>::into(eol_mode) as usize),
            LPARAM::default(),
        );
    }

    fn set_view_eol(&self, visible: bool) {
        let visible = if visible { 1 } else { 0 };
        self.send_message(SCI_SETVIEWEOL, WPARAM(visible), LPARAM::default());
    }

    fn get_view_eol(&self) -> bool {
        let (_, res) = self.send_message(SCI_GETVIEWEOL, WPARAM::default(), LPARAM::default());
        res != 0
    }

    fn get_line_end_types_supported(&self) -> u32 {
        let (_, res) = self.send_message(
            SCI_GETLINEENDTYPESSUPPORTED,
            WPARAM::default(),
            LPARAM::default(),
        );
        res as u32
    }

    fn set_line_end_types_allowed(&self, line_end_bit_set: u32) {
        self.send_message(
            SCI_SETLINEENDTYPESALLOWED,
            WPARAM(line_end_bit_set as usize),
            LPARAM::default(),
        );
    }

    fn get_line_end_types_allowed(&self) -> u32 {
        let (_, res) = self.send_message(
            SCI_GETLINEENDTYPESALLOWED,
            WPARAM::default(),
            LPARAM::default(),
        );
        res as u32
    }

    fn get_line_end_types_active(&self) -> u32 {
        let (_, res) = self.send_message(
            SCI_GETLINEENDTYPESACTIVE,
            WPARAM::default(),
            LPARAM::default(),
        );
        res as u32
    }

    fn word_start_position(&self, pos: usize, only_word_characters: bool) -> usize {
        let only_word_characters = if only_word_characters { 1 } else { 0 };
        let (_, res) = self.send_message(
            SCI_WORDSTARTPOSITION,
            WPARAM(pos),
            LPARAM(only_word_characters),
        );
        res
    }

    fn word_end_position(&self, pos: usize, only_word_characters: bool) -> usize {
        let only_word_characters = if only_word_characters { 1 } else { 0 };
        let (_, res) = self.send_message(
            SCI_WORDENDPOSITION,
            WPARAM(pos),
            LPARAM(only_word_characters),
        );
        res
    }

    fn is_range_word(&self, start: usize, end: usize) -> bool {
        let (_, res) = self.send_message(SCI_ISRANGEWORD, WPARAM(start), LPARAM(end as isize));
        res != 0
    }

    fn set_word_chars(&self, characters: String) {
        let length = characters.as_bytes().len();
        let mem = InProcessMemory::new(self.get_process_handle(), length + 1).unwrap();
        mem.write(characters.as_ptr() as *const c_void, length);
        self.send_message(
            SCI_SETWORDCHARS,
            WPARAM(length),
            LPARAM(mem.as_ptr() as isize),
        );
    }

    fn get_word_chars(&self) -> Option<String> {
        let (_, length) = self.send_message(SCI_GETWORDCHARS, WPARAM::default(), LPARAM::default());
        let mem = InProcessMemory::new(self.get_process_handle(), length + 1).unwrap();
        self.send_message(
            SCI_GETWORDCHARS,
            WPARAM(length),
            LPARAM(mem.as_ptr() as isize),
        );
        mem.read(|buf| (buf as *const u8).to_string())
    }

    fn set_white_space_chars(&self, characters: String) {
        let length = characters.as_bytes().len();
        let mem = InProcessMemory::new(self.get_process_handle(), length + 1).unwrap();
        mem.write(characters.as_ptr() as *const c_void, length);
        self.send_message(
            SCI_SETWHITESPACECHARS,
            WPARAM(length),
            LPARAM(mem.as_ptr() as isize),
        );
    }

    fn get_white_space_chars(&self) -> Option<String> {
        let (_, length) =
            self.send_message(SCI_GETWHITESPACECHARS, WPARAM::default(), LPARAM::default());
        let mem = InProcessMemory::new(self.get_process_handle(), length + 1).unwrap();
        self.send_message(
            SCI_GETWHITESPACECHARS,
            WPARAM(length),
            LPARAM(mem.as_ptr() as isize),
        );
        mem.read(|buf| (buf as *const u8).to_string())
    }

    fn set_punctuation_chars(&self, characters: String) {
        let length = characters.as_bytes().len();
        let mem = InProcessMemory::new(self.get_process_handle(), length + 1).unwrap();
        mem.write(characters.as_ptr() as *const c_void, length);
        self.send_message(
            SCI_SETPUNCTUATIONCHARS,
            WPARAM(length),
            LPARAM(mem.as_ptr() as isize),
        );
    }

    fn get_punctuation_chars(&self) -> Option<String> {
        let (_, length) = self.send_message(
            SCI_GETPUNCTUATIONCHARS,
            WPARAM::default(),
            LPARAM::default(),
        );
        let mem = InProcessMemory::new(self.get_process_handle(), length + 1).unwrap();
        self.send_message(
            SCI_GETPUNCTUATIONCHARS,
            WPARAM(length),
            LPARAM(mem.as_ptr() as isize),
        );
        mem.read(|buf| (buf as *const u8).to_string())
    }

    fn set_chars_default(&self) {
        self.send_message(SCI_SETCHARSDEFAULT, WPARAM::default(), LPARAM::default());
    }

    fn word_left(&self) {
        self.send_message(SCI_WORDLEFT, WPARAM::default(), LPARAM::default());
    }

    fn word_left_extend(&self) {
        self.send_message(SCI_WORDLEFTEXTEND, WPARAM::default(), LPARAM::default());
    }

    fn word_right(&self) {
        self.send_message(SCI_WORDRIGHT, WPARAM::default(), LPARAM::default());
    }

    fn word_right_extend(&self) {
        self.send_message(SCI_WORDRIGHTEXTEND, WPARAM::default(), LPARAM::default());
    }

    fn word_left_end(&self) {
        self.send_message(SCI_WORDLEFTEND, WPARAM::default(), LPARAM::default());
    }

    fn word_left_end_extend(&self) {
        self.send_message(SCI_WORDLEFTENDEXTEND, WPARAM::default(), LPARAM::default());
    }

    fn word_right_end(&self) {
        self.send_message(SCI_WORDRIGHTEND, WPARAM::default(), LPARAM::default());
    }

    fn word_right_end_extend(&self) {
        self.send_message(SCI_WORDRIGHTENDEXTEND, WPARAM::default(), LPARAM::default());
    }

    fn word_part_left(&self) {
        self.send_message(SCI_WORDPARTLEFT, WPARAM::default(), LPARAM::default());
    }

    fn word_part_left_extend(&self) {
        self.send_message(SCI_WORDPARTLEFTEXTEND, WPARAM::default(), LPARAM::default());
    }

    fn word_part_right(&self) {
        self.send_message(SCI_WORDPARTRIGHT, WPARAM::default(), LPARAM::default());
    }

    fn word_part_right_extend(&self) {
        self.send_message(
            SCI_WORDPARTRIGHTEXTEND,
            WPARAM::default(),
            LPARAM::default(),
        );
    }

    fn del_word_left(&self) {
        self.send_message(SCI_DELWORDLEFT, WPARAM::default(), LPARAM::default());
    }

    fn del_word_right(&self) {
        self.send_message(SCI_DELWORDRIGHT, WPARAM::default(), LPARAM::default());
    }

    fn del_word_right_end(&self) {
        self.send_message(SCI_DELWORDRIGHTEND, WPARAM::default(), LPARAM::default());
    }

    fn get_end_styled(&self) -> usize {
        let (_, res) = self.send_message(SCI_GETENDSTYLED, WPARAM::default(), LPARAM::default());
        res
    }

    fn start_styling(&self, start: usize) {
        self.send_message(SCI_STARTSTYLING, WPARAM(start), LPARAM::default());
    }

    fn set_styling(&self, length: usize, style: i32) {
        self.send_message(SCI_SETSTYLING, WPARAM(length), LPARAM(style as isize));
    }

    fn set_styling_ex(&self, styles: &[u8]) {
        let size = styles.len();
        let mem = InProcessMemory::new(self.get_process_handle(), size + 1).unwrap();
        mem.write(styles.as_ptr() as *const c_void, size);
        self.send_message(
            SCI_SETSTYLINGEX,
            WPARAM(size),
            LPARAM(mem.as_ptr() as isize),
        );
    }

    fn set_idle_styling(&self, idle_styling: IdleStyling) {
        self.send_message(
            SCI_SETIDLESTYLING,
            WPARAM(Into::<u32>::into(idle_styling) as usize),
            LPARAM::default(),
        );
    }

    fn get_idle_styling(&self) -> IdleStyling {
        let (_, res) = self.send_message(SCI_GETIDLESTYLING, WPARAM::default(), LPARAM::default());
        IdleStyling::from(res as u32)
    }

    fn set_line_state(&self, line: usize, state: i32) {
        self.send_message(SCI_SETLINESTATE, WPARAM(line), LPARAM(state as isize));
    }

    fn get_line_state(&self, line: usize) -> i32 {
        let (_, res) = self.send_message(SCI_GETLINESTATE, WPARAM(line), LPARAM::default());
        res as i32
    }

    fn get_max_line_state(&self) -> i32 {
        let (_, res) = self.send_message(SCI_GETMAXLINESTATE, WPARAM::default(), LPARAM::default());
        res as i32
    }

    fn style_reset_default(&self) {
        self.send_message(SCI_STYLERESETDEFAULT, WPARAM::default(), LPARAM::default());
    }

    fn style_clear_all(&self) {
        self.send_message(SCI_STYLECLEARALL, WPARAM::default(), LPARAM::default());
    }

    fn style_set_font(&self, style: i32, font: String) {
        let length = font.as_bytes().len();
        let mem = InProcessMemory::new(self.get_process_handle(), length + 1).unwrap();
        mem.write(font.as_ptr() as *const c_void, length);
        self.send_message(
            SCI_STYLESETFONT,
            WPARAM(style as usize),
            LPARAM(mem.as_ptr() as isize),
        );
    }

    fn style_get_font(&self, style: i32) -> Option<String> {
        let (_, length) = self.send_message(SCI_STYLEGETFONT, WPARAM::default(), LPARAM::default());
        let mem = InProcessMemory::new(self.get_process_handle(), length + 1).unwrap();
        self.send_message(
            SCI_STYLEGETFONT,
            WPARAM(style as usize),
            LPARAM(mem.as_ptr() as isize),
        );
        mem.read(|buf| (buf as *const u8).to_string())
    }

    fn style_set_size(&self, style: i32, size_points: i32) {
        self.send_message(
            SCI_STYLESETSIZE,
            WPARAM(style as usize),
            LPARAM(size_points as isize),
        );
    }

    fn style_get_size(&self, style: i32) -> i32 {
        let (_, res) =
            self.send_message(SCI_STYLEGETSIZE, WPARAM(style as usize), LPARAM::default());
        res as i32
    }

    fn style_set_size_fractional(&self, style: i32, size_hundredth_points: i32) {
        self.send_message(
            SCI_STYLESETSIZEFRACTIONAL,
            WPARAM(style as usize),
            LPARAM(size_hundredth_points as isize),
        );
    }

    fn style_get_size_fractional(&self, style: i32) -> i32 {
        let (_, res) = self.send_message(
            SCI_STYLEGETSIZEFRACTIONAL,
            WPARAM(style as usize),
            LPARAM::default(),
        );
        res as i32
    }

    fn style_set_bold(&self, style: i32, bold: bool) {
        let bold = if bold { 1 } else { 0 };
        self.send_message(SCI_STYLESETBOLD, WPARAM(style as usize), LPARAM(bold));
    }

    fn style_get_bold(&self, style: i32) -> bool {
        let (_, res) =
            self.send_message(SCI_STYLEGETBOLD, WPARAM(style as usize), LPARAM::default());
        res != 0
    }

    fn style_set_weight(&self, style: i32, weight: i32) {
        self.send_message(
            SCI_STYLESETWEIGHT,
            WPARAM(style as usize),
            LPARAM(weight as isize),
        );
    }

    fn style_get_weight(&self, style: i32) -> i32 {
        let (_, res) = self.send_message(
            SCI_STYLEGETWEIGHT,
            WPARAM(style as usize),
            LPARAM::default(),
        );
        res as i32
    }

    fn style_set_italic(&self, style: i32, italic: bool) {
        let italic = if italic { 1 } else { 0 };
        self.send_message(SCI_STYLESETITALIC, WPARAM(style as usize), LPARAM(italic));
    }

    fn style_get_italic(&self, style: i32) -> bool {
        let (_, res) = self.send_message(
            SCI_STYLEGETITALIC,
            WPARAM(style as usize),
            LPARAM::default(),
        );
        res != 0
    }

    fn style_set_underline(&self, style: i32, underline: bool) {
        let underline = if underline { 1 } else { 0 };
        self.send_message(
            SCI_STYLESETUNDERLINE,
            WPARAM(style as usize),
            LPARAM(underline),
        );
    }

    fn style_get_underline(&self, style: i32) -> bool {
        let (_, res) = self.send_message(
            SCI_STYLEGETUNDERLINE,
            WPARAM(style as usize),
            LPARAM::default(),
        );
        res != 0
    }

    fn style_set_fore(&self, style: i32, fore: i32) {
        self.send_message(
            SCI_STYLESETFORE,
            WPARAM(style as usize),
            LPARAM(fore as isize),
        );
    }

    fn style_get_fore(&self, style: i32) -> i32 {
        let (_, res) =
            self.send_message(SCI_STYLEGETFORE, WPARAM(style as usize), LPARAM::default());
        res as i32
    }

    fn style_set_back(&self, style: i32, back: i32) {
        self.send_message(
            SCI_STYLESETBACK,
            WPARAM(style as usize),
            LPARAM(back as isize),
        );
    }

    fn style_get_back(&self, style: i32) -> i32 {
        let (_, res) =
            self.send_message(SCI_STYLEGETBACK, WPARAM(style as usize), LPARAM::default());
        res as i32
    }

    fn style_set_eol_filled(&self, style: i32, eol_filled: bool) {
        let eol_filled = if eol_filled { 1 } else { 0 };
        self.send_message(
            SCI_STYLESETEOLFILLED,
            WPARAM(style as usize),
            LPARAM(eol_filled),
        );
    }

    fn style_get_eol_filled(&self, style: i32) -> bool {
        let (_, res) = self.send_message(
            SCI_STYLEGETEOLFILLED,
            WPARAM(style as usize),
            LPARAM::default(),
        );
        res != 0
    }

    fn style_set_character_set(&self, style: i32, charset_set: CharacterSet) {
        self.send_message(
            SCI_STYLESETCHARACTERSET,
            WPARAM(style as usize),
            LPARAM(Into::<u32>::into(charset_set) as isize),
        );
    }

    fn style_get_character_set(&self, style: i32) -> CharacterSet {
        let (_, res) = self.send_message(
            SCI_STYLEGETCHARACTERSET,
            WPARAM(style as usize),
            LPARAM::default(),
        );
        CharacterSet::from(res as u32)
    }

    fn style_set_case(&self, style: i32, case_visible: Case) {
        self.send_message(
            SCI_STYLESETCASE,
            WPARAM(style as usize),
            LPARAM(Into::<u32>::into(case_visible) as isize),
        );
    }

    fn style_get_case(&self, style: i32) -> Case {
        let (_, res) =
            self.send_message(SCI_STYLEGETCASE, WPARAM(style as usize), LPARAM::default());
        Case::from(res as u32)
    }

    fn style_set_visible(&self, style: i32, visible: bool) {
        let visible = if visible { 1 } else { 0 };
        self.send_message(SCI_STYLESETVISIBLE, WPARAM(style as usize), LPARAM(visible));
    }

    fn style_get_visible(&self, style: i32) -> bool {
        let (_, res) = self.send_message(
            SCI_STYLEGETUNDERLINE,
            WPARAM(style as usize),
            LPARAM::default(),
        );
        res != 0
    }

    fn style_set_changeable(&self, style: i32, changeable: bool) {
        let changeable = if changeable { 1 } else { 0 };
        self.send_message(
            SCI_STYLESETCHANGEABLE,
            WPARAM(style as usize),
            LPARAM(changeable),
        );
    }

    fn style_get_changeable(&self, style: i32) -> bool {
        let (_, res) = self.send_message(
            SCI_STYLEGETCHANGEABLE,
            WPARAM(style as usize),
            LPARAM::default(),
        );
        res != 0
    }

    fn style_set_hotspot(&self, style: i32, hotspot: bool) {
        let hotspot = if hotspot { 1 } else { 0 };
        self.send_message(SCI_STYLESETHOTSPOT, WPARAM(style as usize), LPARAM(hotspot));
    }

    fn style_get_hotspot(&self, style: i32) -> bool {
        let (_, res) = self.send_message(
            SCI_STYLEGETHOTSPOT,
            WPARAM(style as usize),
            LPARAM::default(),
        );
        res != 0
    }

    fn set_sel_fore(&self, use_setting: bool, fore: i32) {
        let use_setting = if use_setting { 1 } else { 0 };
        self.send_message(SCI_SETSELFORE, WPARAM(use_setting), LPARAM(fore as isize));
    }

    fn set_sel_back(&self, use_setting: bool, back: i32) {
        let use_setting = if use_setting { 1 } else { 0 };
        self.send_message(SCI_SETSELBACK, WPARAM(use_setting), LPARAM(back as isize));
    }

    fn set_sel_alpha(&self, alpha: i32) {
        self.send_message(SCI_SETSELALPHA, WPARAM(alpha as usize), LPARAM::default());
    }

    fn get_sel_alpha(&self) -> i32 {
        let (_, res) = self.send_message(SCI_GETSELALPHA, WPARAM::default(), LPARAM::default());
        res as i32
    }

    fn set_sel_eol_filled(&self, filled: bool) {
        let filled = if filled { 1 } else { 0 };
        self.send_message(SCI_SETSELEOLFILLED, WPARAM(filled), LPARAM::default());
    }

    fn get_sel_eol_filled(&self) -> bool {
        let (_, res) = self.send_message(SCI_GETSELEOLFILLED, WPARAM::default(), LPARAM::default());
        res != 0
    }

    fn set_caret_fore(&self, fore: i32) {
        self.send_message(SCI_SETCARETFORE, WPARAM(fore as usize), LPARAM::default());
    }

    fn get_caret_fore(&self) -> i32 {
        let (_, res) = self.send_message(SCI_GETCARETFORE, WPARAM::default(), LPARAM::default());
        res as i32
    }

    fn set_caret_line_visible(&self, show: bool) {
        let show = if show { 1 } else { 0 };
        self.send_message(SCI_SETCARETLINEVISIBLE, WPARAM(show), LPARAM::default());
    }

    fn get_caret_line_visible(&self) -> bool {
        let (_, res) = self.send_message(
            SCI_GETCARETLINEVISIBLE,
            WPARAM::default(),
            LPARAM::default(),
        );
        res != 0
    }

    fn set_caret_line_back(&self, back: i32) {
        self.send_message(
            SCI_SETCARETLINEBACK,
            WPARAM(back as usize),
            LPARAM::default(),
        );
    }

    fn get_caret_line_back(&self) -> i32 {
        let (_, res) =
            self.send_message(SCI_GETCARETLINEBACK, WPARAM::default(), LPARAM::default());
        res as i32
    }

    fn set_caret_line_back_alpha(&self, alpha: i32) {
        self.send_message(
            SCI_SETCARETLINEBACKALPHA,
            WPARAM(alpha as usize),
            LPARAM::default(),
        );
    }

    fn get_caret_line_back_alpha(&self) -> i32 {
        let (_, res) = self.send_message(
            SCI_GETCARETLINEBACKALPHA,
            WPARAM::default(),
            LPARAM::default(),
        );
        res as i32
    }

    fn set_caret_line_frame(&self, width: i32) {
        self.send_message(
            SCI_SETCARETLINEFRAME,
            WPARAM(width as usize),
            LPARAM::default(),
        );
    }

    fn get_caret_line_frame(&self) -> i32 {
        let (_, res) =
            self.send_message(SCI_GETCARETLINEFRAME, WPARAM::default(), LPARAM::default());
        res as i32
    }

    fn set_caret_line_visible_always(&self, always_visible: bool) {
        let always_visible = if always_visible { 1 } else { 0 };
        self.send_message(
            SCI_SETCARETLINEVISIBLEALWAYS,
            WPARAM(always_visible),
            LPARAM::default(),
        );
    }

    fn get_caret_line_visible_always(&self) -> bool {
        let (_, res) = self.send_message(
            SCI_GETCARETLINEVISIBLEALWAYS,
            WPARAM::default(),
            LPARAM::default(),
        );
        res != 0
    }

    fn set_caret_period(&self, period_milliseconds: i32) {
        self.send_message(
            SCI_SETCARETPERIOD,
            WPARAM(period_milliseconds as usize),
            LPARAM::default(),
        );
    }

    fn get_caret_period(&self) -> i32 {
        let (_, res) = self.send_message(SCI_GETCARETPERIOD, WPARAM::default(), LPARAM::default());
        res as i32
    }

    fn set_caret_style(&self, caret_style: u32) {
        self.send_message(
            SCI_SETCARETSTYLE,
            WPARAM(caret_style as usize),
            LPARAM::default(),
        );
    }

    fn get_caret_style(&self) -> u32 {
        let (_, res) = self.send_message(SCI_GETCARETSTYLE, WPARAM::default(), LPARAM::default());
        res as u32
    }

    fn set_caret_width(&self, pixel_width: i32) {
        self.send_message(
            SCI_SETCARETWIDTH,
            WPARAM(pixel_width as usize),
            LPARAM::default(),
        );
    }

    fn get_caret_width(&self) -> i32 {
        let (_, res) = self.send_message(SCI_GETCARETWIDTH, WPARAM::default(), LPARAM::default());
        res as i32
    }

    fn set_caret_sticky(&self, use_caret_sticky_behaviour: CaretSticky) {
        self.send_message(
            SCI_SETCARETSTICKY,
            WPARAM(Into::<u32>::into(use_caret_sticky_behaviour) as usize),
            LPARAM::default(),
        );
    }

    fn get_caret_sticky(&self) -> CaretSticky {
        let (_, res) = self.send_message(SCI_GETCARETSTICKY, WPARAM::default(), LPARAM::default());
        CaretSticky::from(res as u32)
    }

    fn toggle_caret_sticky(&self) {
        self.send_message(SCI_TOGGLECARETSTICKY, WPARAM::default(), LPARAM::default());
    }

    fn set_hotspot_active_fore(&self, use_setting: bool, fore: i32) {
        let use_setting = if use_setting { 1 } else { 0 };
        self.send_message(
            SCI_SETHOTSPOTACTIVEFORE,
            WPARAM(use_setting),
            LPARAM(fore as isize),
        );
    }

    fn get_hotspot_active_fore(&self) -> i32 {
        let (_, res) = self.send_message(
            SCI_GETHOTSPOTACTIVEFORE,
            WPARAM::default(),
            LPARAM::default(),
        );
        res as i32
    }

    fn set_hotspot_active_back(&self, use_setting: bool, back: i32) {
        let use_setting = if use_setting { 1 } else { 0 };
        self.send_message(
            SCI_SETHOTSPOTACTIVEBACK,
            WPARAM(use_setting),
            LPARAM(back as isize),
        );
    }

    fn get_hotspot_active_back(&self) -> i32 {
        let (_, res) = self.send_message(
            SCI_GETHOTSPOTACTIVEBACK,
            WPARAM::default(),
            LPARAM::default(),
        );
        res as i32
    }

    fn set_hotspot_active_underline(&self, underline: bool) {
        let underline = if underline { 1 } else { 0 };
        self.send_message(
            SCI_SETHOTSPOTACTIVEUNDERLINE,
            WPARAM(underline),
            LPARAM::default(),
        );
    }

    fn get_hotspot_active_underline(&self) -> bool {
        let (_, res) = self.send_message(
            SCI_GETHOTSPOTACTIVEUNDERLINE,
            WPARAM::default(),
            LPARAM::default(),
        );
        res != 0
    }

    fn set_hotspot_single_line(&self, single_line: bool) {
        let single_line = if single_line { 1 } else { 0 };
        self.send_message(
            SCI_SETHOTSPOTSINGLELINE,
            WPARAM(single_line),
            LPARAM::default(),
        );
    }

    fn get_hotspot_single_line(&self) -> bool {
        let (_, res) = self.send_message(
            SCI_GETHOTSPOTSINGLELINE,
            WPARAM::default(),
            LPARAM::default(),
        );
        res != 0
    }

    fn set_representation(&self, encoded_character: String, representation: String) {
        let length = encoded_character.as_bytes().len();
        let mem = InProcessMemory::new(self.get_process_handle(), length + 1).unwrap();
        mem.write(encoded_character.as_ptr() as *const c_void, length);
        let length = representation.as_bytes().len();
        let mem2 = InProcessMemory::new(self.get_process_handle(), length + 1).unwrap();
        mem2.write(representation.as_ptr() as *const c_void, length);
        self.send_message(
            SCI_SETREPRESENTATION,
            WPARAM(mem.as_ptr() as usize),
            LPARAM(mem2.as_ptr() as isize),
        );
    }

    fn get_representation(&self, encoded_character: String) -> Option<String> {
        let length = encoded_character.as_bytes().len();
        let mem = InProcessMemory::new(self.get_process_handle(), length + 1).unwrap();
        mem.write(encoded_character.as_ptr() as *const c_void, length);
        let (_, length) = self.send_message(
            SCI_GETREPRESENTATION,
            WPARAM(mem.as_ptr() as usize),
            LPARAM::default(),
        );
        let mem2 = InProcessMemory::new(self.get_process_handle(), length + 1).unwrap();
        self.send_message(
            SCI_GETREPRESENTATION,
            WPARAM(mem.as_ptr() as usize),
            LPARAM(mem2.as_ptr() as isize),
        );
        mem2.read(|buf| (buf as *const u8).to_string())
    }

    fn clear_representation(&self, encoded_character: String) {
        let length = encoded_character.as_bytes().len();
        let mem = InProcessMemory::new(self.get_process_handle(), length + 1).unwrap();
        mem.write(encoded_character.as_ptr() as *const c_void, length);
        self.send_message(
            SCI_CLEARREPRESENTATION,
            WPARAM(mem.as_ptr() as usize),
            LPARAM::default(),
        );
    }

    fn set_control_char_symbol(&self, symbol: i32) {
        self.send_message(
            SCI_SETCONTROLCHARSYMBOL,
            WPARAM(symbol as usize),
            LPARAM::default(),
        );
    }

    fn get_control_char_symbol(&self) -> i32 {
        let (_, res) = self.send_message(
            SCI_GETCONTROLCHARSYMBOL,
            WPARAM::default(),
            LPARAM::default(),
        );
        res as i32
    }

    fn set_margins(&self, margins: i32) {
        self.send_message(SCI_SETMARGINS, WPARAM(margins as usize), LPARAM::default());
    }

    fn get_margins(&self) -> i32 {
        let (_, res) = self.send_message(SCI_GETMARGINS, WPARAM::default(), LPARAM::default());
        res as i32
    }

    fn set_margin_type_n(&self, margin: u32, margin_type: i32) {
        self.send_message(
            SCI_SETMARGINTYPEN,
            WPARAM(margin as usize),
            LPARAM(margin_type as isize),
        );
    }

    fn get_margin_type_n(&self, margin: u32) -> i32 {
        let (_, res) = self.send_message(
            SCI_GETMARGINTYPEN,
            WPARAM(margin as usize),
            LPARAM::default(),
        );
        res as i32
    }

    fn set_margin_width_n(&self, margin: u32, pixel_width: i32) {
        self.send_message(
            SCI_SETMARGINWIDTHN,
            WPARAM(margin as usize),
            LPARAM(pixel_width as isize),
        );
    }

    fn get_margin_width_n(&self, margin: u32) -> i32 {
        let (_, res) = self.send_message(
            SCI_GETMARGINWIDTHN,
            WPARAM(margin as usize),
            LPARAM::default(),
        );
        res as i32
    }

    fn set_margin_mask_n(&self, margin: u32, mask: i32) {
        self.send_message(
            SCI_SETMARGINMASKN,
            WPARAM(margin as usize),
            LPARAM(mask as isize),
        );
    }

    fn get_margin_mask_n(&self, margin: u32) -> i32 {
        let (_, res) = self.send_message(
            SCI_GETMARGINMASKN,
            WPARAM(margin as usize),
            LPARAM::default(),
        );
        res as i32
    }

    fn set_margin_sensitive_n(&self, margin: u32, sensitive: bool) {
        let sensitive = if sensitive { 1 } else { 0 };
        self.send_message(
            SCI_SETMARGINSENSITIVEN,
            WPARAM(margin as usize),
            LPARAM(sensitive),
        );
    }

    fn get_margin_sensitive_n(&self, margin: u32) -> bool {
        let (_, res) = self.send_message(
            SCI_GETMARGINSENSITIVEN,
            WPARAM(margin as usize),
            LPARAM::default(),
        );
        res != 0
    }

    fn set_margin_cursor_n(&self, margin: u32, cursor: u32) {
        self.send_message(
            SCI_SETMARGINCURSORN,
            WPARAM(margin as usize),
            LPARAM(cursor as isize),
        );
    }

    fn get_margin_cursor_n(&self, margin: u32) -> u32 {
        let (_, res) = self.send_message(
            SCI_GETMARGINCURSORN,
            WPARAM(margin as usize),
            LPARAM::default(),
        );
        res as u32
    }

    fn set_margin_back_n(&self, margin: u32, back: i32) {
        self.send_message(
            SCI_SETMARGINBACKN,
            WPARAM(margin as usize),
            LPARAM(back as isize),
        );
    }

    fn get_margin_back_n(&self, margin: u32) -> i32 {
        let (_, res) = self.send_message(
            SCI_GETMARGINBACKN,
            WPARAM(margin as usize),
            LPARAM::default(),
        );
        res as i32
    }

    fn set_margin_left(&self, pixel_width: i32) {
        self.send_message(
            SCI_SETMARGINLEFT,
            WPARAM::default(),
            LPARAM(pixel_width as isize),
        );
    }

    fn get_margin_left(&self) -> i32 {
        let (_, res) = self.send_message(SCI_GETMARGINLEFT, WPARAM::default(), LPARAM::default());
        res as i32
    }

    fn set_margin_right(&self, pixel_width: i32) {
        self.send_message(
            SCI_SETMARGINRIGHT,
            WPARAM::default(),
            LPARAM(pixel_width as isize),
        );
    }

    fn get_margin_right(&self) -> i32 {
        let (_, res) = self.send_message(SCI_GETMARGINRIGHT, WPARAM::default(), LPARAM::default());
        res as i32
    }

    fn set_fold_margin_colour(&self, use_setting: bool, back: i32) {
        let use_setting = if use_setting { 1 } else { 0 };
        self.send_message(
            SCI_SETFOLDMARGINCOLOUR,
            WPARAM(use_setting),
            LPARAM(back as isize),
        );
    }

    fn set_fold_margin_hi_colour(&self, use_setting: bool, fore: i32) {
        let use_setting = if use_setting { 1 } else { 0 };
        self.send_message(
            SCI_SETFOLDMARGINHICOLOUR,
            WPARAM(use_setting),
            LPARAM(fore as isize),
        );
    }

    fn margin_set_text(&self, line: usize, text: String) {
        let length = text.as_bytes().len();
        let mem = InProcessMemory::new(self.get_process_handle(), length + 1).unwrap();
        mem.write(text.as_ptr() as *const c_void, length);
        self.send_message(
            SCI_MARGINSETTEXT,
            WPARAM(line),
            LPARAM(mem.as_ptr() as isize),
        );
    }

    fn margin_get_text(&self, line: usize) -> Option<String> {
        let (_, length) = self.send_message(SCI_MARGINGETTEXT, WPARAM(line), LPARAM::default());
        let mem = InProcessMemory::new(self.get_process_handle(), length + 1).unwrap();
        self.send_message(
            SCI_MARGINGETTEXT,
            WPARAM(line),
            LPARAM(mem.as_ptr() as isize),
        );
        mem.read(|buf| (buf as *const u8).to_string())
    }

    fn margin_set_style(&self, line: usize, style: u32) {
        self.send_message(SCI_MARGINSETSTYLE, WPARAM(line), LPARAM(style as isize));
    }

    fn margin_get_style(&self, line: usize) -> u32 {
        let (_, res) = self.send_message(SCI_MARGINGETSTYLE, WPARAM(line), LPARAM::default());
        res as u32
    }

    fn margin_set_styles(&self, line: usize, styles: &[u8]) {
        let length = styles.len();
        let mem = InProcessMemory::new(self.get_process_handle(), length + 1).unwrap();
        mem.write(styles.as_ptr() as *const c_void, length);
        self.send_message(
            SCI_MARGINSETSTYLES,
            WPARAM(line),
            LPARAM(mem.as_ptr() as isize),
        );
    }

    fn margin_get_styles(&self, line: usize) -> Vec<u8> {
        let (_, length) = self.send_message(SCI_MARGINGETSTYLES, WPARAM(line), LPARAM::default());
        let mem = InProcessMemory::new(self.get_process_handle(), length + 1).unwrap();
        let (_, length) = self.send_message(
            SCI_MARGINGETSTYLES,
            WPARAM(line),
            LPARAM(mem.as_ptr() as isize),
        );
        mem.read(|buf| unsafe {
            let mut v = Vec::with_capacity(length);
            v.extend_from_slice(&*slice_from_raw_parts(buf as *const u8, length));
            v
        })
        .unwrap_or(Vec::new())
    }

    fn margin_text_clear_all(&self) {
        self.send_message(SCI_MARGINTEXTCLEARALL, WPARAM::default(), LPARAM::default());
    }

    fn margin_set_style_offset(&self, style: i32) {
        self.send_message(
            SCI_MARGINSETSTYLEOFFSET,
            WPARAM(style as usize),
            LPARAM::default(),
        );
    }

    fn margin_get_style_offset(&self) -> i32 {
        let (_, res) = self.send_message(
            SCI_MARGINGETSTYLEOFFSET,
            WPARAM::default(),
            LPARAM::default(),
        );
        res as i32
    }

    fn set_margin_options(&self, margin_options: MarginOptions) {
        self.send_message(
            SCI_SETMARGINOPTIONS,
            WPARAM(Into::<u32>::into(margin_options) as usize),
            LPARAM::default(),
        );
    }

    fn get_margin_options(&self) -> MarginOptions {
        let (_, res) =
            self.send_message(SCI_GETMARGINOPTIONS, WPARAM::default(), LPARAM::default());
        MarginOptions::from(res as u32)
    }

    fn annotation_set_text(&self, line: usize, text: Option<String>) {
        let Some(text) = text else {
            self.send_message(SCI_ANNOTATIONSETTEXT, WPARAM(line), LPARAM::default());
            return;
        };
        let length = text.as_bytes().len();
        let mem = InProcessMemory::new(self.get_process_handle(), length + 1).unwrap();
        mem.write(text.as_ptr() as *const c_void, length);
        self.send_message(
            SCI_ANNOTATIONSETTEXT,
            WPARAM(line),
            LPARAM(mem.as_ptr() as isize),
        );
    }

    fn annotation_get_text(&self, line: usize) -> Option<String> {
        let (_, length) = self.send_message(SCI_ANNOTATIONGETTEXT, WPARAM(line), LPARAM::default());
        let mem = InProcessMemory::new(self.get_process_handle(), length + 1).unwrap();
        self.send_message(
            SCI_ANNOTATIONGETTEXT,
            WPARAM(line),
            LPARAM(mem.as_ptr() as isize),
        );
        mem.read(|buf| (buf as *const u8).to_string())
    }

    fn annotation_set_style(&self, line: usize, style: u32) {
        self.send_message(SCI_ANNOTATIONSETSTYLE, WPARAM(line), LPARAM(style as isize));
    }

    fn annotation_get_style(&self, line: usize) -> u32 {
        let (_, res) = self.send_message(SCI_ANNOTATIONGETSTYLE, WPARAM(line), LPARAM::default());
        res as u32
    }

    fn annotation_set_styles(&self, line: usize, styles: &[u8]) {
        let length = styles.len();
        let mem = InProcessMemory::new(self.get_process_handle(), length + 1).unwrap();
        mem.write(styles.as_ptr() as *const c_void, length);
        self.send_message(
            SCI_ANNOTATIONSETSTYLES,
            WPARAM(line),
            LPARAM(mem.as_ptr() as isize),
        );
    }

    fn annotation_get_styles(&self, line: usize) -> Vec<u8> {
        let (_, length) =
            self.send_message(SCI_ANNOTATIONGETSTYLES, WPARAM(line), LPARAM::default());
        let mem = InProcessMemory::new(self.get_process_handle(), length + 1).unwrap();
        self.send_message(
            SCI_ANNOTATIONGETSTYLES,
            WPARAM(line),
            LPARAM(mem.as_ptr() as isize),
        );
        mem.read(|buf| unsafe {
            let mut v = Vec::with_capacity(length);
            v.extend_from_slice(&*slice_from_raw_parts(buf as *const u8, length));
            v
        })
        .unwrap_or(Vec::new())
    }

    fn annotation_get_lines(&self, line: usize) -> i32 {
        let (_, res) = self.send_message(SCI_ANNOTATIONGETLINES, WPARAM(line), LPARAM::default());
        res as i32
    }

    fn annotation_clear_all(&self) {
        self.send_message(SCI_ANNOTATIONCLEARALL, WPARAM::default(), LPARAM::default());
    }

    fn annotation_set_visible(&self, visible: Annotation) {
        self.send_message(
            SCI_ANNOTATIONSETVISIBLE,
            WPARAM(Into::<u32>::into(visible) as usize),
            LPARAM::default(),
        );
    }

    fn annotation_get_visible(&self) -> Annotation {
        let (_, res) = self.send_message(
            SCI_ANNOTATIONGETVISIBLE,
            WPARAM::default(),
            LPARAM::default(),
        );
        Annotation::from(res as u32)
    }

    fn annotation_set_style_offset(&self, style: i32) {
        self.send_message(
            SCI_ANNOTATIONSETSTYLEOFFSET,
            WPARAM(style as usize),
            LPARAM::default(),
        );
    }

    fn annotation_get_style_offset(&self) -> i32 {
        let (_, res) = self.send_message(
            SCI_ANNOTATIONGETSTYLEOFFSET,
            WPARAM::default(),
            LPARAM::default(),
        );
        res as i32
    }

    fn set_buffered_draw(&self, buffered: bool) {
        let buffered = if buffered { 1 } else { 0 };
        self.send_message(SCI_SETBUFFEREDDRAW, WPARAM(buffered), LPARAM::default());
    }

    fn get_buffered_draw(&self) -> bool {
        let (_, res) = self.send_message(SCI_GETBUFFEREDDRAW, WPARAM::default(), LPARAM::default());
        res != 0
    }

    fn set_phases_draw(&self, phases: Phases) {
        self.send_message(
            SCI_SETPHASESDRAW,
            WPARAM(Into::<u32>::into(phases) as usize),
            LPARAM::default(),
        );
    }

    fn get_phases_draw(&self) -> Phases {
        let (_, res) = self.send_message(SCI_GETPHASESDRAW, WPARAM::default(), LPARAM::default());
        Phases::from(res as u32)
    }

    fn set_technology(&self, technology: Technology) {
        self.send_message(
            SCI_SETTECHNOLOGY,
            WPARAM(Into::<u32>::into(technology) as usize),
            LPARAM::default(),
        );
    }

    fn get_technology(&self) -> Technology {
        let (_, res) = self.send_message(SCI_GETTECHNOLOGY, WPARAM::default(), LPARAM::default());
        Technology::from(res as u32)
    }

    fn set_font_quality(&self, font_quality: u32) {
        self.send_message(
            SCI_SETFONTQUALITY,
            WPARAM(font_quality as usize),
            LPARAM::default(),
        );
    }

    fn get_font_quality(&self) -> u32 {
        let (_, res) = self.send_message(SCI_GETFONTQUALITY, WPARAM::default(), LPARAM::default());
        res as u32
    }

    fn set_code_page(&self, code_page: u32) {
        self.send_message(
            SCI_SETCODEPAGE,
            WPARAM(code_page as usize),
            LPARAM::default(),
        );
    }

    fn get_code_page(&self) -> u32 {
        let (_, res) = self.send_message(SCI_GETCODEPAGE, WPARAM::default(), LPARAM::default());
        res as u32
    }

    fn set_ime_interaction(&self, ime_interaction: Ime) {
        self.send_message(
            SCI_SETIMEINTERACTION,
            WPARAM(Into::<u32>::into(ime_interaction) as usize),
            LPARAM::default(),
        );
    }

    fn get_ime_interaction(&self) -> Ime {
        let (_, res) =
            self.send_message(SCI_GETIMEINTERACTION, WPARAM::default(), LPARAM::default());
        Ime::from(res as u32)
    }

    fn set_bidirectional(&self, bidirectional: Bidirectional) {
        self.send_message(
            SCI_SETBIDIRECTIONAL,
            WPARAM(Into::<u32>::into(bidirectional) as usize),
            LPARAM::default(),
        );
    }

    fn get_bidirectional(&self) -> Bidirectional {
        let (_, res) =
            self.send_message(SCI_GETBIDIRECTIONAL, WPARAM::default(), LPARAM::default());
        Bidirectional::from(res as u32)
    }

    fn grab_focus(&self) {
        self.send_message(SCI_GRABFOCUS, WPARAM::default(), LPARAM::default());
    }

    fn set_focus(&self, focus: bool) {
        let focus = if focus { 1 } else { 0 };
        self.send_message(SCI_SETFOCUS, WPARAM(focus), LPARAM::default());
    }

    fn get_focus(&self) -> bool {
        let (_, res) = self.send_message(SCI_GETFOCUS, WPARAM::default(), LPARAM::default());
        res != 0
    }

    fn brace_highlight(&self, pos_a: usize, pos_b: usize) {
        self.send_message(SCI_BRACEHIGHLIGHT, WPARAM(pos_a), LPARAM(pos_b as isize));
    }

    fn brace_badlight(&self, pos: usize) {
        self.send_message(SCI_BRACEBADLIGHT, WPARAM(pos), LPARAM::default());
    }

    fn brace_highlight_indicator(&self, use_setting: bool, indicator: i32) {
        let use_setting = if use_setting { 1 } else { 0 };
        self.send_message(
            SCI_BRACEHIGHLIGHTINDICATOR,
            WPARAM(use_setting),
            LPARAM(indicator as isize),
        );
    }

    fn brace_badlight_indicator(&self, use_setting: bool, indicator: i32) {
        let use_setting = if use_setting { 1 } else { 0 };
        self.send_message(
            SCI_BRACEBADLIGHTINDICATOR,
            WPARAM(use_setting),
            LPARAM(indicator as isize),
        );
    }

    fn brace_match(&self, pos: usize, max_re_style: i32) -> usize {
        let (_, res) =
            self.send_message(SCI_BRACEMATCH, WPARAM(pos), LPARAM(max_re_style as isize));
        res
    }

    fn set_tab_width(&self, tab_width: i32) {
        self.send_message(
            SCI_SETTABWIDTH,
            WPARAM(tab_width as usize),
            LPARAM::default(),
        );
    }

    fn get_tab_width(&self) -> i32 {
        let (_, res) = self.send_message(SCI_GETTABWIDTH, WPARAM::default(), LPARAM::default());
        res as i32
    }

    fn clear_tabstops(&self, line: usize) {
        self.send_message(SCI_CLEARTABSTOPS, WPARAM(line), LPARAM::default());
    }

    fn add_tabstop(&self, line: usize, x: i32) {
        self.send_message(SCI_ADDTABSTOP, WPARAM(line), LPARAM(x as isize));
    }

    fn get_next_tabstop(&self, line: usize, x: i32) -> i32 {
        let (_, res) = self.send_message(SCI_GETNEXTTABSTOP, WPARAM(line), LPARAM(x as isize));
        res as i32
    }

    fn set_use_tabs(&self, use_tabs: bool) {
        let use_tabs = if use_tabs { 1 } else { 0 };
        self.send_message(SCI_SETUSETABS, WPARAM(use_tabs), LPARAM::default());
    }

    fn get_use_tabs(&self) -> bool {
        let (_, res) = self.send_message(SCI_GETUSETABS, WPARAM::default(), LPARAM::default());
        res != 0
    }

    fn set_indent(&self, indent_size: i32) {
        self.send_message(
            SCI_SETINDENT,
            WPARAM(indent_size as usize),
            LPARAM::default(),
        );
    }

    fn get_indent(&self) -> i32 {
        let (_, res) = self.send_message(SCI_GETINDENT, WPARAM::default(), LPARAM::default());
        res as i32
    }

    fn set_tab_indents(&self, tab_indents: bool) {
        let tab_indents = if tab_indents { 1 } else { 0 };
        self.send_message(SCI_SETTABINDENTS, WPARAM(tab_indents), LPARAM::default());
    }

    fn get_tab_indents(&self) -> bool {
        let (_, res) = self.send_message(SCI_GETTABINDENTS, WPARAM::default(), LPARAM::default());
        res != 0
    }

    fn set_backspace_un_indents(&self, bs_un_indents: bool) {
        let bs_un_indents = if bs_un_indents { 1 } else { 0 };
        self.send_message(
            SCI_SETBACKSPACEUNINDENTS,
            WPARAM(bs_un_indents),
            LPARAM::default(),
        );
    }

    fn get_backspace_un_indents(&self) -> bool {
        let (_, res) = self.send_message(
            SCI_GETBACKSPACEUNINDENTS,
            WPARAM::default(),
            LPARAM::default(),
        );
        res != 0
    }

    fn set_line_indentation(&self, line: usize, indentation: i32) {
        self.send_message(
            SCI_SETLINEINDENTATION,
            WPARAM(line),
            LPARAM(indentation as isize),
        );
    }

    fn get_line_indentation(&self, line: usize) -> i32 {
        let (_, res) = self.send_message(SCI_GETLINEINDENTATION, WPARAM(line), LPARAM::default());
        res as i32
    }

    fn get_line_indent_position(&self, line: usize) -> usize {
        let (_, res) =
            self.send_message(SCI_GETLINEINDENTPOSITION, WPARAM(line), LPARAM::default());
        res
    }

    fn set_indentation_guides(&self, indent_view: IndentView) {
        self.send_message(
            SCI_SETINDENTATIONGUIDES,
            WPARAM(Into::<u32>::into(indent_view) as usize),
            LPARAM::default(),
        );
    }

    fn get_indentation_guides(&self) -> IndentView {
        let (_, res) = self.send_message(
            SCI_GETINDENTATIONGUIDES,
            WPARAM::default(),
            LPARAM::default(),
        );
        IndentView::from(res as u32)
    }

    fn set_highlight_guide(&self, column: usize) {
        self.send_message(SCI_SETHIGHLIGHTGUIDE, WPARAM(column), LPARAM::default());
    }

    fn get_highlight_guide(&self) -> usize {
        let (_, res) =
            self.send_message(SCI_GETHIGHLIGHTGUIDE, WPARAM::default(), LPARAM::default());
        res
    }

    fn marker_define(&self, marker_number: u32, marker_symbol: Mark) {
        self.send_message(
            SCI_MARKERDEFINE,
            WPARAM(marker_number as usize),
            LPARAM(Into::<u32>::into(marker_symbol) as isize),
        );
    }

    fn marker_define_pixmap(&self, marker_number: u32, pixmap: &[&str]) {
        self.send_message(
            SCI_MARKERDEFINEPIXMAP,
            WPARAM(marker_number as usize),
            LPARAM(pixmap.as_ptr() as isize),
        );
    }

    fn rgba_image_set_width(&self, width: i32) {
        self.send_message(
            SCI_RGBAIMAGESETWIDTH,
            WPARAM(width as usize),
            LPARAM::default(),
        );
    }

    fn rgba_image_set_height(&self, height: i32) {
        self.send_message(
            SCI_RGBAIMAGESETHEIGHT,
            WPARAM(height as usize),
            LPARAM::default(),
        );
    }

    fn rgba_image_set_scale(&self, scale_percent: i32) {
        self.send_message(
            SCI_RGBAIMAGESETSCALE,
            WPARAM(scale_percent as usize),
            LPARAM::default(),
        );
    }

    fn marker_define_rgba_image(&self, marker_number: u32, pixels: &[u8]) {
        let length = pixels.len();
        let mem = InProcessMemory::new(self.get_process_handle(), length + 1).unwrap();
        mem.write(pixels.as_ptr() as *const c_void, length);
        self.send_message(
            SCI_MARKERDEFINERGBAIMAGE,
            WPARAM(marker_number as usize),
            LPARAM(mem.as_ptr() as isize),
        );
    }

    fn marker_symbol_defined(&self, marker_number: u32) -> Mark {
        let (_, res) = self.send_message(
            SCI_MARKERSYMBOLDEFINED,
            WPARAM(marker_number as usize),
            LPARAM::default(),
        );
        Mark::from(res as u32)
    }

    fn marker_set_fore(&self, marker_number: u32, fore: i32) {
        self.send_message(
            SCI_MARKERSETFORE,
            WPARAM(marker_number as usize),
            LPARAM(fore as isize),
        );
    }

    fn marker_set_back(&self, marker_number: u32, back: i32) {
        self.send_message(
            SCI_MARKERSETBACK,
            WPARAM(marker_number as usize),
            LPARAM(back as isize),
        );
    }

    fn marker_set_back_selected(&self, marker_number: u32, back: i32) {
        self.send_message(
            SCI_MARKERSETBACKSELECTED,
            WPARAM(marker_number as usize),
            LPARAM(back as isize),
        );
    }

    fn marker_enable_highlight(&self, enabled: bool) {
        let enabled = if enabled { 1 } else { 0 };
        self.send_message(
            SCI_MARKERENABLEHIGHLIGHT,
            WPARAM(enabled),
            LPARAM::default(),
        );
    }

    fn marker_set_alpha(&self, marker_number: u32, alpha: i32) {
        self.send_message(
            SCI_MARKERSETALPHA,
            WPARAM(marker_number as usize),
            LPARAM(alpha as isize),
        );
    }

    fn marker_add(&self, line: usize, marker_number: u32) -> i32 {
        let (_, res) =
            self.send_message(SCI_MARKERADD, WPARAM(line), LPARAM(marker_number as isize));
        res as i32
    }

    fn marker_add_set(&self, line: usize, marker_set: i32) {
        self.send_message(SCI_MARKERADDSET, WPARAM(line), LPARAM(marker_set as isize));
    }

    fn marker_delete(&self, line: usize, marker_number: u32) {
        self.send_message(
            SCI_MARKERDELETE,
            WPARAM(line),
            LPARAM(marker_number as isize),
        );
    }

    fn marker_get(&self, line: usize) -> i32 {
        let (_, res) = self.send_message(SCI_MARKERGET, WPARAM(line), LPARAM::default());
        res as i32
    }

    fn marker_delete_all(&self, marker_number: u32) {
        self.send_message(
            SCI_MARKERDELETEALL,
            WPARAM(marker_number as usize),
            LPARAM::default(),
        );
    }

    fn marker_next(&self, line_start: usize, marker_mask: i32) -> usize {
        let (_, res) = self.send_message(
            SCI_MARKERNEXT,
            WPARAM(line_start),
            LPARAM(marker_mask as isize),
        );
        res
    }

    fn marker_previous(&self, line_start: usize, marker_mask: i32) -> usize {
        let (_, res) = self.send_message(
            SCI_MARKERPREVIOUS,
            WPARAM(line_start),
            LPARAM(marker_mask as isize),
        );
        res
    }

    fn marker_line_from_handle(&self, marker_handle: i32) -> usize {
        let (_, res) = self.send_message(
            SCI_MARKERLINEFROMHANDLE,
            WPARAM(marker_handle as usize),
            LPARAM::default(),
        );
        res
    }

    fn marker_delete_handle(&self, marker_handle: i32) {
        self.send_message(
            SCI_MARKERDELETEHANDLE,
            WPARAM(marker_handle as usize),
            LPARAM::default(),
        );
    }

    fn indic_set_style(&self, indicator: i32, indicator_style: Indicator) {
        self.send_message(
            SCI_INDICSETSTYLE,
            WPARAM(indicator as usize),
            LPARAM(Into::<u32>::into(indicator_style) as isize),
        );
    }

    fn indic_get_style(&self, indicator: i32) -> Indicator {
        let (_, res) = self.send_message(
            SCI_INDICGETSTYLE,
            WPARAM(indicator as usize),
            LPARAM::default(),
        );
        Indicator::from(res as u32)
    }

    fn indic_set_fore(&self, indicator: i32, fore: i32) {
        self.send_message(
            SCI_INDICSETFORE,
            WPARAM(indicator as usize),
            LPARAM(fore as isize),
        );
    }

    fn indic_get_fore(&self, indicator: i32) -> i32 {
        let (_, res) = self.send_message(
            SCI_INDICGETFORE,
            WPARAM(indicator as usize),
            LPARAM::default(),
        );
        res as i32
    }

    fn indic_set_alpha(&self, indicator: i32, alpha: i32) {
        self.send_message(
            SCI_INDICSETALPHA,
            WPARAM(indicator as usize),
            LPARAM(alpha as isize),
        );
    }

    fn indic_get_alpha(&self, indicator: i32) -> i32 {
        let (_, res) = self.send_message(
            SCI_INDICGETALPHA,
            WPARAM(indicator as usize),
            LPARAM::default(),
        );
        res as i32
    }

    fn indic_set_outline_alpha(&self, indicator: i32, alpha: i32) {
        self.send_message(
            SCI_INDICSETOUTLINEALPHA,
            WPARAM(indicator as usize),
            LPARAM(alpha as isize),
        );
    }

    fn indic_get_outline_alpha(&self, indicator: i32) -> i32 {
        let (_, res) = self.send_message(
            SCI_INDICGETOUTLINEALPHA,
            WPARAM(indicator as usize),
            LPARAM::default(),
        );
        res as i32
    }

    fn indic_set_under(&self, indicator: i32, under: bool) {
        let under = if under { 1 } else { 0 };
        self.send_message(SCI_INDICSETUNDER, WPARAM(indicator as usize), LPARAM(under));
    }

    fn indic_get_under(&self, indicator: i32) -> bool {
        let (_, res) = self.send_message(
            SCI_INDICGETUNDER,
            WPARAM(indicator as usize),
            LPARAM::default(),
        );
        res != 0
    }

    fn indic_set_hover_style(&self, indicator: i32, indicator_style: Indicator) {
        self.send_message(
            SCI_INDICSETHOVERSTYLE,
            WPARAM(indicator as usize),
            LPARAM(Into::<u32>::into(indicator_style) as isize),
        );
    }

    fn indic_get_hover_style(&self, indicator: i32) -> Indicator {
        let (_, res) = self.send_message(
            SCI_INDICGETHOVERSTYLE,
            WPARAM(indicator as usize),
            LPARAM::default(),
        );
        Indicator::from(res as u32)
    }

    fn indic_set_hover_fore(&self, indicator: i32, fore: i32) {
        self.send_message(
            SCI_INDICSETHOVERFORE,
            WPARAM(indicator as usize),
            LPARAM(fore as isize),
        );
    }

    fn indic_get_hover_fore(&self, indicator: i32) -> i32 {
        let (_, res) = self.send_message(
            SCI_INDICGETHOVERFORE,
            WPARAM(indicator as usize),
            LPARAM::default(),
        );
        res as i32
    }

    fn indic_set_flags(&self, indicator: i32, flags: u32) {
        self.send_message(
            SCI_INDICSETFLAGS,
            WPARAM(indicator as usize),
            LPARAM(flags as isize),
        );
    }

    fn indic_get_flags(&self, indicator: i32) -> u32 {
        let (_, res) = self.send_message(
            SCI_INDICGETFLAGS,
            WPARAM(indicator as usize),
            LPARAM::default(),
        );
        res as u32
    }

    fn set_indicator_current(&self, indicator: i32) {
        self.send_message(
            SCI_SETINDICATORCURRENT,
            WPARAM(indicator as usize),
            LPARAM::default(),
        );
    }

    fn get_indicator_current(&self) -> i32 {
        let (_, res) = self.send_message(
            SCI_GETINDICATORCURRENT,
            WPARAM::default(),
            LPARAM::default(),
        );
        res as i32
    }

    fn set_indicator_value(&self, value: i32) {
        self.send_message(
            SCI_SETINDICATORVALUE,
            WPARAM(value as usize),
            LPARAM::default(),
        );
    }

    fn get_indicator_value(&self) -> i32 {
        let (_, res) =
            self.send_message(SCI_GETINDICATORVALUE, WPARAM::default(), LPARAM::default());
        res as i32
    }

    fn indicator_fill_range(&self, start: usize, length_fill: usize) {
        self.send_message(
            SCI_INDICATORFILLRANGE,
            WPARAM(start),
            LPARAM(length_fill as isize),
        );
    }

    fn indicator_clear_range(&self, start: usize, length_clear: usize) {
        self.send_message(
            SCI_INDICATORCLEARRANGE,
            WPARAM(start),
            LPARAM(length_clear as isize),
        );
    }

    fn indicator_all_on_for(&self, pos: usize) -> i32 {
        let (_, res) = self.send_message(SCI_INDICATORALLONFOR, WPARAM(pos), LPARAM::default());
        res as i32
    }

    fn indicator_value_at(&self, indicator: i32, pos: usize) -> i32 {
        let (_, res) = self.send_message(
            SCI_INDICATORVALUEAT,
            WPARAM(indicator as usize),
            LPARAM(pos as isize),
        );
        res as i32
    }

    fn indicator_start(&self, indicator: i32, pos: usize) -> usize {
        let (_, res) = self.send_message(
            SCI_INDICATORSTART,
            WPARAM(indicator as usize),
            LPARAM(pos as isize),
        );
        res
    }

    fn indicator_end(&self, indicator: i32, pos: usize) -> usize {
        let (_, res) = self.send_message(
            SCI_INDICATOREND,
            WPARAM(indicator as usize),
            LPARAM(pos as isize),
        );
        res
    }

    fn find_indicator_show(&self, start: usize, end: usize) {
        self.send_message(SCI_FINDINDICATORSHOW, WPARAM(start), LPARAM(end as isize));
    }

    fn find_indicator_flash(&self, start: usize, end: usize) {
        self.send_message(SCI_FINDINDICATORFLASH, WPARAM(start), LPARAM(end as isize));
    }

    fn find_indicator_hide(&self) {
        self.send_message(SCI_FINDINDICATORHIDE, WPARAM::default(), LPARAM::default());
    }

    fn autoc_show(&self, item_list: String) {
        let length = item_list.as_bytes().len();
        let mem = InProcessMemory::new(self.get_process_handle(), length + 1).unwrap();
        mem.write(item_list.as_ptr() as *const c_void, length);
        self.send_message(SCI_AUTOCSHOW, WPARAM(length), LPARAM(mem.as_ptr() as isize));
    }

    fn autoc_cancel(&self) {
        self.send_message(SCI_AUTOCCANCEL, WPARAM::default(), LPARAM::default());
    }

    fn autoc_active(&self) -> bool {
        let (_, res) = self.send_message(SCI_AUTOCACTIVE, WPARAM::default(), LPARAM::default());
        res != 0
    }

    fn autoc_pos_start(&self) -> usize {
        let (_, res) = self.send_message(SCI_AUTOCPOSSTART, WPARAM::default(), LPARAM::default());
        res
    }

    fn autoc_complete(&self) {
        self.send_message(SCI_AUTOCCOMPLETE, WPARAM::default(), LPARAM::default());
    }

    fn autoc_stops(&self, character_set: String) {
        let length = character_set.as_bytes().len();
        let mem = InProcessMemory::new(self.get_process_handle(), length + 1).unwrap();
        mem.write(character_set.as_ptr() as *const c_void, length);
        self.send_message(
            SCI_AUTOCSTOPS,
            WPARAM(length),
            LPARAM(mem.as_ptr() as isize),
        );
    }

    fn autoc_set_separator(&self, separator_character: u32) {
        self.send_message(
            SCI_AUTOCSETSEPARATOR,
            WPARAM(separator_character as usize),
            LPARAM::default(),
        );
    }

    fn autoc_get_separator(&self) -> u32 {
        let (_, res) =
            self.send_message(SCI_AUTOCGETSEPARATOR, WPARAM::default(), LPARAM::default());
        res as u32
    }

    fn autoc_select(&self, select: String) {
        let length = select.as_bytes().len();
        let mem = InProcessMemory::new(self.get_process_handle(), length + 1).unwrap();
        mem.write(select.as_ptr() as *const c_void, length);
        self.send_message(
            SCI_AUTOCSELECT,
            WPARAM(length),
            LPARAM(mem.as_ptr() as isize),
        );
    }

    fn autoc_get_current(&self) -> i32 {
        let (_, res) = self.send_message(SCI_AUTOCGETCURRENT, WPARAM::default(), LPARAM::default());
        res as i32
    }

    fn autoc_get_current_text(&self) -> Option<String> {
        let (_, length) = self.send_message(
            SCI_AUTOCGETCURRENTTEXT,
            WPARAM::default(),
            LPARAM::default(),
        );
        let mem = InProcessMemory::new(self.get_process_handle(), length + 1).unwrap();
        self.send_message(
            SCI_AUTOCGETCURRENTTEXT,
            WPARAM(length),
            LPARAM(mem.as_ptr() as isize),
        );
        mem.read(|buf| (buf as *const u8).to_string())
    }

    fn autoc_set_cancel_at_start(&self, cancel: bool) {
        let cancel = if cancel { 1 } else { 0 };
        self.send_message(SCI_AUTOCSETCANCELATSTART, WPARAM(cancel), LPARAM::default());
    }

    fn autoc_get_cancel_at_start(&self) -> bool {
        let (_, res) = self.send_message(
            SCI_AUTOCGETCANCELATSTART,
            WPARAM::default(),
            LPARAM::default(),
        );
        res != 0
    }

    fn autoc_set_fillups(&self, character_set: String) {
        let length = character_set.as_bytes().len();
        let mem = InProcessMemory::new(self.get_process_handle(), length + 1).unwrap();
        mem.write(character_set.as_ptr() as *const c_void, length);
        self.send_message(
            SCI_AUTOCSETFILLUPS,
            WPARAM(length),
            LPARAM(mem.as_ptr() as isize),
        );
    }

    fn autoc_set_choose_single(&self, choose_single: bool) {
        let choose_single = if choose_single { 1 } else { 0 };
        self.send_message(
            SCI_AUTOCSETCHOOSESINGLE,
            WPARAM(choose_single),
            LPARAM::default(),
        );
    }

    fn autoc_get_choose_single(&self) -> bool {
        let (_, res) = self.send_message(
            SCI_AUTOCGETCHOOSESINGLE,
            WPARAM::default(),
            LPARAM::default(),
        );
        res != 0
    }

    fn autoc_set_ignore_case(&self, ignore_case: bool) {
        let ignore_case = if ignore_case { 1 } else { 0 };
        self.send_message(
            SCI_AUTOCSETIGNORECASE,
            WPARAM(ignore_case),
            LPARAM::default(),
        );
    }

    fn autoc_get_ignore_case(&self) -> bool {
        let (_, res) =
            self.send_message(SCI_AUTOCGETIGNORECASE, WPARAM::default(), LPARAM::default());
        res != 0
    }

    fn autoc_set_case_in_sensitive_behaviour(&self, behaviour: u32) {
        self.send_message(
            SCI_AUTOCSETCASEINSENSITIVEBEHAVIOUR,
            WPARAM(behaviour as usize),
            LPARAM::default(),
        );
    }

    fn autoc_get_case_in_sensitive_behaviour(&self) -> u32 {
        let (_, res) = self.send_message(
            SCI_AUTOCGETCASEINSENSITIVEBEHAVIOUR,
            WPARAM::default(),
            LPARAM::default(),
        );
        res as u32
    }

    fn autoc_set_multi(&self, multi: MultiAutoc) {
        self.send_message(
            SCI_AUTOCSETMULTI,
            WPARAM(Into::<u32>::into(multi) as usize),
            LPARAM::default(),
        );
    }

    fn autoc_get_multi(&self) -> MultiAutoc {
        let (_, res) = self.send_message(SCI_AUTOCGETMULTI, WPARAM::default(), LPARAM::default());
        MultiAutoc::from(res as u32)
    }

    fn autoc_set_order(&self, order: Order) {
        self.send_message(
            SCI_AUTOCSETORDER,
            WPARAM(Into::<u32>::into(order) as usize),
            LPARAM::default(),
        );
    }

    fn autoc_get_order(&self) -> Order {
        let (_, res) = self.send_message(SCI_AUTOCGETORDER, WPARAM::default(), LPARAM::default());
        Order::from(res as u32)
    }

    fn autoc_set_auto_hide(&self, auto_hide: bool) {
        let auto_hide = if auto_hide { 1 } else { 0 };
        self.send_message(SCI_AUTOCSETAUTOHIDE, WPARAM(auto_hide), LPARAM::default());
    }

    fn autoc_get_auto_hide(&self) -> bool {
        let (_, res) =
            self.send_message(SCI_AUTOCGETAUTOHIDE, WPARAM::default(), LPARAM::default());
        res != 0
    }

    fn autoc_set_drop_rest_of_word(&self, drop_rest_of_word: bool) {
        let drop_rest_of_word = if drop_rest_of_word { 1 } else { 0 };
        self.send_message(
            SCI_AUTOCSETDROPRESTOFWORD,
            WPARAM(drop_rest_of_word),
            LPARAM::default(),
        );
    }

    fn autoc_get_drop_rest_of_word(&self) -> bool {
        let (_, res) = self.send_message(
            SCI_AUTOCGETDROPRESTOFWORD,
            WPARAM::default(),
            LPARAM::default(),
        );
        res != 0
    }

    fn register_image(&self, r#type: i32, xpm_data: &[&str]) {
        self.send_message(
            SCI_REGISTERIMAGE,
            WPARAM(r#type as usize),
            LPARAM(xpm_data.as_ptr() as isize),
        );
    }

    fn register_rgba_image(&self, r#type: i32, pixels: &[u8]) {
        let length = pixels.len();
        let mem = InProcessMemory::new(self.get_process_handle(), length + 1).unwrap();
        mem.write(pixels.as_ptr() as *const c_void, length);
        self.send_message(
            SCI_REGISTERRGBAIMAGE,
            WPARAM(r#type as usize),
            LPARAM(mem.as_ptr() as isize),
        );
    }

    fn clear_registered_images(&self) {
        self.send_message(
            SCI_CLEARREGISTEREDIMAGES,
            WPARAM::default(),
            LPARAM::default(),
        );
    }

    fn autoc_set_type_separator(&self, separator_character: u32) {
        self.send_message(
            SCI_AUTOCSETTYPESEPARATOR,
            WPARAM(separator_character as usize),
            LPARAM::default(),
        );
    }

    fn autoc_get_type_separator(&self) -> u32 {
        let (_, res) = self.send_message(
            SCI_AUTOCGETTYPESEPARATOR,
            WPARAM::default(),
            LPARAM::default(),
        );
        res as u32
    }

    fn autoc_set_max_height(&self, row_count: i32) {
        self.send_message(
            SCI_AUTOCSETMAXHEIGHT,
            WPARAM(row_count as usize),
            LPARAM::default(),
        );
    }

    fn autoc_get_max_height(&self) -> i32 {
        let (_, res) =
            self.send_message(SCI_AUTOCGETMAXHEIGHT, WPARAM::default(), LPARAM::default());
        res as i32
    }

    fn autoc_set_max_width(&self, character_count: i32) {
        self.send_message(
            SCI_AUTOCSETMAXWIDTH,
            WPARAM(character_count as usize),
            LPARAM::default(),
        );
    }

    fn autoc_get_max_width(&self) -> i32 {
        let (_, res) =
            self.send_message(SCI_AUTOCGETMAXWIDTH, WPARAM::default(), LPARAM::default());
        res as i32
    }

    fn user_list_show(&self, list_type: i32, item_list: String) {
        let length = item_list.as_bytes().len();
        let mem = InProcessMemory::new(self.get_process_handle(), length + 1).unwrap();
        mem.write(item_list.as_ptr() as *const c_void, length);
        self.send_message(
            SCI_USERLISTSHOW,
            WPARAM(list_type as usize),
            LPARAM(mem.as_ptr() as isize),
        );
    }

    fn call_tip_show(&self, pos: usize, definition: String) {
        let length = definition.as_bytes().len();
        let mem = InProcessMemory::new(self.get_process_handle(), length + 1).unwrap();
        mem.write(definition.as_ptr() as *const c_void, length);
        self.send_message(SCI_CALLTIPSHOW, WPARAM(pos), LPARAM(mem.as_ptr() as isize));
    }

    fn call_tip_cancel(&self) {
        self.send_message(SCI_CALLTIPCANCEL, WPARAM::default(), LPARAM::default());
    }

    fn call_tip_active(&self) -> bool {
        let (_, res) = self.send_message(SCI_CALLTIPACTIVE, WPARAM::default(), LPARAM::default());
        res != 0
    }

    fn call_tip_pos_start(&self) -> usize {
        let (_, res) = self.send_message(SCI_CALLTIPPOSSTART, WPARAM::default(), LPARAM::default());
        res
    }

    fn call_tip_set_pos_start(&self, pos_start: usize) {
        self.send_message(SCI_CALLTIPSETPOSSTART, WPARAM(pos_start), LPARAM::default());
    }

    fn call_tip_set_hlt(&self, highlight_start: usize, highlight_end: usize) {
        self.send_message(
            SCI_CALLTIPSETHLT,
            WPARAM(highlight_start),
            LPARAM(highlight_end as isize),
        );
    }

    fn call_tip_set_back(&self, back: i32) {
        self.send_message(SCI_CALLTIPSETBACK, WPARAM(back as usize), LPARAM::default());
    }

    fn call_tip_set_fore(&self, fore: i32) {
        self.send_message(SCI_CALLTIPSETFORE, WPARAM(fore as usize), LPARAM::default());
    }

    fn call_tip_set_fore_hlt(&self, fore: i32) {
        self.send_message(
            SCI_CALLTIPSETFOREHLT,
            WPARAM(fore as usize),
            LPARAM::default(),
        );
    }

    fn call_tip_use_style(&self, tab_size: i32) {
        self.send_message(
            SCI_CALLTIPUSESTYLE,
            WPARAM(tab_size as usize),
            LPARAM::default(),
        );
    }

    fn call_tip_set_position(&self, above: bool) {
        let above = if above { 1 } else { 0 };
        self.send_message(SCI_CALLTIPSETPOSITION, WPARAM(above), LPARAM::default());
    }

    fn line_down(&self) {
        self.send_message(SCI_LINEDOWN, WPARAM::default(), LPARAM::default());
    }

    fn line_down_extend(&self) {
        self.send_message(SCI_LINEDOWNEXTEND, WPARAM::default(), LPARAM::default());
    }

    fn line_down_rect_extend(&self) {
        self.send_message(SCI_LINEDOWNRECTEXTEND, WPARAM::default(), LPARAM::default());
    }

    fn line_scroll_down(&self) {
        self.send_message(SCI_LINESCROLLDOWN, WPARAM::default(), LPARAM::default());
    }

    fn line_up(&self) {
        self.send_message(SCI_LINEUP, WPARAM::default(), LPARAM::default());
    }

    fn line_up_extend(&self) {
        self.send_message(SCI_LINEUPEXTEND, WPARAM::default(), LPARAM::default());
    }

    fn line_up_rect_extend(&self) {
        self.send_message(SCI_LINEUPRECTEXTEND, WPARAM::default(), LPARAM::default());
    }

    fn line_scroll_up(&self) {
        self.send_message(SCI_LINESCROLLUP, WPARAM::default(), LPARAM::default());
    }

    fn para_down(&self) {
        self.send_message(SCI_PARADOWN, WPARAM::default(), LPARAM::default());
    }

    fn para_down_extend(&self) {
        self.send_message(SCI_PARADOWNEXTEND, WPARAM::default(), LPARAM::default());
    }

    fn para_up(&self) {
        self.send_message(SCI_PARAUP, WPARAM::default(), LPARAM::default());
    }

    fn para_up_extend(&self) {
        self.send_message(SCI_PARAUPEXTEND, WPARAM::default(), LPARAM::default());
    }

    fn char_left(&self) {
        self.send_message(SCI_CHARLEFT, WPARAM::default(), LPARAM::default());
    }

    fn char_left_extend(&self) {
        self.send_message(SCI_CHARLEFTEXTEND, WPARAM::default(), LPARAM::default());
    }

    fn char_left_rect_extend(&self) {
        self.send_message(SCI_CHARLEFTRECTEXTEND, WPARAM::default(), LPARAM::default());
    }

    fn char_right(&self) {
        self.send_message(SCI_CHARRIGHT, WPARAM::default(), LPARAM::default());
    }

    fn char_right_extend(&self) {
        self.send_message(SCI_CHARRIGHTEXTEND, WPARAM::default(), LPARAM::default());
    }

    fn char_right_rect_extend(&self) {
        self.send_message(
            SCI_CHARRIGHTRECTEXTEND,
            WPARAM::default(),
            LPARAM::default(),
        );
    }

    fn home(&self) {
        self.send_message(SCI_HOME, WPARAM::default(), LPARAM::default());
    }

    fn home_extend(&self) {
        self.send_message(SCI_HOMEEXTEND, WPARAM::default(), LPARAM::default());
    }

    fn home_rect_extend(&self) {
        self.send_message(SCI_HOMERECTEXTEND, WPARAM::default(), LPARAM::default());
    }

    fn home_display(&self) {
        self.send_message(SCI_HOMEDISPLAY, WPARAM::default(), LPARAM::default());
    }

    fn home_display_extend(&self) {
        self.send_message(SCI_HOMEDISPLAYEXTEND, WPARAM::default(), LPARAM::default());
    }

    fn home_wrap(&self) {
        self.send_message(SCI_HOMEWRAP, WPARAM::default(), LPARAM::default());
    }

    fn home_wrap_extend(&self) {
        self.send_message(SCI_HOMEWRAPEXTEND, WPARAM::default(), LPARAM::default());
    }

    fn vc_home(&self) {
        self.send_message(SCI_VCHOME, WPARAM::default(), LPARAM::default());
    }

    fn vc_home_extend(&self) {
        self.send_message(SCI_VCHOMEEXTEND, WPARAM::default(), LPARAM::default());
    }

    fn vc_home_rectextend(&self) {
        self.send_message(SCI_VCHOMERECTEXTEND, WPARAM::default(), LPARAM::default());
    }

    fn vc_home_wrap(&self) {
        self.send_message(SCI_VCHOMEWRAP, WPARAM::default(), LPARAM::default());
    }

    fn vc_home_wrap_extend(&self) {
        self.send_message(SCI_VCHOMEWRAPEXTEND, WPARAM::default(), LPARAM::default());
    }

    fn vc_home_display(&self) {
        self.send_message(SCI_VCHOMEDISPLAY, WPARAM::default(), LPARAM::default());
    }

    fn vc_home_display_extend(&self) {
        self.send_message(
            SCI_VCHOMEDISPLAYEXTEND,
            WPARAM::default(),
            LPARAM::default(),
        );
    }

    fn line_end(&self) {
        self.send_message(SCI_LINEEND, WPARAM::default(), LPARAM::default());
    }

    fn line_end_extend(&self) {
        self.send_message(SCI_LINEENDEXTEND, WPARAM::default(), LPARAM::default());
    }

    fn line_end_rect_extend(&self) {
        self.send_message(SCI_LINEENDRECTEXTEND, WPARAM::default(), LPARAM::default());
    }

    fn line_end_display(&self) {
        self.send_message(SCI_LINEENDDISPLAY, WPARAM::default(), LPARAM::default());
    }

    fn line_end_display_extend(&self) {
        self.send_message(
            SCI_LINEENDDISPLAYEXTEND,
            WPARAM::default(),
            LPARAM::default(),
        );
    }

    fn line_end_wrap(&self) {
        self.send_message(SCI_LINEENDWRAP, WPARAM::default(), LPARAM::default());
    }

    fn line_end_wrap_extend(&self) {
        self.send_message(SCI_LINEENDWRAPEXTEND, WPARAM::default(), LPARAM::default());
    }

    fn document_start(&self) {
        self.send_message(SCI_DOCUMENTSTART, WPARAM::default(), LPARAM::default());
    }

    fn document_start_extend(&self) {
        self.send_message(
            SCI_DOCUMENTSTARTEXTEND,
            WPARAM::default(),
            LPARAM::default(),
        );
    }

    fn document_end(&self) {
        self.send_message(SCI_DOCUMENTEND, WPARAM::default(), LPARAM::default());
    }

    fn document_end_extend(&self) {
        self.send_message(SCI_DOCUMENTENDEXTEND, WPARAM::default(), LPARAM::default());
    }

    fn page_up(&self) {
        self.send_message(SCI_PAGEUP, WPARAM::default(), LPARAM::default());
    }

    fn page_up_extend(&self) {
        self.send_message(SCI_PAGEUPEXTEND, WPARAM::default(), LPARAM::default());
    }

    fn page_up_rect_extend(&self) {
        self.send_message(SCI_PAGEUPRECTEXTEND, WPARAM::default(), LPARAM::default());
    }

    fn page_down(&self) {
        self.send_message(SCI_PAGEDOWN, WPARAM::default(), LPARAM::default());
    }

    fn page_down_extend(&self) {
        self.send_message(SCI_PAGEDOWNEXTEND, WPARAM::default(), LPARAM::default());
    }

    fn page_down_rect_extend(&self) {
        self.send_message(SCI_PAGEDOWNRECTEXTEND, WPARAM::default(), LPARAM::default());
    }

    fn stuttered_page_up(&self) {
        self.send_message(SCI_STUTTEREDPAGEUP, WPARAM::default(), LPARAM::default());
    }

    fn stuttered_page_up_extend(&self) {
        self.send_message(
            SCI_STUTTEREDPAGEUPEXTEND,
            WPARAM::default(),
            LPARAM::default(),
        );
    }

    fn stuttered_page_down(&self) {
        self.send_message(SCI_STUTTEREDPAGEDOWN, WPARAM::default(), LPARAM::default());
    }

    fn stuttered_page_down_extend(&self) {
        self.send_message(
            SCI_STUTTEREDPAGEDOWNEXTEND,
            WPARAM::default(),
            LPARAM::default(),
        );
    }

    fn delete_back(&self) {
        self.send_message(SCI_DELETEBACK, WPARAM::default(), LPARAM::default());
    }

    fn delete_back_not_line(&self) {
        self.send_message(SCI_DELETEBACKNOTLINE, WPARAM::default(), LPARAM::default());
    }

    fn del_line_left(&self) {
        self.send_message(SCI_DELLINELEFT, WPARAM::default(), LPARAM::default());
    }

    fn del_line_right(&self) {
        self.send_message(SCI_DELLINERIGHT, WPARAM::default(), LPARAM::default());
    }

    fn line_delete(&self) {
        self.send_message(SCI_LINEDELETE, WPARAM::default(), LPARAM::default());
    }

    fn line_cut(&self) {
        self.send_message(SCI_LINECUT, WPARAM::default(), LPARAM::default());
    }

    fn line_copy(&self) {
        self.send_message(SCI_LINECOPY, WPARAM::default(), LPARAM::default());
    }

    fn line_transpose(&self) {
        self.send_message(SCI_LINETRANSPOSE, WPARAM::default(), LPARAM::default());
    }

    fn line_reverse(&self) {
        self.send_message(SCI_LINEREVERSE, WPARAM::default(), LPARAM::default());
    }

    fn line_duplicate(&self) {
        self.send_message(SCI_LINEDUPLICATE, WPARAM::default(), LPARAM::default());
    }

    fn lower_case(&self) {
        self.send_message(SCI_LOWERCASE, WPARAM::default(), LPARAM::default());
    }

    fn upper_case(&self) {
        self.send_message(SCI_UPPERCASE, WPARAM::default(), LPARAM::default());
    }

    fn cancel(&self) {
        self.send_message(SCI_CANCEL, WPARAM::default(), LPARAM::default());
    }

    fn edit_toggle_overtype(&self) {
        self.send_message(SCI_EDITTOGGLEOVERTYPE, WPARAM::default(), LPARAM::default());
    }

    fn new_line(&self) {
        self.send_message(SCI_NEWLINE, WPARAM::default(), LPARAM::default());
    }

    fn form_feed(&self) {
        self.send_message(SCI_FORMFEED, WPARAM::default(), LPARAM::default());
    }

    fn tab(&self) {
        self.send_message(SCI_TAB, WPARAM::default(), LPARAM::default());
    }

    fn back_tab(&self) {
        self.send_message(SCI_BACKTAB, WPARAM::default(), LPARAM::default());
    }

    fn selection_duplicate(&self) {
        self.send_message(SCI_SELECTIONDUPLICATE, WPARAM::default(), LPARAM::default());
    }

    fn vertical_centre_caret(&self) {
        self.send_message(
            SCI_VERTICALCENTRECARET,
            WPARAM::default(),
            LPARAM::default(),
        );
    }

    fn scroll_tostart(&self) {
        self.send_message(SCI_SCROLLTOSTART, WPARAM::default(), LPARAM::default());
    }

    fn scroll_to_end(&self) {
        self.send_message(SCI_SCROLLTOEND, WPARAM::default(), LPARAM::default());
    }

    fn assign_cmd_key(&self, modifier: u32, key_code: KeyCode, sci_command: u32) {
        self.send_message(
            SCI_ASSIGNCMDKEY,
            WPARAM((modifier << 16 | Into::<u32>::into(key_code)) as usize),
            LPARAM(sci_command as isize),
        );
    }

    fn clear_cmd_key(&self, modifier: u32, key_code: KeyCode) {
        self.send_message(
            SCI_CLEARCMDKEY,
            WPARAM((modifier << 16 | Into::<u32>::into(key_code)) as usize),
            LPARAM::default(),
        );
    }

    fn clear_all_cmd_keys(&self) {
        self.send_message(SCI_CLEARALLCMDKEYS, WPARAM::default(), LPARAM::default());
    }

    fn use_pop_up(&self, pop_up_mode: PopUpMode) {
        self.send_message(
            SCI_USEPOPUP,
            WPARAM(Into::<u32>::into(pop_up_mode) as usize),
            LPARAM::default(),
        );
    }

    fn start_record(&self) {
        self.send_message(SCI_STARTRECORD, WPARAM::default(), LPARAM::default());
    }

    fn stop_record(&self) {
        self.send_message(SCI_STOPRECORD, WPARAM::default(), LPARAM::default());
    }

    fn format_range(
        &self,
        draw: bool,
        hdc: SurfaceId,
        hdc_target: SurfaceId,
        rc: Rectangle,
        rc_page: Rectangle,
        min: usize,
        max: usize,
    ) -> usize {
        let rtf = Sci_RangeToFormat {
            hdc,
            hdcTarget: hdc_target,
            rc,
            rcPage: rc_page,
            chrg: Sci_CharacterRange {
                cpMin: min as Sci_PositionCR,
                cpMax: max as Sci_PositionCR,
            },
        };
        let draw = if draw { 1 } else { 0 };
        let size = std::mem::size_of::<Sci_RangeToFormat>();
        let mem = InProcessMemory::new(self.get_process_handle(), size + 1).unwrap();
        mem.write(&rtf as *const Sci_RangeToFormat as *const c_void, size);
        let (_, res) =
            self.send_message(SCI_FORMATRANGE, WPARAM(draw), LPARAM(mem.as_ptr() as isize));
        res
    }

    fn set_print_magnification(&self, magnification: i32) {
        self.send_message(
            SCI_SETPRINTMAGNIFICATION,
            WPARAM(magnification as usize),
            LPARAM::default(),
        );
    }

    fn get_print_magnification(&self) -> i32 {
        let (_, res) = self.send_message(
            SCI_GETPRINTMAGNIFICATION,
            WPARAM::default(),
            LPARAM::default(),
        );
        res as i32
    }

    fn set_print_colour_mode(&self, mode: PrintMode) {
        self.send_message(
            SCI_SETPRINTCOLOURMODE,
            WPARAM(Into::<u32>::into(mode) as usize),
            LPARAM::default(),
        );
    }

    fn get_print_colour_mode(&self) -> PrintMode {
        let (_, res) =
            self.send_message(SCI_GETPRINTCOLOURMODE, WPARAM::default(), LPARAM::default());
        PrintMode::from(res as u32)
    }

    fn set_print_wrap_mode(&self, wrap_mode: WrapMode) {
        self.send_message(
            SCI_SETPRINTWRAPMODE,
            WPARAM(Into::<u32>::into(wrap_mode) as usize),
            LPARAM::default(),
        );
    }

    fn get_print_wrap_mode(&self) -> WrapMode {
        let (_, res) =
            self.send_message(SCI_GETPRINTWRAPMODE, WPARAM::default(), LPARAM::default());
        WrapMode::from(res as u32)
    }

    fn get_direct_function(&self) -> fn(isize, u32, usize, isize) -> isize {
        let (_, res) =
            self.send_message(SCI_GETDIRECTFUNCTION, WPARAM::default(), LPARAM::default());
        unsafe { std::mem::transmute(res as *const ()) }
    }

    fn get_direct_pointer(&self) -> isize {
        let (_, res) =
            self.send_message(SCI_GETDIRECTPOINTER, WPARAM::default(), LPARAM::default());
        res as isize
    }

    fn get_character_pointer(&self) -> isize {
        let (_, res) = self.send_message(
            SCI_GETCHARACTERPOINTER,
            WPARAM::default(),
            LPARAM::default(),
        );
        res as isize
    }

    fn get_range_pointer(&self, start: usize, length_range: usize) -> isize {
        let (_, res) = self.send_message(
            SCI_GETRANGEPOINTER,
            WPARAM(start),
            LPARAM(length_range as isize),
        );
        res as isize
    }

    fn get_gap_position(&self) -> usize {
        let (_, res) = self.send_message(SCI_GETGAPPOSITION, WPARAM::default(), LPARAM::default());
        res
    }

    fn get_doc_pointer(&self) -> isize {
        let (_, res) = self.send_message(SCI_GETDOCPOINTER, WPARAM::default(), LPARAM::default());
        res as isize
    }

    fn set_doc_pointer(&self, doc: isize) {
        self.send_message(SCI_SETDOCPOINTER, WPARAM::default(), LPARAM(doc));
    }

    fn create_document(&self, bytes: usize, document_options: u32) -> isize {
        let (_, res) = self.send_message(
            SCI_CREATEDOCUMENT,
            WPARAM(bytes),
            LPARAM(document_options as isize),
        );
        res as isize
    }

    fn add_ref_document(&self, doc: isize) {
        self.send_message(SCI_ADDREFDOCUMENT, WPARAM::default(), LPARAM(doc));
    }

    fn release_document(&self, doc: isize) {
        self.send_message(SCI_RELEASEDOCUMENT, WPARAM::default(), LPARAM(doc));
    }

    fn create_loader(&self, bytes: usize, document_options: u32) -> isize {
        let (_, res) = self.send_message(
            SCI_CREATELOADER,
            WPARAM(bytes),
            LPARAM(document_options as isize),
        );
        res as isize
    }

    fn visible_from_doc_line(&self, doc_line: usize) -> usize {
        let (_, res) =
            self.send_message(SCI_VISIBLEFROMDOCLINE, WPARAM(doc_line), LPARAM::default());
        res
    }

    fn show_lines(&self, line_start: usize, line_end: usize) {
        self.send_message(SCI_SHOWLINES, WPARAM(line_start), LPARAM(line_end as isize));
    }

    fn hide_lines(&self, line_start: usize, line_end: usize) {
        self.send_message(SCI_HIDELINES, WPARAM(line_start), LPARAM(line_end as isize));
    }

    fn get_line_visible(&self, line: usize) -> bool {
        let (_, res) = self.send_message(SCI_GETLINEVISIBLE, WPARAM(line), LPARAM::default());
        res != 0
    }

    fn get_all_lines_visible(&self) -> bool {
        let (_, res) =
            self.send_message(SCI_GETALLLINESVISIBLE, WPARAM::default(), LPARAM::default());
        res != 0
    }

    fn set_fold_level(&self, line: usize, level: u32) {
        self.send_message(SCI_SETFOLDLEVEL, WPARAM(line), LPARAM(level as isize));
    }

    fn get_fold_level(&self, line: usize) -> u32 {
        let (_, res) = self.send_message(SCI_GETFOLDLEVEL, WPARAM(line), LPARAM::default());
        res as u32
    }

    fn set_fold_flags(&self, flags: u32) {
        self.send_message(SCI_SETFOLDFLAGS, WPARAM(flags as usize), LPARAM::default());
    }

    fn get_last_child(&self, line: usize, level: u32) -> usize {
        let (_, res) = self.send_message(SCI_GETLASTCHILD, WPARAM(line), LPARAM(level as isize));
        res
    }

    fn get_fold_parent(&self, line: usize) -> usize {
        let (_, res) = self.send_message(SCI_GETFOLDPARENT, WPARAM(line), LPARAM::default());
        res
    }

    fn toggle_fold(&self, line: usize) {
        self.send_message(SCI_TOGGLEFOLD, WPARAM(line), LPARAM::default());
    }

    fn toggle_fold_show_text(&self, line: usize, text: String) {
        let length = text.as_bytes().len();
        let mem = InProcessMemory::new(self.get_process_handle(), length + 1).unwrap();
        mem.write(text.as_ptr() as *const c_void, length);
        self.send_message(
            SCI_TOGGLEFOLDSHOWTEXT,
            WPARAM(line),
            LPARAM(mem.as_ptr() as isize),
        );
    }

    fn fold_display_text_set_style(&self, style: FoldDisplayText) {
        self.send_message(
            SCI_FOLDDISPLAYTEXTSETSTYLE,
            WPARAM(Into::<u32>::into(style) as usize),
            LPARAM::default(),
        );
    }

    fn set_fold_expanded(&self, line: usize, expanded: bool) {
        let expanded = if expanded { 1 } else { 0 };
        self.send_message(SCI_SETFOLDEXPANDED, WPARAM(line), LPARAM(expanded));
    }

    fn get_fold_expanded(&self, line: usize) -> bool {
        let (_, res) = self.send_message(SCI_GETFOLDEXPANDED, WPARAM(line), LPARAM::default());
        res != 0
    }

    fn fold_line(&self, line: usize, action: FoldAction) {
        self.send_message(
            SCI_FOLDLINE,
            WPARAM(line),
            LPARAM(Into::<u32>::into(action) as isize),
        );
    }

    fn fold_children(&self, line: usize, action: FoldAction) {
        self.send_message(
            SCI_FOLDCHILDREN,
            WPARAM(line),
            LPARAM(Into::<u32>::into(action) as isize),
        );
    }

    fn fold_all(&self, action: FoldAction) {
        self.send_message(
            SCI_FOLDALL,
            WPARAM(Into::<u32>::into(action) as usize),
            LPARAM::default(),
        );
    }

    fn expand_children(&self, line: usize, level: u32) {
        self.send_message(SCI_EXPANDCHILDREN, WPARAM(line), LPARAM(level as isize));
    }

    fn set_automatic_fold(&self, automatic_fold: u32) {
        self.send_message(
            SCI_SETAUTOMATICFOLD,
            WPARAM(automatic_fold as usize),
            LPARAM::default(),
        );
    }

    fn get_automatic_fold(&self) -> u32 {
        let (_, res) =
            self.send_message(SCI_GETAUTOMATICFOLD, WPARAM::default(), LPARAM::default());
        res as u32
    }

    fn contracted_fold_next(&self, line_start: usize) -> usize {
        let (_, res) = self.send_message(
            SCI_CONTRACTEDFOLDNEXT,
            WPARAM(line_start),
            LPARAM::default(),
        );
        res
    }

    fn ensure_visible(&self, line: usize) {
        self.send_message(SCI_ENSUREVISIBLE, WPARAM(line), LPARAM::default());
    }

    fn ensure_visible_enforce_policy(&self, line: usize) {
        self.send_message(
            SCI_ENSUREVISIBLEENFORCEPOLICY,
            WPARAM(line),
            LPARAM::default(),
        );
    }

    fn set_wrap_mode(&self, wrap_mode: WrapMode) {
        self.send_message(
            SCI_SETWRAPMODE,
            WPARAM(Into::<u32>::into(wrap_mode) as usize),
            LPARAM::default(),
        );
    }

    fn get_wrap_mode(&self) -> WrapMode {
        let (_, res) = self.send_message(SCI_GETWRAPMODE, WPARAM::default(), LPARAM::default());
        WrapMode::from(res as u32)
    }

    fn set_wrap_visual_flags(&self, wrap_visual_flags: u32) {
        self.send_message(
            SCI_SETWRAPVISUALFLAGS,
            WPARAM(wrap_visual_flags as usize),
            LPARAM::default(),
        );
    }

    fn get_wrap_visual_flags(&self) -> u32 {
        let (_, res) =
            self.send_message(SCI_GETWRAPVISUALFLAGS, WPARAM::default(), LPARAM::default());
        res as u32
    }

    fn set_wrap_visual_flags_location(&self, wrap_visual_flags_location: u32) {
        self.send_message(
            SCI_SETWRAPVISUALFLAGSLOCATION,
            WPARAM(wrap_visual_flags_location as usize),
            LPARAM::default(),
        );
    }

    fn get_wrap_visual_flags_location(&self) -> u32 {
        let (_, res) = self.send_message(
            SCI_GETWRAPVISUALFLAGSLOCATION,
            WPARAM::default(),
            LPARAM::default(),
        );
        res as u32
    }

    fn set_wrap_indent_mode(&self, wrap_indent_mode: WrapIndent) {
        self.send_message(
            SCI_SETWRAPINDENTMODE,
            WPARAM(Into::<u32>::into(wrap_indent_mode) as usize),
            LPARAM::default(),
        );
    }

    fn get_wrap_indent_mode(&self) -> WrapIndent {
        let (_, res) =
            self.send_message(SCI_GETWRAPINDENTMODE, WPARAM::default(), LPARAM::default());
        WrapIndent::from(res as u32)
    }

    fn set_wrap_start_indent(&self, indent: i32) {
        self.send_message(
            SCI_SETWRAPSTARTINDENT,
            WPARAM(indent as usize),
            LPARAM::default(),
        );
    }

    fn get_wrap_start_indent(&self) -> i32 {
        let (_, res) =
            self.send_message(SCI_GETWRAPSTARTINDENT, WPARAM::default(), LPARAM::default());
        res as i32
    }

    fn set_layout_cache(&self, cache_mode: CacheMode) {
        self.send_message(
            SCI_SETLAYOUTCACHE,
            WPARAM(Into::<u32>::into(cache_mode) as usize),
            LPARAM::default(),
        );
    }

    fn get_layout_cache(&self) -> CacheMode {
        let (_, res) = self.send_message(SCI_GETLAYOUTCACHE, WPARAM::default(), LPARAM::default());
        CacheMode::from(res as u32)
    }

    fn set_position_cache(&self, size: i32) {
        self.send_message(
            SCI_SETPOSITIONCACHE,
            WPARAM(size as usize),
            LPARAM::default(),
        );
    }

    fn get_position_cache(&self) -> i32 {
        let (_, res) =
            self.send_message(SCI_GETPOSITIONCACHE, WPARAM::default(), LPARAM::default());
        res as i32
    }

    fn lines_split(&self, pixel_width: i32) {
        self.send_message(
            SCI_LINESSPLIT,
            WPARAM(pixel_width as usize),
            LPARAM::default(),
        );
    }

    fn lines_join(&self) {
        self.send_message(SCI_LINESJOIN, WPARAM::default(), LPARAM::default());
    }

    fn wrap_count(&self, doc_line: usize) -> usize {
        let (_, res) = self.send_message(SCI_WRAPCOUNT, WPARAM(doc_line), LPARAM::default());
        res
    }

    fn zoom_in(&self) {
        self.send_message(SCI_ZOOMIN, WPARAM::default(), LPARAM::default());
    }

    fn zoom_out(&self) {
        self.send_message(SCI_ZOOMOUT, WPARAM::default(), LPARAM::default());
    }

    fn set_zoom(&self, zoom_in_points: i32) {
        self.send_message(
            SCI_SETZOOM,
            WPARAM(zoom_in_points as usize),
            LPARAM::default(),
        );
    }

    fn get_zoom(&self) -> i32 {
        let (_, res) = self.send_message(SCI_GETZOOM, WPARAM::default(), LPARAM::default());
        res as i32
    }

    fn set_edge_mode(&self, edge_mode: EdgeMode) {
        self.send_message(
            SCI_SETEDGEMODE,
            WPARAM(Into::<u32>::into(edge_mode) as usize),
            LPARAM::default(),
        );
    }

    fn get_edge_mode(&self) -> EdgeMode {
        let (_, res) = self.send_message(SCI_GETEDGEMODE, WPARAM::default(), LPARAM::default());
        EdgeMode::from(res as u32)
    }

    fn set_edge_column(&self, column: usize) {
        self.send_message(SCI_SETEDGECOLUMN, WPARAM(column), LPARAM::default());
    }

    fn get_edge_column(&self) -> usize {
        let (_, res) = self.send_message(SCI_GETEDGECOLUMN, WPARAM::default(), LPARAM::default());
        res
    }

    fn set_edge_colour(&self, edge_colour: i32) {
        self.send_message(
            SCI_SETEDGECOLOUR,
            WPARAM(edge_colour as usize),
            LPARAM::default(),
        );
    }

    fn get_edge_colour(&self) -> i32 {
        let (_, res) = self.send_message(SCI_GETEDGECOLOUR, WPARAM::default(), LPARAM::default());
        res as i32
    }

    fn multi_edge_add_line(&self, column: usize, edge_colour: i32) {
        self.send_message(
            SCI_MULTIEDGEADDLINE,
            WPARAM(column),
            LPARAM(edge_colour as isize),
        );
    }

    fn multi_edge_clear_all(&self) {
        self.send_message(SCI_MULTIEDGECLEARALL, WPARAM::default(), LPARAM::default());
    }

    fn set_accessibility(&self, accessibility: Accessibility) {
        self.send_message(
            SCI_SETACCESSIBILITY,
            WPARAM(Into::<u32>::into(accessibility) as usize),
            LPARAM::default(),
        );
    }

    fn get_accessibility(&self) -> Accessibility {
        let (_, res) =
            self.send_message(SCI_GETACCESSIBILITY, WPARAM::default(), LPARAM::default());
        Accessibility::from(res as u32)
    }

    fn get_lexer(&self) -> u32 {
        let (_, res) = self.send_message(SCI_GETLEXER, WPARAM::default(), LPARAM::default());
        res as u32
    }

    fn get_lexer_language(&self) -> Option<String> {
        let (_, length) =
            self.send_message(SCI_GETLEXERLANGUAGE, WPARAM::default(), LPARAM::default());
        let mem = InProcessMemory::new(self.get_process_handle(), length + 1).unwrap();
        self.send_message(
            SCI_GETLEXERLANGUAGE,
            WPARAM(length),
            LPARAM(mem.as_ptr() as isize),
        );
        mem.read(|buf| (buf as *const u8).to_string())
    }

    fn colourise(&self, start: usize, end: usize) {
        self.send_message(SCI_COLOURISE, WPARAM(start), LPARAM(end as isize));
    }

    fn change_lexer_state(&self, start: usize, end: usize) -> i32 {
        let (_, res) = self.send_message(SCI_CHANGELEXERSTATE, WPARAM(start), LPARAM(end as isize));
        res as i32
    }

    fn property_names(&self) -> Option<String> {
        let (_, length) =
            self.send_message(SCI_PROPERTYNAMES, WPARAM::default(), LPARAM::default());
        let mem = InProcessMemory::new(self.get_process_handle(), length + 1).unwrap();
        self.send_message(
            SCI_PROPERTYNAMES,
            WPARAM(length),
            LPARAM(mem.as_ptr() as isize),
        );
        mem.read(|buf| (buf as *const u8).to_string())
    }

    fn property_type(&self, name: String) -> PropertyType {
        let size = name.as_bytes().len();
        let mem = InProcessMemory::new(self.get_process_handle(), size + 1).unwrap();
        mem.write(name.as_ptr() as *const c_void, size);
        let (_, res) = self.send_message(
            SCI_PROPERTYTYPE,
            WPARAM(mem.as_ptr() as usize),
            LPARAM::default(),
        );
        PropertyType::from(res as u32)
    }

    fn describe_property(&self, name: String) -> Option<String> {
        let size = name.as_bytes().len();
        let mem = InProcessMemory::new(self.get_process_handle(), size + 1).unwrap();
        mem.write(name.as_ptr() as *const c_void, size);
        let (_, length) = self.send_message(
            SCI_DESCRIBEPROPERTY,
            WPARAM(mem.as_ptr() as usize),
            LPARAM::default(),
        );
        let mem2 = InProcessMemory::new(self.get_process_handle(), length + 1).unwrap();
        self.send_message(
            SCI_DESCRIBEPROPERTY,
            WPARAM(mem.as_ptr() as usize),
            LPARAM(mem2.as_ptr() as isize),
        );
        mem2.read(|buf| (buf as *const u8).to_string())
    }

    fn set_property(&self, key: String, value: String) {
        let size = key.as_bytes().len();
        let mem = InProcessMemory::new(self.get_process_handle(), size + 1).unwrap();
        mem.write(key.as_ptr() as *const c_void, size);
        let size = value.as_bytes().len();
        let mem2 = InProcessMemory::new(self.get_process_handle(), size + 1).unwrap();
        mem2.write(value.as_ptr() as *const c_void, size);
        self.send_message(
            SCI_SETPROPERTY,
            WPARAM(mem.as_ptr() as usize),
            LPARAM(mem2.as_ptr() as isize),
        );
    }

    fn get_property_expanded(&self, key: String) -> Option<String> {
        let size = key.as_bytes().len();
        let mem = InProcessMemory::new(self.get_process_handle(), size + 1).unwrap();
        mem.write(key.as_ptr() as *const c_void, size);
        let (_, length) = self.send_message(
            SCI_GETPROPERTYEXPANDED,
            WPARAM(mem.as_ptr() as usize),
            LPARAM::default(),
        );
        let mem2 = InProcessMemory::new(self.get_process_handle(), length + 1).unwrap();
        self.send_message(
            SCI_GETPROPERTYEXPANDED,
            WPARAM(mem.as_ptr() as usize),
            LPARAM(mem2.as_ptr() as isize),
        );
        mem2.read(|buf| (buf as *const u8).to_string())
    }

    fn get_property_int(&self, key: String, default_value: u32) -> u32 {
        let size = key.as_bytes().len();
        let mem = InProcessMemory::new(self.get_process_handle(), size + 1).unwrap();
        mem.write(key.as_ptr() as *const c_void, size);
        let (_, res) = self.send_message(
            SCI_GETPROPERTYINT,
            WPARAM(mem.as_ptr() as usize),
            LPARAM(default_value as isize),
        );
        res as u32
    }

    fn get_property(&self, key: String) -> Option<String> {
        let size = key.as_bytes().len();
        let mem = InProcessMemory::new(self.get_process_handle(), size + 1).unwrap();
        mem.write(key.as_ptr() as *const c_void, size);
        let (_, length) = self.send_message(
            SCI_GETPROPERTY,
            WPARAM(mem.as_ptr() as usize),
            LPARAM::default(),
        );
        let mem2 = InProcessMemory::new(self.get_process_handle(), length + 1).unwrap();
        self.send_message(
            SCI_GETPROPERTY,
            WPARAM(mem.as_ptr() as usize),
            LPARAM(mem2.as_ptr() as isize),
        );
        mem2.read(|buf| (buf as *const u8).to_string())
    }

    fn set_keywords(&self, key_word_set: u32, key_words: String) {
        let size = key_words.as_bytes().len();
        let mem = InProcessMemory::new(self.get_process_handle(), size + 1).unwrap();
        mem.write(key_words.as_ptr() as *const c_void, size);
        self.send_message(
            SCI_SETKEYWORDS,
            WPARAM(key_word_set as usize),
            LPARAM(mem.as_ptr() as isize),
        );
    }

    fn describe_keyword_sets(&self) -> Option<String> {
        let (_, length) = self.send_message(
            SCI_DESCRIBEKEYWORDSETS,
            WPARAM::default(),
            LPARAM::default(),
        );
        let mem = InProcessMemory::new(self.get_process_handle(), length + 1).unwrap();
        self.send_message(
            SCI_DESCRIBEKEYWORDSETS,
            WPARAM(length),
            LPARAM(mem.as_ptr() as isize),
        );
        mem.read(|buf| (buf as *const u8).to_string())
    }

    fn get_substyle_bases(&self) -> Option<Vec<u8>> {
        let (_, length) =
            self.send_message(SCI_GETSUBSTYLEBASES, WPARAM::default(), LPARAM::default());
        let mem = InProcessMemory::new(self.get_process_handle(), length + 1).unwrap();
        self.send_message(
            SCI_GETSUBSTYLEBASES,
            WPARAM(length),
            LPARAM(mem.as_ptr() as isize),
        );
        mem.read(|buf| (buf as *const u8).to_vec(length))
    }

    fn distance_to_secondary_styles(&self) -> i32 {
        let (_, res) = self.send_message(
            SCI_DISTANCETOSECONDARYSTYLES,
            WPARAM::default(),
            LPARAM::default(),
        );
        res as i32
    }

    fn allocate_substyles(&self, style_base: i32, number_styles: i32) -> i32 {
        let (_, res) = self.send_message(
            SCI_ALLOCATESUBSTYLES,
            WPARAM(style_base as usize),
            LPARAM(number_styles as isize),
        );
        res as i32
    }

    fn free_substyles(&self) {
        self.send_message(SCI_FREESUBSTYLES, WPARAM::default(), LPARAM::default());
    }

    fn get_substyles_start(&self, style_base: i32) -> i32 {
        let (_, res) = self.send_message(
            SCI_GETSUBSTYLESSTART,
            WPARAM(style_base as usize),
            LPARAM::default(),
        );
        res as i32
    }

    fn get_substyles_length(&self, style_base: i32) -> i32 {
        let (_, res) = self.send_message(
            SCI_GETSUBSTYLESLENGTH,
            WPARAM(style_base as usize),
            LPARAM::default(),
        );
        res as i32
    }

    fn get_style_from_substyle(&self, sub_style: i32) -> i32 {
        let (_, res) = self.send_message(
            SCI_GETSTYLEFROMSUBSTYLE,
            WPARAM(sub_style as usize),
            LPARAM::default(),
        );
        res as i32
    }

    fn get_primary_style_from_style(&self, style: i32) -> i32 {
        let (_, res) = self.send_message(
            SCI_GETPRIMARYSTYLEFROMSTYLE,
            WPARAM(style as usize),
            LPARAM::default(),
        );
        res as i32
    }

    fn set_identifiers(&self, style: i32, identifiers: String) {
        let size = identifiers.as_bytes().len();
        let mem = InProcessMemory::new(self.get_process_handle(), size + 1).unwrap();
        mem.write(identifiers.as_ptr() as *const c_void, size);
        self.send_message(
            SCI_SETIDENTIFIERS,
            WPARAM(style as usize),
            LPARAM(mem.as_ptr() as isize),
        );
    }

    fn private_lexer_call(&self, operation: i32, pointer: isize) -> isize {
        let (_, res) = self.send_message(
            SCI_PRIVATELEXERCALL,
            WPARAM(operation as usize),
            LPARAM(pointer),
        );
        res as isize
    }

    fn get_named_styles(&self) -> i32 {
        let (_, res) = self.send_message(SCI_GETNAMEDSTYLES, WPARAM::default(), LPARAM::default());
        res as i32
    }

    fn name_of_style(&self, style: i32) -> Option<String> {
        let (_, length) =
            self.send_message(SCI_NAMEOFSTYLE, WPARAM(style as usize), LPARAM::default());
        let mem = InProcessMemory::new(self.get_process_handle(), length + 1).unwrap();
        self.send_message(
            SCI_NAMEOFSTYLE,
            WPARAM(style as usize),
            LPARAM(mem.as_ptr() as isize),
        );
        mem.read(|buf| (buf as *const u8).to_string())
    }

    fn tags_of_style(&self, style: i32) -> Option<String> {
        let (_, length) =
            self.send_message(SCI_TAGSOFSTYLE, WPARAM(style as usize), LPARAM::default());
        let mem = InProcessMemory::new(self.get_process_handle(), length + 1).unwrap();
        self.send_message(
            SCI_TAGSOFSTYLE,
            WPARAM(style as usize),
            LPARAM(mem.as_ptr() as isize),
        );
        mem.read(|buf| (buf as *const u8).to_string())
    }

    fn description_of_style(&self, style: i32) -> Option<String> {
        let (_, length) = self.send_message(
            SCI_DESCRIPTIONOFSTYLE,
            WPARAM(style as usize),
            LPARAM::default(),
        );
        let mem = InProcessMemory::new(self.get_process_handle(), length + 1).unwrap();
        self.send_message(
            SCI_DESCRIPTIONOFSTYLE,
            WPARAM(style as usize),
            LPARAM(mem.as_ptr() as isize),
        );
        mem.read(|buf| (buf as *const u8).to_string())
    }
}
