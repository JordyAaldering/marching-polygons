use eframe::egui::*;

const N: u8 = 7;

const ANGLE: f32 = std::f32::consts::TAU / N as f32;

fn main() -> eframe::Result {
    let mut state = [false; N as usize];

    eframe::run_simple_native("example", Default::default(), move |ctx, _frame| {
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
            ui.label(format!("{:#010b}", mask));

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

            let mut mesh = Mesh::default();
            let mut i = 0;

            let indices = marching_polygons::march::<N>(mask);
            for [i0, i1, i2] in &indices {
                let v0 = i0.transform(&vertices);
                let v1 = i1.transform(&vertices);
                let v2 = i2.transform(&vertices);
                mesh.colored_vertex(v0.into(), Color32::ORANGE);
                mesh.colored_vertex(v1.into(), Color32::ORANGE);
                mesh.colored_vertex(v2.into(), Color32::ORANGE);
                mesh.add_triangle(i, i + 1, i + 2);
                i += 3;
            }

            painter.add(Shape::mesh(mesh));

            for [i0, i1, i2] in &indices {
                let v0 = i0.transform(&vertices);
                let v1 = i1.transform(&vertices);
                let v2 = i2.transform(&vertices);
                painter.add(epaint::Shape::line_segment([v0.into(), v1.into()], Stroke::new(3.0, Color32::RED)));
                painter.add(epaint::Shape::line_segment([v1.into(), v2.into()], Stroke::new(3.0, Color32::RED)));
                painter.add(epaint::Shape::line_segment([v2.into(), v0.into()], Stroke::new(3.0, Color32::RED)));
            }
        });
    })
}
