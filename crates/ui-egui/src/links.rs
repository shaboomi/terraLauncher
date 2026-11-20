//! Link cards under the apps: web apps and products ([`catalog::WEB_APPS`], [`catalog::PRODUCTS`]).
//! A link card opens its page in the browser. Nothing is installed, checked or updated.

use artcraft_launcher_core::catalog::{Link, PRODUCTS, WEB_APPS};
use egui::{Color32, CornerRadius, FontId, Rect, RichText, Sense, Ui, vec2};

use crate::app::LauncherUi;
use crate::theme::{self, Tokens};
use crate::widgets::{self, Kind};

const CARD_MIN_W: f32 = 320.0;
const CARD_H: f32 = 132.0;
const PAD: f32 = 20.0;
const TILE: f32 = 52.0;

impl LauncherUi {
    /// The "Web apps" and "Products" sections, each a grid of link cards. `query` is the search
    /// box's text: a section shows the links that match it, and is left out when none do.
    pub(crate) fn link_sections(&mut self, ui: &mut Ui, query: &str) {
        let wipe = if self.reduce_motion { 0.0 } else { 0.3 };
        section(ui, tl!("Web apps"), &WEB_APPS, tl!("Open"), query, wipe);
        section(ui, tl!("Products"), &PRODUCTS, tl!("View"), query, wipe);
    }
}

fn matches(link: &Link, query: &str) -> bool {
    let query = query.trim().to_lowercase();
    query.is_empty() || link.name.to_lowercase().contains(&query) || link.tagline.to_lowercase().contains(&query)
}

fn section(ui: &mut Ui, title: &str, links: &[Link], action: &str, query: &str, wipe: f32) {
    let shown: Vec<&Link> = links.iter().filter(|l| matches(l, query)).collect();
    if shown.is_empty() {
        return;
    }
    let t = Tokens::get(ui.ctx());
    ui.add_space(12.0);
    widgets::hud(ui, title, t.faint);
    ui.add_space(12.0);
    let width = ui.available_width();
    // Neighbouring cards share their 1 px hairline, as the app cards do.
    let cols = ((width / CARD_MIN_W).floor() as usize).max(1);
    let card_w = (width + (cols as f32 - 1.0)) / cols as f32;
    let rows = shown.len().div_ceil(cols);
    let height = rows as f32 * (CARD_H - 1.0) + 1.0;
    let (area, _) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
    for (i, link) in shown.iter().enumerate() {
        let (row, col) = (i / cols, i % cols);
        let min = area.min + vec2(col as f32 * (card_w - 1.0), row as f32 * (CARD_H - 1.0));
        card(ui, link, Rect::from_min_size(min, vec2(card_w, CARD_H)), action, wipe);
    }
    ui.add_space(16.0);
}

/// The host of an `https://` URL, for the card's small print (`terrapen.xyz`).
fn host(url: &str) -> &str {
    url.strip_prefix("https://").unwrap_or(url).split(['/', '?', '#']).next().unwrap_or_default()
}

/// A link's name set like an app's: "terra" in the heading colour, the rest in the link's own.
fn name(name: &str, terra: Color32, rest: Color32) -> egui::text::LayoutJob {
    let (first, second) = name.strip_prefix("terra").map_or((name, ""), |second| ("terra", second));
    let face = |color| egui::TextFormat { font_id: FontId::new(20.0, theme::display()), color, extra_letter_spacing: -0.4, ..Default::default() };
    let mut job = egui::text::LayoutJob::default();
    job.append(first, 0.0, face(terra));
    job.append(second, 0.0, face(rest));
    job
}

fn card(ui: &mut Ui, link: &Link, rect: Rect, action: &str, wipe: f32) {
    let t = Tokens::get(ui.ctx());
    let color = theme::rgb(link.color);
    let ink = theme::rgb(if t.dark { link.ink_dark } else { link.ink_light });
    // The whole card opens the page. Added before the button, so the button sits on top of it.
    let open = ui.interact(rect, ui.scope_id().with(("link-card", link.id)), Sense::click());
    open.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Link, true, link.name));
    let lift = ui.ctx().animate_bool_with_time(ui.scope_id().with(("link-hover", link.id)), ui.rect_contains_pointer(rect), wipe);
    let painter = ui.painter_at(rect);
    painter.rect(rect, CornerRadius::ZERO, t.bg.lerp_to_gamma(color, 0.05 * lift), t.hairline(), egui::StrokeKind::Inside);
    if lift > 0.0 {
        let inside = rect.shrink(1.0);
        painter.rect_filled(Rect::from_min_size(inside.min, vec2(inside.width() * lift, 2.0)), CornerRadius::ZERO, color);
    }
    let mut clicked = open.on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_text(link.url).clicked();

    let inner = rect.shrink(PAD);
    let mut child = ui.new_child(egui::UiBuilder::new().max_rect(inner).layout(egui::Layout::left_to_right(egui::Align::Center)));
    child.set_clip_rect(rect.intersect(ui.clip_rect()));
    let ui = &mut child;
    ui.spacing_mut().item_spacing.x = 14.0;
    // The terraPen mark, where an app card has the app's icon.
    let (tile, _) = ui.allocate_exact_size(vec2(TILE, TILE), Sense::hover());
    egui::Image::new(crate::icons::terrapen()).corner_radius(CornerRadius::same(12)).paint_at(ui, tile);
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        clicked |= widgets::button(ui, action, Kind::Secondary, 0.0).on_hover_text(link.url).clicked();
        ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
            ui.spacing_mut().item_spacing.y = 4.0;
            // Centre the three lines on the tile.
            ui.add_space(((ui.available_height() - 80.0) / 2.0).max(0.0));
            ui.add(egui::Label::new(name(link.name, t.ink_strong, ink)).selectable(false).truncate());
            let tagline = crate::i18n::tr_ctx(crate::i18n::current(), link.id, link.tagline);
            ui.add(egui::Label::new(RichText::new(tagline).size(13.5).color(t.muted)).wrap());
            widgets::hud(ui, &format!("{} ↗", host(link.url)), t.faint);
        });
    });
    if clicked {
        widgets::open_external_link(ui.ctx(), link.url);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_card_names_its_host_and_colours_the_second_word() {
        assert_eq!(host("https://plotterlens.terrapen.xyz/"), "plotterlens.terrapen.xyz");
        assert_eq!(host("https://terrapen.xyz/products/terradial"), "terrapen.xyz");
        let job = name("terraLens", Color32::WHITE, Color32::RED);
        assert_eq!(job.text, "terraLens");
        assert_eq!(job.sections.iter().map(|s| s.format.color).collect::<Vec<_>>(), [Color32::WHITE, Color32::RED]);
    }

    #[test]
    fn search_finds_links_by_name_or_tagline() {
        let lens = &WEB_APPS[0];
        assert!(matches(lens, "") && matches(lens, " LENS ") && matches(lens, "camera"));
        assert!(!matches(lens, "forge"));
    }
}
