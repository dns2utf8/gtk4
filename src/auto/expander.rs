// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::ToValue;
use glib_sys;
use gobject_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Align;
use Buildable;
use Container;
use LayoutManager;
use Overflow;
use Widget;

glib_wrapper! {
    pub struct Expander(Object<gtk_sys::GtkExpander, gtk_sys::GtkExpanderClass, ExpanderClass>) @extends Container, Widget, @implements Buildable;

    match fn {
        get_type => || gtk_sys::gtk_expander_get_type(),
    }
}

impl Expander {
    pub fn new(label: Option<&str>) -> Expander {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_expander_new(label.to_glib_none().0)).unsafe_cast()
        }
    }

    pub fn new_with_mnemonic(label: Option<&str>) -> Expander {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_expander_new_with_mnemonic(
                label.to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }
}

#[derive(Clone, Default)]
pub struct ExpanderBuilder {
    expanded: Option<bool>,
    label: Option<String>,
    label_widget: Option<Widget>,
    resize_toplevel: Option<bool>,
    use_markup: Option<bool>,
    use_underline: Option<bool>,
    can_focus: Option<bool>,
    can_target: Option<bool>,
    css_name: Option<String>,
    cursor: Option<gdk::Cursor>,
    expand: Option<bool>,
    focus_on_click: Option<bool>,
    halign: Option<Align>,
    has_focus: Option<bool>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    is_focus: Option<bool>,
    layout_manager: Option<LayoutManager>,
    margin: Option<i32>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    opacity: Option<f64>,
    overflow: Option<Overflow>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
}

impl ExpanderBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Expander {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref expanded) = self.expanded {
            properties.push(("expanded", expanded));
        }
        if let Some(ref label) = self.label {
            properties.push(("label", label));
        }
        if let Some(ref label_widget) = self.label_widget {
            properties.push(("label-widget", label_widget));
        }
        if let Some(ref resize_toplevel) = self.resize_toplevel {
            properties.push(("resize-toplevel", resize_toplevel));
        }
        if let Some(ref use_markup) = self.use_markup {
            properties.push(("use-markup", use_markup));
        }
        if let Some(ref use_underline) = self.use_underline {
            properties.push(("use-underline", use_underline));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref can_target) = self.can_target {
            properties.push(("can-target", can_target));
        }
        if let Some(ref css_name) = self.css_name {
            properties.push(("css-name", css_name));
        }
        if let Some(ref cursor) = self.cursor {
            properties.push(("cursor", cursor));
        }
        if let Some(ref expand) = self.expand {
            properties.push(("expand", expand));
        }
        if let Some(ref focus_on_click) = self.focus_on_click {
            properties.push(("focus-on-click", focus_on_click));
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_focus) = self.has_focus {
            properties.push(("has-focus", has_focus));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref is_focus) = self.is_focus {
            properties.push(("is-focus", is_focus));
        }
        if let Some(ref layout_manager) = self.layout_manager {
            properties.push(("layout-manager", layout_manager));
        }
        if let Some(ref margin) = self.margin {
            properties.push(("margin", margin));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref overflow) = self.overflow {
            properties.push(("overflow", overflow));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        glib::Object::new(Expander::static_type(), &properties)
            .expect("object new")
            .downcast()
            .expect("downcast")
    }

    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = Some(expanded);
        self
    }

    pub fn label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }

    pub fn label_widget<P: IsA<Widget>>(mut self, label_widget: &P) -> Self {
        self.label_widget = Some(label_widget.clone().upcast());
        self
    }

    pub fn resize_toplevel(mut self, resize_toplevel: bool) -> Self {
        self.resize_toplevel = Some(resize_toplevel);
        self
    }

    pub fn use_markup(mut self, use_markup: bool) -> Self {
        self.use_markup = Some(use_markup);
        self
    }

    pub fn use_underline(mut self, use_underline: bool) -> Self {
        self.use_underline = Some(use_underline);
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn can_target(mut self, can_target: bool) -> Self {
        self.can_target = Some(can_target);
        self
    }

    pub fn css_name(mut self, css_name: &str) -> Self {
        self.css_name = Some(css_name.to_string());
        self
    }

    pub fn cursor(mut self, cursor: &gdk::Cursor) -> Self {
        self.cursor = Some(cursor.clone());
        self
    }

    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = Some(expand);
        self
    }

    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_focus(mut self, has_focus: bool) -> Self {
        self.has_focus = Some(has_focus);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn is_focus(mut self, is_focus: bool) -> Self {
        self.is_focus = Some(is_focus);
        self
    }

    pub fn layout_manager<P: IsA<LayoutManager>>(mut self, layout_manager: &P) -> Self {
        self.layout_manager = Some(layout_manager.clone().upcast());
        self
    }

    pub fn margin(mut self, margin: i32) -> Self {
        self.margin = Some(margin);
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = Some(overflow);
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }
}

pub const NONE_EXPANDER: Option<&Expander> = None;

pub trait ExpanderExt: 'static {
    fn get_expanded(&self) -> bool;

    fn get_label(&self) -> Option<GString>;

    fn get_label_widget(&self) -> Option<Widget>;

    fn get_resize_toplevel(&self) -> bool;

    fn get_use_markup(&self) -> bool;

    fn get_use_underline(&self) -> bool;

    fn set_expanded(&self, expanded: bool);

    fn set_label(&self, label: Option<&str>);

    fn set_label_widget<P: IsA<Widget>>(&self, label_widget: Option<&P>);

    fn set_resize_toplevel(&self, resize_toplevel: bool);

    fn set_use_markup(&self, use_markup: bool);

    fn set_use_underline(&self, use_underline: bool);

    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_activate(&self);

    fn connect_property_expanded_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_label_widget_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_resize_toplevel_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_use_markup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_use_underline_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<Expander>> ExpanderExt for O {
    fn get_expanded(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_expander_get_expanded(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_label(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_expander_get_label(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_label_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_expander_get_label_widget(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_resize_toplevel(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_expander_get_resize_toplevel(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_use_markup(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_expander_get_use_markup(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_use_underline(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_expander_get_use_underline(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_expanded(&self, expanded: bool) {
        unsafe {
            gtk_sys::gtk_expander_set_expanded(self.as_ref().to_glib_none().0, expanded.to_glib());
        }
    }

    fn set_label(&self, label: Option<&str>) {
        unsafe {
            gtk_sys::gtk_expander_set_label(self.as_ref().to_glib_none().0, label.to_glib_none().0);
        }
    }

    fn set_label_widget<P: IsA<Widget>>(&self, label_widget: Option<&P>) {
        unsafe {
            gtk_sys::gtk_expander_set_label_widget(
                self.as_ref().to_glib_none().0,
                label_widget.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_resize_toplevel(&self, resize_toplevel: bool) {
        unsafe {
            gtk_sys::gtk_expander_set_resize_toplevel(
                self.as_ref().to_glib_none().0,
                resize_toplevel.to_glib(),
            );
        }
    }

    fn set_use_markup(&self, use_markup: bool) {
        unsafe {
            gtk_sys::gtk_expander_set_use_markup(
                self.as_ref().to_glib_none().0,
                use_markup.to_glib(),
            );
        }
    }

    fn set_use_underline(&self, use_underline: bool) {
        unsafe {
            gtk_sys::gtk_expander_set_use_underline(
                self.as_ref().to_glib_none().0,
                use_underline.to_glib(),
            );
        }
    }

    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn activate_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkExpander,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Expander>,
        {
            let f: &F = &*(f as *const F);
            f(&Expander::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"activate\0".as_ptr() as *const _,
                Some(transmute(activate_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_activate(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject)
                .emit("activate", &[])
                .unwrap()
        };
    }

    fn connect_property_expanded_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_expanded_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkExpander,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Expander>,
        {
            let f: &F = &*(f as *const F);
            f(&Expander::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::expanded\0".as_ptr() as *const _,
                Some(transmute(notify_expanded_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_label_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkExpander,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Expander>,
        {
            let f: &F = &*(f as *const F);
            f(&Expander::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::label\0".as_ptr() as *const _,
                Some(transmute(notify_label_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_label_widget_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_label_widget_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkExpander,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Expander>,
        {
            let f: &F = &*(f as *const F);
            f(&Expander::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::label-widget\0".as_ptr() as *const _,
                Some(transmute(
                    notify_label_widget_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_resize_toplevel_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_resize_toplevel_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkExpander,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Expander>,
        {
            let f: &F = &*(f as *const F);
            f(&Expander::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::resize-toplevel\0".as_ptr() as *const _,
                Some(transmute(
                    notify_resize_toplevel_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_use_markup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_markup_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkExpander,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Expander>,
        {
            let f: &F = &*(f as *const F);
            f(&Expander::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::use-markup\0".as_ptr() as *const _,
                Some(transmute(notify_use_markup_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_use_underline_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_underline_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkExpander,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Expander>,
        {
            let f: &F = &*(f as *const F);
            f(&Expander::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::use-underline\0".as_ptr() as *const _,
                Some(transmute(
                    notify_use_underline_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Expander {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Expander")
    }
}
