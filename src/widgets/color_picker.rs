use egui_macroquad::egui::*;

use crate::utils::GameColor;

pub fn show_color_at(painter: &Painter, color: Color32, rect: Rect) {
    painter.rect_filled(rect, 0.0, color);
}

fn color_button(ui: &mut Ui, color: Color32, open: bool) -> Response {
    let size = ui.spacing().interact_size;
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());
    response.widget_info(|| WidgetInfo::new(WidgetType::ColorButton));

    if ui.is_rect_visible(rect) {
        let visuals = if open {
            &ui.visuals().widgets.open
        } else {
            ui.style().interact(&response)
        };
        let rect = rect.expand(visuals.expansion);

        show_color_at(ui.painter(), color, rect);

        let rounding = visuals.rounding.at_most(2.0);
        ui.painter()
            .rect_stroke(rect, rounding, (2.0, visuals.bg_fill)); // fill is intentional, because default style has no border
    }

    response
}

fn color_picker_palette(ui: &mut Ui, color: &mut GameColor, palette: &[GameColor]) -> bool {
    let id = ui.auto_id_with("cp");
    let mut close = false;
    Grid::new(id).show(ui, |ui| {
        for (index, gc) in palette.iter().enumerate() {
            if index != 0 && index % 6 == 0 {
                ui.end_row();
            }
            if color_button(ui, (*gc).into(), false).clicked() {
                *color = *gc;
                close = true;
            }
        }
        ui.end_row();
    });
    close
}

/// Shows a button that, when clicked, allows the user to select a color from a limited,
/// pre-defined, palette.
pub fn color_edit_palette_button(
    ui: &mut Ui,
    color: &mut GameColor,
    palette: &[GameColor],
) -> Response {
    let popup_id = ui.auto_id_with("popup");
    let open = ui.memory(|mem| mem.is_popup_open(popup_id));
    let btn_response = color_button(ui, (*color).into(), open);

    if btn_response.clicked() {
        ui.memory_mut(|mem| mem.toggle_popup(popup_id));
    }

    let mut close = false;
    if ui.memory(|mem| mem.is_popup_open(popup_id)) {
        let _area_response = Area::new(popup_id)
            .order(Order::Foreground)
            .fixed_pos(btn_response.rect.max)
            .constrain(true)
            .show(ui.ctx(), |ui| {
                ui.spacing_mut().slider_width = btn_response.rect.width() * 8.;
                Frame::popup(ui.style()).show(ui, |ui| {
                    close = color_picker_palette(ui, color, palette);
                });
            })
            .response;
    }

    if close {
        ui.memory_mut(|mem| mem.toggle_popup(popup_id));
    }

    btn_response
}
