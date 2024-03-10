use crate::bindings::resource_types;
use crate::globals::{
    Qalpha, Qalpha_background, Qauto_lower, Qauto_raise, Qbackground_color, Qborder_color,
    Qborder_width, Qbottom_divider_width, Qbuffer_predicate, Qchild_frame_border_width,
    Qcursor_color, Qcursor_type, Qdisplay, Qfont, Qfont_backend, Qforeground_color, Qfullscreen,
    Qheight, Qhorizontal_scroll_bars, Qicon_name, Qicon_type, Qinhibit_double_buffering,
    Qinternal_border_width, Qleft_fringe, Qline_spacing, Qmenu_bar_lines, Qmin_height, Qmin_width,
    Qminibuffer, Qmouse_color, Qname, Qno_accept_focus, Qno_focus_on_map, Qno_special_glyphs,
    Qns_appearance, Qns_transparent_titlebar, Qoverride_redirect, Qparent_frame, Qparent_id,
    Qright_divider_width, Qright_fringe, Qscreen_gamma, Qscroll_bar_background,
    Qscroll_bar_foreground, Qscroll_bar_height, Qscroll_bar_width, Qshaded, Qskip_taskbar, Qsticky,
    Qtab_bar_lines, Qterminal, Qtitle, Qtool_bar_lines, Qtool_bar_position, Qundecorated,
    Qunsplittable, Quse_frame_synchronization, Qvertical_scroll_bars, Qvisibility, Qwait_for_wm,
    Qwidth, Qz_group,
};
use crate::lisp::LispObject;
use std::ffi::CString;

pub enum ResourceType {
    Num,
    Float,
    Bool,
    Str,
    Symbol,
    BoolNum,
}

impl From<resource_types::Type> for ResourceType {
    fn from(t: resource_types::Type) -> Self {
        match t {
            resource_types::RES_TYPE_NUMBER => ResourceType::Num,
            resource_types::RES_TYPE_FLOAT => ResourceType::Float,
            resource_types::RES_TYPE_BOOLEAN => ResourceType::Bool,
            resource_types::RES_TYPE_STRING => ResourceType::Str,
            resource_types::RES_TYPE_SYMBOL => ResourceType::Symbol,
            resource_types::RES_TYPE_BOOLEAN_NUMBER => ResourceType::BoolNum,
            _ => panic!("unsupported resource type"),
        }
    }
}

impl Into<resource_types::Type> for ResourceType {
    fn into(self) -> resource_types::Type {
        match self {
            ResourceType::Num => resource_types::RES_TYPE_NUMBER,
            ResourceType::Float => resource_types::RES_TYPE_FLOAT,
            ResourceType::Bool => resource_types::RES_TYPE_BOOLEAN,
            ResourceType::Str => resource_types::RES_TYPE_STRING,
            ResourceType::Symbol => resource_types::RES_TYPE_SYMBOL,
            ResourceType::BoolNum => resource_types::RES_TYPE_BOOLEAN_NUMBER,
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum FrameParam {
    Alpha,
    AlphaBackground,
    AutoLower,
    AutoRaise,
    BackgroundColor,
    BorderColor,
    BorderWidth,
    BottomDividerWidth,
    BufferPredicate,
    ChildFrameBorderWidth,
    CursorColor,
    CursorType,
    Font,
    FontBackend,
    ForegroundColor,
    Fullscreen,
    HorizontalScrollBars,
    IconName,
    IconType,
    InhibitDoubleBuffering,
    InternalBorderWidth,
    LeftFringe,
    LineSpacing,
    MenuBarLines,
    Minibuffer,
    MouseColor,
    MinWidth,
    MinHeight,
    Width,
    Height,
    Name,
    NoAcceptFocus,
    NoFocusOnMap,
    NoSpecialGlyphs,
    NsAppearance,
    NsTransparentTitlebar,
    OverrideRedirect,
    ParentFrame,
    ParentId,
    RightDividerWidth,
    RightFringe,
    ScreenGamma,
    ScrollBarBackground,
    ScrollBarForeground,
    ScrollBarHeight,
    Terminal,
    Display,
    ScrollBarWidth,
    Shaded,
    SkipTaskbar,
    Sticky,
    TabBarLines,
    Title,
    ToolBarLines,
    ToolBarPosition,
    Undecorated,
    Unsplittable,
    UseFrameSynchronization,
    VerticalScrollBars,
    Visibility,
    WaitForWm,
    ZGroup,
}

impl FrameParam {
    //  Resources are grouped into named classes.  For instance, the
    // ‘Foreground’ class contains the ‘cursorColor’, ‘foreground’ and
    // ‘pointerColor’ resources (*note Table of Resources::)
    pub fn x_resource(&self) -> (CString, CString) {
        let (name, class) = match self {
            FrameParam::Alpha => ("alpha", "Alpha"),
            FrameParam::AlphaBackground => ("alphaBackground", "AlphaBackground"),
            FrameParam::AutoLower => ("autoRaise", "AutoRaiseLower"),
            FrameParam::AutoRaise => ("autoLower", "AutoRaiseLower"),
            FrameParam::BackgroundColor => ("background", "Background"),
            FrameParam::BorderColor => ("borderColor", "BorderColor"),
            FrameParam::BorderWidth => ("borderWidth", "BorderWidth"),
            FrameParam::BufferPredicate => ("bufferPredicate", "BufferPredicate"),
            FrameParam::ChildFrameBorderWidth => ("childFrameBorderWidth", "childFrameBorderWidth"),
            FrameParam::CursorColor => ("cursorColor", "Foreground"),
            FrameParam::CursorType => ("cursorType", "CursorType"),
            FrameParam::Font => ("font", "Font"),
            FrameParam::FontBackend => ("fontBackend", "FontBackend"),
            FrameParam::ForegroundColor => ("foreground", "Foreground"),
            FrameParam::Fullscreen => ("fullscreen", "Fullscreen"),
            FrameParam::HorizontalScrollBars => ("horizontalScrollBars", "ScrollBars"),
            FrameParam::IconName => ("iconName", "Title"),
            FrameParam::IconType => ("bitmapIcon", "BitmapIcon"),
            FrameParam::InhibitDoubleBuffering => {
                ("inhibitDoubleBuffering", "InhibitDoubleBuffering")
            }
            FrameParam::InternalBorderWidth => ("internalBorderWidth", "internalBorderWidth"),
            FrameParam::LeftFringe => ("leftFringe", "LeftFringe"),
            FrameParam::LineSpacing => ("lineSpacing", "LineSpacing"),
            FrameParam::MouseColor => ("pointerColor", "Foreground"),
            FrameParam::Minibuffer => ("minibuffer", "Minibuffer"),
            FrameParam::Name => ("name", "Name"),
            FrameParam::RightFringe => ("rightFringe", "RightFringe"),
            FrameParam::ScreenGamma => ("screenGamma", "ScreenGamma"),
            FrameParam::ScrollBarBackground => ("scrollBarBackground", "ScrollBarBackground"),
            FrameParam::ScrollBarForeground => ("scrollBarForeground", "ScrollBarForeground"),
            FrameParam::ScrollBarHeight => ("scrollBarHeight", "ScrollBarHeight"),
            FrameParam::ScrollBarWidth => ("scrollBarWidth", "ScrollBarWidth"),
            FrameParam::Title => ("title", "Title"),
            FrameParam::VerticalScrollBars => ("verticalScrollBars", "ScrollBars"),
            FrameParam::WaitForWm => ("waitForWM", "WaitForWM"),
            _ => ("", ""),
        };
        (CString::new(name).unwrap(), CString::new(class).unwrap())
    }

    pub fn resource_type(&self) -> ResourceType {
        match self {
            FrameParam::Alpha
            | FrameParam::AlphaBackground
            | FrameParam::BorderWidth
            | FrameParam::BottomDividerWidth
            | FrameParam::ChildFrameBorderWidth
            | FrameParam::InternalBorderWidth
            | FrameParam::LeftFringe
            | FrameParam::LineSpacing
            | FrameParam::MenuBarLines
            | FrameParam::MinWidth
            | FrameParam::MinHeight
            | FrameParam::Width
            | FrameParam::Height
            | FrameParam::RightDividerWidth
            | FrameParam::RightFringe
            | FrameParam::ScrollBarHeight
            | FrameParam::ScrollBarWidth
            | FrameParam::TabBarLines
            | FrameParam::ParentId
            | FrameParam::Terminal
            | FrameParam::ToolBarLines => ResourceType::Num,

            FrameParam::AutoLower
            | FrameParam::AutoRaise
            | FrameParam::IconType
            | FrameParam::InhibitDoubleBuffering
            | FrameParam::NoAcceptFocus
            | FrameParam::NoFocusOnMap
            | FrameParam::NoSpecialGlyphs
            | FrameParam::NsTransparentTitlebar
            | FrameParam::OverrideRedirect
            | FrameParam::Shaded
            | FrameParam::SkipTaskbar
            | FrameParam::Sticky
            | FrameParam::Undecorated
            | FrameParam::Unsplittable
            | FrameParam::UseFrameSynchronization
            | FrameParam::WaitForWm => ResourceType::Bool,

            FrameParam::BackgroundColor
            | FrameParam::BorderColor
            | FrameParam::CursorColor
            | FrameParam::Font
            | FrameParam::FontBackend
            | FrameParam::ForegroundColor
            | FrameParam::IconName
            | FrameParam::MouseColor
            | FrameParam::Name
            | FrameParam::ScrollBarBackground
            | FrameParam::ScrollBarForeground
            | FrameParam::Display
            | FrameParam::Title => ResourceType::Str,

            FrameParam::BufferPredicate
            | FrameParam::CursorType
            | FrameParam::Fullscreen
            | FrameParam::HorizontalScrollBars
            | FrameParam::NsAppearance
            | FrameParam::ParentFrame
            | FrameParam::ToolBarPosition
            | FrameParam::VerticalScrollBars
            | FrameParam::Visibility
            | FrameParam::Minibuffer
            | FrameParam::ZGroup => ResourceType::Symbol,

            FrameParam::ScreenGamma => ResourceType::Float,
        }
    }
}

impl From<LispObject> for FrameParam {
    fn from(param: LispObject) -> FrameParam {
        match param {
            Qalpha => FrameParam::Alpha,
            Qalpha_background => FrameParam::AlphaBackground,
            Qauto_lower => FrameParam::AutoLower,
            Qauto_raise => FrameParam::AutoRaise,
            Qbackground_color => FrameParam::BackgroundColor,
            Qborder_color => FrameParam::BorderColor,
            Qborder_width => FrameParam::BorderWidth,
            Qbottom_divider_width => FrameParam::BottomDividerWidth,
            Qbuffer_predicate => FrameParam::BufferPredicate,
            Qchild_frame_border_width => FrameParam::ChildFrameBorderWidth,
            Qcursor_color => FrameParam::CursorColor,
            Qcursor_type => FrameParam::CursorType,
            Qfont => FrameParam::Font,
            Qfont_backend => FrameParam::FontBackend,
            Qforeground_color => FrameParam::ForegroundColor,
            Qfullscreen => FrameParam::Fullscreen,
            Qhorizontal_scroll_bars => FrameParam::HorizontalScrollBars,
            Qicon_name => FrameParam::IconName,
            Qicon_type => FrameParam::IconType,
            Qinhibit_double_buffering => FrameParam::InhibitDoubleBuffering,
            Qinternal_border_width => FrameParam::InternalBorderWidth,
            Qleft_fringe => FrameParam::LeftFringe,
            Qline_spacing => FrameParam::LineSpacing,
            Qmenu_bar_lines => FrameParam::MenuBarLines,
            Qmouse_color => FrameParam::MouseColor,
            Qmin_width => FrameParam::MinWidth,
            Qmin_height => FrameParam::MinHeight,
            Qwidth => FrameParam::Width,
            Qheight => FrameParam::Height,
            Qname => FrameParam::Name,
            Qno_accept_focus => FrameParam::NoAcceptFocus,
            Qno_focus_on_map => FrameParam::NoFocusOnMap,
            Qno_special_glyphs => FrameParam::NoSpecialGlyphs,
            Qns_appearance => FrameParam::NsAppearance,
            Qns_transparent_titlebar => FrameParam::NsTransparentTitlebar,
            Qoverride_redirect => FrameParam::OverrideRedirect,
            Qparent_frame => FrameParam::ParentFrame,
            Qparent_id => FrameParam::ParentId,
            Qright_divider_width => FrameParam::RightDividerWidth,
            Qright_fringe => FrameParam::RightFringe,
            Qscreen_gamma => FrameParam::ScreenGamma,
            Qminibuffer => FrameParam::Minibuffer,
            Qscroll_bar_background => FrameParam::ScrollBarBackground,
            Qscroll_bar_foreground => FrameParam::ScrollBarForeground,
            Qscroll_bar_height => FrameParam::ScrollBarHeight,
            Qscroll_bar_width => FrameParam::ScrollBarWidth,
            Qshaded => FrameParam::Shaded,
            Qskip_taskbar => FrameParam::SkipTaskbar,
            Qsticky => FrameParam::Sticky,
            Qtab_bar_lines => FrameParam::TabBarLines,
            Qtitle => FrameParam::Title,
            Qtool_bar_lines => FrameParam::ToolBarLines,
            Qterminal => FrameParam::Terminal,
            Qdisplay => FrameParam::Display,
            Qtool_bar_position => FrameParam::ToolBarPosition,
            Qundecorated => FrameParam::Undecorated,
            Qunsplittable => FrameParam::Unsplittable,
            Quse_frame_synchronization => FrameParam::UseFrameSynchronization,
            Qvertical_scroll_bars => FrameParam::VerticalScrollBars,
            Qvisibility => FrameParam::Visibility,
            Qwait_for_wm => FrameParam::WaitForWm,
            Qz_group => FrameParam::ZGroup,
            _ => panic!("unknow frame param {param:?}"),
        }
    }
}

impl Into<LispObject> for FrameParam {
    fn into(self) -> LispObject {
        match self {
            FrameParam::Alpha => Qalpha,
            FrameParam::AlphaBackground => Qalpha_background,
            FrameParam::AutoLower => Qauto_lower,
            FrameParam::AutoRaise => Qauto_raise,
            FrameParam::BackgroundColor => Qbackground_color,
            FrameParam::BorderColor => Qborder_color,
            FrameParam::BorderWidth => Qborder_width,
            FrameParam::BottomDividerWidth => Qbottom_divider_width,
            FrameParam::BufferPredicate => Qbuffer_predicate,
            FrameParam::ChildFrameBorderWidth => Qchild_frame_border_width,
            FrameParam::CursorColor => Qcursor_color,
            FrameParam::CursorType => Qcursor_type,
            FrameParam::Font => Qfont,
            FrameParam::FontBackend => Qfont_backend,
            FrameParam::ForegroundColor => Qforeground_color,
            FrameParam::Fullscreen => Qfullscreen,
            FrameParam::HorizontalScrollBars => Qhorizontal_scroll_bars,
            FrameParam::IconName => Qicon_name,
            FrameParam::IconType => Qicon_type,
            FrameParam::InhibitDoubleBuffering => Qinhibit_double_buffering,
            FrameParam::InternalBorderWidth => Qinternal_border_width,
            FrameParam::LeftFringe => Qleft_fringe,
            FrameParam::LineSpacing => Qline_spacing,
            FrameParam::MenuBarLines => Qmenu_bar_lines,
            FrameParam::MouseColor => Qmouse_color,
            FrameParam::Minibuffer => Qminibuffer,
            FrameParam::MinWidth => Qmin_width,
            FrameParam::MinHeight => Qmin_height,
            FrameParam::Width => Qwidth,
            FrameParam::Height => Qheight,
            FrameParam::Name => Qname,
            FrameParam::NoAcceptFocus => Qno_accept_focus,
            FrameParam::NoFocusOnMap => Qno_focus_on_map,
            FrameParam::NoSpecialGlyphs => Qno_special_glyphs,
            FrameParam::NsAppearance => Qns_appearance,
            FrameParam::NsTransparentTitlebar => Qns_transparent_titlebar,
            FrameParam::OverrideRedirect => Qoverride_redirect,
            FrameParam::ParentFrame => Qparent_frame,
            FrameParam::ParentId => Qparent_id,
            FrameParam::RightDividerWidth => Qright_divider_width,
            FrameParam::RightFringe => Qright_fringe,
            FrameParam::ScreenGamma => Qscreen_gamma,
            FrameParam::ScrollBarBackground => Qscroll_bar_background,
            FrameParam::ScrollBarForeground => Qscroll_bar_foreground,
            FrameParam::ScrollBarHeight => Qscroll_bar_height,
            FrameParam::ScrollBarWidth => Qscroll_bar_width,
            FrameParam::Shaded => Qshaded,
            FrameParam::SkipTaskbar => Qskip_taskbar,
            FrameParam::Sticky => Qsticky,
            FrameParam::TabBarLines => Qtab_bar_lines,
            FrameParam::Title => Qtitle,
            FrameParam::ToolBarLines => Qtool_bar_lines,
            FrameParam::Terminal => Qterminal,
            FrameParam::Display => Qdisplay,
            FrameParam::ToolBarPosition => Qtool_bar_position,
            FrameParam::Undecorated => Qundecorated,
            FrameParam::Unsplittable => Qunsplittable,
            FrameParam::UseFrameSynchronization => Quse_frame_synchronization,
            FrameParam::VerticalScrollBars => Qvertical_scroll_bars,
            FrameParam::Visibility => Qvisibility,
            FrameParam::WaitForWm => Qwait_for_wm,
            FrameParam::ZGroup => Qz_group,
        }
    }
}
