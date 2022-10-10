use eframe::{
    egui::{style::Margin, CentralPanel, Frame, Layout, RichText, TextEdit, TextFormat},
    emath::Align,
    epaint::{text::LayoutSection, Color32, FontId, Pos2, Rect, Stroke, Vec2},
};
use egui_extras::{Size, StripBuilder};

use super::{widgets::device_id_input_field::DeviceIDInputField, View};

#[derive(Default)]
pub struct ConnectPage {
    show_password: bool,
    input_device_id: super::widgets::device_id_input_field::DeviceIDInputText,
}

impl ConnectPage {}

impl View for ConnectPage {
    fn build(&mut self, ui: &mut eframe::egui::Ui) {
        Frame::default()
            .inner_margin(Margin::same(8.0))
            .show(ui, |ui| {
                StripBuilder::new(ui)
                    .size(Size::relative(0.09)) // Domain Title
                    .size(Size::relative(0.16)) // Domain
                    .size(Size::relative(0.16)) // Device ID Panel
                    .size(Size::relative(0.16)) // Password Panel
                    .size(Size::relative(0.12)) // Connect Remote Title
                    .size(Size::relative(0.13)) // Connect Device ID Panel
                    .size(Size::relative(0.18)) // Connect Button
                    .vertical(|mut strip| {
                        // Domain Title
                        strip.cell(|ui| {
                            ui.centered_and_justified(|ui| {
                                let label_rect = ui
                                    .label(RichText::new("Domain").font(FontId::proportional(28.0)))
                                    .rect;

                                let button_pos = Pos2::new(
                                    label_rect.right_center().x - 14.0,
                                    label_rect.right_center().y,
                                );

                                ui.allocate_ui_at_rect(
                                    Rect::from_center_size(button_pos, Vec2::new(20.0, 20.0)),
                                    |ui| {
                                        if ui.button("🔄").clicked() {}
                                    },
                                );
                            });
                        });

                        // Domain
                        strip.cell(|ui| {
                            ui.centered_and_justified(|ui| {
                                ui.label(
                                    RichText::new("MirrorX.cloud").font(FontId::proportional(40.0)),
                                );
                            });
                        });

                        // Device ID Panel
                        strip.cell(|ui| {
                            ui.centered_and_justified(|ui| {
                                ui.label(
                                    RichText::new("00-0000-0000").font(FontId::monospace(50.0)),
                                );
                            });
                        });

                        // Password Panel
                        strip.cell(|ui| {
                            ui.centered_and_justified(|ui| {
                                let password_label = ui.label(
                                    RichText::new(if self.show_password {
                                        "00-0000-0000"
                                    } else {
                                        "＊＊＊＊＊＊＊"
                                    })
                                    .font(FontId::proportional(50.0)),
                                );

                                // put the show password toggle button on the label right-top corner
                                let password_right_top_pos = password_label.rect.right_top();
                                let show_password_toggle_pos = Pos2::new(
                                    password_right_top_pos.x - 14.0,
                                    password_right_top_pos.y + 12.0,
                                );

                                ui.allocate_ui_at_rect(
                                    Rect::from_center_size(
                                        show_password_toggle_pos,
                                        Vec2::new(20.0, 20.0),
                                    ),
                                    |ui| {
                                        if !ui
                                            .toggle_value(
                                                &mut self.show_password,
                                                RichText::new("👁").font(FontId::proportional(18.0)),
                                            )
                                            .hovered()
                                        {
                                            self.show_password = false;
                                        };
                                    },
                                );
                            });
                        });

                        // Connect Remote Title
                        strip.cell(|ui| {
                            ui.painter().line_segment(
                                [
                                    ui.max_rect().left_top() + Vec2::new(2.0, 0.0),
                                    ui.max_rect().right_top() + Vec2::new(-2.0, 0.0),
                                ],
                                Stroke::new(1.0, Color32::GRAY),
                            );

                            ui.centered_and_justified(|ui| {
                                ui.label(
                                    RichText::new("Connect Remote")
                                        .font(FontId::proportional(28.0)),
                                );
                            });
                        });

                        // Connect Device ID Panel
                        strip.cell(|ui| {
                            ui.centered_and_justified(|ui| {
                                ui.add(DeviceIDInputField::text(&mut self.input_device_id));
                            });
                        });

                        // Connect Button
                        strip.strip(|strip| {
                            strip
                                .size(Size::relative(0.28))
                                .size(Size::relative(0.54))
                                .size(Size::relative(0.28))
                                .vertical(|mut strip| {
                                    strip.empty();
                                    strip.strip(|strip| {
                                        strip
                                            .size(Size::relative(0.15))
                                            .size(Size::relative(0.35))
                                            .size(Size::relative(0.35))
                                            .size(Size::relative(0.15))
                                            .horizontal(|mut strip| {
                                                strip.empty();
                                                strip.cell(|ui| {
                                                    ui.centered_and_justified(|ui| {
                                                        if ui.button("A").clicked() {}
                                                    });
                                                });
                                                strip.cell(|ui| {
                                                    ui.centered_and_justified(|ui| {
                                                        if ui.button("b").clicked() {}
                                                    });
                                                });
                                                strip.empty();
                                            });
                                    });
                                    strip.empty();
                                });
                        });
                    });
            });
    }
}
