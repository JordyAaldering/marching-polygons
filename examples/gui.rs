use eframe::egui::*;

const N: usize = 7;

const ANGLE: f32 = std::f32::consts::TAU / N as f32;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();

    let mut state = [false; N];

    eframe::run_simple_native("example", options, move |ctx, _frame| {
        CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                for s in &mut state {
                    ui.checkbox(s, "");
                }
            });

            let mut mask = state[0] as u8;
            for s in &state[1..] {
                mask <<= 1;
                mask |= *s as u8;
            }
            ui.label(format!("0b{:0>N$b}", mask));

            let (response, painter) = ui.allocate_painter(ui.available_size(), Sense::hover());

            let to_screen = emath::RectTransform::from_to(Rect { min: Pos2::new(-1.0, -1.0), max: Pos2::new(1.0, 1.0) }, response.rect);

            let vertices: Vec<(f32, f32)> = (0..N)
                .map(|i|
                    (to_screen * Pos2::new(
                        f32::cos(i as f32 * ANGLE - 90.0),
                        f32::sin(i as f32 * ANGLE - 90.0)
                    )).into()
                ).collect();

            let mut mesh = Mesh::default();
            for p in &vertices {
                mesh.colored_vertex(p.into(), Color32::BLUE);
            }
            for i in 0..(N as u32 - 2) {
                mesh.add_triangle(0, i + 1, i + 2);
            }
            painter.add(Shape::mesh(mesh));

            let mask = marching_shapes::Mask::<N>::new(mask);
            let mut mesh = Mesh::default();
            let mut i = 0;

            for t in mask.march().iter() {
                let [p0, p1, p2] = t.map(&vertices);
                mesh.colored_vertex(p0.into(), Color32::ORANGE);
                mesh.colored_vertex(p1.into(), Color32::ORANGE);
                mesh.colored_vertex(p2.into(), Color32::ORANGE);
                mesh.add_triangle(i, i + 1, i + 2);
                i += 3;
            }

            painter.add(Shape::mesh(mesh));

            for t in mask.march().iter() {
                let [p0, p1, p2] = t.map(&vertices);
                painter.add(epaint::Shape::line_segment([p0.into(), p1.into()], Stroke::new(3.0, Color32::RED)));
                painter.add(epaint::Shape::line_segment([p1.into(), p2.into()], Stroke::new(3.0, Color32::RED)));
                painter.add(epaint::Shape::line_segment([p2.into(), p0.into()], Stroke::new(3.0, Color32::RED)));
            }
        });
    })
}
