use eframe::egui;
use rand::Rng;

struct MyApp {
    target_number: u32,
    user_guess: String,
    message: String,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // 卡片式容器
            egui::Frame::group(ui.style())
                .inner_margin(egui::Margin::symmetric(30.0, 20.0))
                .rounding(10.0)
                .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
                .show(ui, |ui| {

                    // 游戏标题
                    ui.heading(
                        egui::RichText::new("🔢 猜数游戏")
                            .color(egui::Color32::from_rgb(25, 118, 210))
                            .size(24.0)
                    );

                    // 输入区域
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("输入数字 (1-100):")
                                .color(egui::Color32::DARK_GRAY)
                                .text_style(egui::TextStyle::Button)
                        );
                        ui.text_edit_singleline(&mut self.user_guess)
                            .min_size(egui::Vec2::new(150.0, 0.0));
                    });

                    // 操作按钮
                    ui.horizontal(|ui| {
                        // 提交按钮
                        let submit_btn = egui::Button::new(
                            egui::RichText::new("🚀 提交")
                                .color(egui::Color32::WHITE)
                        )
                            .fill(egui::Color32::from_rgb(76, 175, 80))
                            .min_size(egui::Vec2::new(100.0, 30.0));

                        if ui.add(submit_btn).clicked() {
                            match self.user_guess.parse::<u32>() {
                                Ok(guess) => {
                                    if guess > 100 || guess < 1 {
                                        self.message = "请输入1-100之间的数字！".to_string();
                                    } else if guess > self.target_number {
                                        self.message = "太大了！".to_string();
                                    } else if guess < self.target_number {
                                        self.message = "太小了！".to_string();
                                    } else {
                                        self.message = "🎉 恭喜猜对了！".to_string();
                                    }
                                }
                                Err(_) => {
                                    self.message = "⚠️ 请输入有效数字".to_string();
                                }
                            }
                        }

                        // 重置按钮
                        let reset_btn = egui::Button::new(
                            egui::RichText::new("🔄 重置")
                                .color(egui::Color32::WHITE)
                        )
                            .fill(egui::Color32::from_rgb(239, 83, 80))
                            .min_size(egui::Vec2::new(100.0, 30.0));

                        if ui.add(reset_btn).clicked() {
                            self.target_number = rand::thread_rng().gen_range(1..=100);
                            self.user_guess.clear();
                            self.message.clear();
                        }
                    });

                    // 反馈信息
                    ui.add_space(10.0);
                    ui.label(
                        egui::RichText::new(&self.message)
                            .color(match self.message.as_str() {
                                "🎉 恭喜猜对了！" => egui::Color32::GREEN,
                                "⚠️ 请输入有效数字" => egui::Color32::RED,
                                _ => egui::Color32::DARK_GRAY,
                            })
                            .text_style(egui::TextStyle::Body)
                            .strong()
                    );
                });
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "猜数游戏",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp {
            target_number: rand::thread_rng().gen_range(1..=100),
            user_guess: String::new(),
            message: String::new(),
        }))),
    ).unwrap();
}
