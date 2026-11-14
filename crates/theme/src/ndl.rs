//! The Nodalia Design Language on top of Torno's palettes: every interface surface, text, border,
//! control and accent comes from NDL's palettes with Torno's accent (`nodalia_theme`). What a
//! design means stays Torno's own: sketch, selection, workspace and analysis colours, the view
//! cube's axes, previews and the colour vision variants are not touched here.
//!
//! The dark palette is NDL's dark one lifted a few steps, as in Forja: NDL's near-black frame reads
//! too dark around a 3D view over a long session. The steps between the frame, the panels and the
//! raised surfaces stay NDL's. The 3D view has one flat colour, never a gradient (NDL has none).

use crate::Tokens;
use egui::Color32;
use nodalia_theme::{Accent, Mode, Palette, Rgba};

/// NDL's light palette with Torno's accent.
pub const LIGHT: Palette = Mode::Light.palette_with(Accent::Torno);
/// NDL's dark palette with Torno's accent, lifted ([`lifted_dark`]).
pub const DARK: Palette = lifted_dark(Mode::Dark.palette_with(Accent::Torno));

/// The palette of `dark` or light.
pub const fn palette(dark: bool) -> Palette {
    if dark { DARK } else { LIGHT }
}

const fn rgb(r: u8, g: u8, b: u8) -> Rgba {
    Rgba::new(r, g, b, 255)
}

/// NDL's dark palette with its surfaces, controls and borders lifted, the same values as Forja's.
const fn lifted_dark(mut p: Palette) -> Palette {
    p.surface_window = rgb(25, 27, 33);
    p.surface_panel = rgb(36, 39, 48);
    p.surface_raised = rgb(45, 49, 60);
    p.surface_sunken = rgb(30, 32, 40);
    p.surface_overlay = rgb(45, 49, 60);
    p.text_tertiary = rgb(152, 158, 177);
    p.border_subtle = rgb(52, 57, 70);
    p.border_default = rgb(82, 89, 108);
    p.control_bg = rgb(45, 49, 60);
    p.control_bg_hover = rgb(54, 59, 72);
    p.control_bg_pressed = rgb(65, 71, 87);
    p.control_bg_disabled = rgb(36, 39, 48);
    p.control_field_bg = rgb(30, 32, 40);
    p
}

/// An NDL colour as egui's, opaque.
pub const fn c(c: Rgba) -> Color32 {
    Color32::from_rgb(c.r, c.g, c.b)
}

/// An NDL colour at `alpha` (0-255).
const fn ca(c: Rgba, alpha: u8) -> Color32 {
    Color32::from_rgba_unmultiplied_const(c.r, c.g, c.b, alpha)
}

/// The 3D view's flat background and its grid, `[background, grid, major grid]`.
pub const fn viewport(dark: bool) -> [Color32; 3] {
    if dark {
        [Color32::from_rgb(43, 46, 56), Color32::from_rgb(56, 60, 72), Color32::from_rgb(71, 76, 90)]
    } else {
        [Color32::from_rgb(240, 241, 245), Color32::from_rgb(212, 215, 223), Color32::from_rgb(188, 192, 204)]
    }
}

/// `t` with NDL's interface colours for `dark` or light.
pub const fn apply(mut t: Tokens, dark: bool) -> Tokens {
    let p = palette(dark);
    // Hover washes over the bars: the text colour at a low alpha, so they work on any surface.
    let wash = if dark { 255 } else { 0 };

    // The window frame and its bars: the application bar is the frame itself, as in every
    // Nodalia app; panels and the ribbon are islands on it.
    t.app_bar = c(p.surface_window);
    t.app_bar_separator = c(p.border_default);
    t.app_bar_text = c(p.text_primary);
    t.app_bar_hover = Color32::from_rgba_unmultiplied_const(wash, wash, wash, 22);
    t.app_bar_hover_subtle = Color32::from_rgba_unmultiplied_const(wash, wash, wash, 14);
    t.app_bar_hover_button = Color32::from_rgba_unmultiplied_const(wash, wash, wash, 24);
    t.app_bar_hover_strong = Color32::from_rgba_unmultiplied_const(wash, wash, wash, 34);
    t.app_bar_tab_active = c(p.surface_raised);
    t.toolbar = c(p.surface_panel);
    t.panel = c(p.surface_panel);
    t.panel_header = c(p.surface_raised);
    t.border = c(p.border_subtle);
    t.timeline = c(p.surface_panel);
    t.timeline_item = c(p.control_bg);
    t.timeline_marker = c(p.text_tertiary);

    // Text, icons and the accent.
    t.text = c(p.text_primary);
    t.text_dim = c(p.text_secondary);
    t.icon = c(p.text_secondary);
    t.accent = c(p.accent_default);
    t.accent_soft = c(p.accent_subtle);
    t.on_accent = c(p.text_on_accent);
    t.chip = c(p.accent_default);
    t.hover = c(p.control_bg_hover);
    t.selected_ink = c(p.accent_text);
    t.selected_bg = c(p.selection_bg);
    t.error = c(p.status_danger_fg);
    t.warning = c(p.status_warning_fg);

    // Dialogs, forms and fields.
    t.dialog_bg = c(p.surface_overlay);
    t.dialog_body = c(p.surface_overlay);
    t.dialog_footer = c(p.surface_overlay);
    t.dialog_rule = c(p.border_subtle);
    t.dialog_border = c(p.border_subtle);
    t.dialog_label = c(p.text_secondary);
    t.control_fill = c(p.control_field_bg);
    t.control_border = c(p.border_default);
    t.field = c(p.control_field_bg);
    t.overlay = ca(p.surface_overlay, 240);
    t.form_ink = c(p.text_primary);
    t.form_dim = c(p.text_secondary);
    t.form_primary = c(p.accent_text);
    t.form_primary_fill = c(p.accent_default);
    t.form_error = c(p.status_danger_fg);
    t.form_error_fill = c(p.status_danger_bg);
    t.form_success = c(p.status_success_fg);
    t.form_success_fill = c(p.status_success_bg);

    // Preferences and the figures in them.
    t.pref_well = c(p.surface_sunken);
    t.pref_on_accent = c(p.text_on_accent);
    t.pref_ok = c(p.status_success_fg);
    t.pref_ok_fill = c(p.status_success_bg);
    t.pref_note = c(p.status_warning_fg);
    t.pref_note_fill = c(p.status_warning_bg);
    t.pref_disabled = c(p.text_disabled);
    t.figure_ground = c(p.surface_sunken);
    t.figure_card = c(p.surface_raised);
    t.figure_accent = c(p.accent_text);

    // Navigation bar, view cube and tips.
    t.nav_hover = c(p.control_bg_hover);
    t.nav_press = c(p.control_bg_pressed);
    t.nav_dim = c(p.text_tertiary);
    t.nav_viewport = c(p.surface_panel);
    t.nav_tip = c(p.surface_inverse);
    t.nav_tip_ink = c(p.text_inverse);
    t.nav_field = c(p.control_field_bg);
    t.cube_border = c(p.border_default);
    t.cube_text = c(p.text_primary);
    t.cube_hover = c(p.accent_subtle);
    t.glyph_bg = c(p.surface_raised);

    // The start page and file views.
    t.file_background = c(p.surface_window);
    t.file_surface = c(p.surface_panel);
    t.file_border = c(p.border_subtle);
    t.file_dim = c(p.text_secondary);
    t.file_rail = c(p.accent_default);
    t.file_rail_ink = c(p.text_on_accent);
    t.file_rail_hover = c(p.accent_hover);
    t.file_primary = c(p.accent_subtle);
    t.file_primary_ink = c(p.accent_text);
    t.file_primary_border = c(p.accent_default);
    t.file_recovery = c(p.status_warning_bg);
    t.file_recovery_ink = c(p.status_warning_fg);

    // The 3D view: one flat colour, its grid a few steps away.
    let [view, grid, major] = viewport(dark);
    t.viewport_top = view;
    t.viewport_bottom = view;
    t.grid = grid;
    t.grid_major = major;
    t.sketch_halo = view;
    // NDL's viewports need a conflict red (and, on white, a deeper gold for filled form controls)
    // that keeps them legible; these are the values the readability pass settles on, so a saved
    // theme resolves to the defaults.
    if dark {
        t.sketch_conflict = Color32::from_rgb(255, 154, 144);
    } else {
        t.sketch_conflict = Color32::from_rgb(161, 42, 34);
        t.form_primary_fill = Color32::from_rgb(111, 91, 19);
    }
    t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_interface_takes_ndls_colours_and_tornos_accent() {
        for dark in [false, true] {
            let t = if dark { Tokens::dark() } else { Tokens::light() };
            let p = palette(dark);
            assert_eq!(t.accent, c(p.accent_default));
            assert_eq!(t.panel, c(p.surface_panel));
            assert_eq!(t.text, c(p.text_primary));
            assert_eq!(t.app_bar, c(p.surface_window));
        }
        assert_eq!(c(Mode::Dark.palette_with(Accent::Torno).accent_default), Tokens::dark().accent);
    }

    /// NDL draws no gradients: the 3D view is one colour from top to bottom.
    #[test]
    fn the_view_has_no_gradient() {
        for t in [Tokens::light(), Tokens::dark()] {
            assert_eq!(t.viewport_top, t.viewport_bottom);
        }
    }

    /// The icon review page (`crates/look/icons/design/tools/review/review.css`) lays its icons
    /// on the app's own surfaces, so its colours are these tokens.
    #[test]
    fn the_icon_review_page_uses_the_apps_colours() {
        let css = include_str!("../../look/icons/design/tools/review/review.css");
        let hex = |c: Color32| format!("#{:02x}{:02x}{:02x}", c.r(), c.g(), c.b());
        let (l, d) = (Tokens::light(), Tokens::dark());
        for (block, values) in [
            (":root{", [("--bg", l.panel), ("--fg", l.text), ("--card", l.toolbar), ("--line", l.border), ("--mut", l.text_dim)]),
            ("body.dark{", [("--bg", d.viewport_top), ("--fg", d.text), ("--card", d.toolbar), ("--line", d.border), ("--mut", d.text_dim)]),
        ] {
            let line = css.lines().find(|l| l.starts_with(block)).unwrap_or_default();
            for (var, colour) in values {
                assert!(line.contains(&format!("{var}:{}", hex(colour))), "{block} {var} should be {}", hex(colour));
            }
        }
    }
}
