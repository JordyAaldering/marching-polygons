use eframe::egui::*;
use marching_shapes::MarchPoint;

const N: usize = 8;

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

            let vertices: Vec<_> = (0..N).into_iter().map(|i| {
                let x = f32::cos(i as f32 * marching_shapes::Mask::<N>::ANGLE);
                let y = f32::sin(i as f32 * marching_shapes::Mask::<N>::ANGLE);
                (x, y)
            }).collect();
            let points_in_screen: Vec<Pos2> = vertices.iter().map(|p| to_screen * Pos2::from(*p)).collect();

            let mut mesh = Mesh::default();
            for p in &points_in_screen {
                mesh.colored_vertex(*p, Color32::BLUE);
            }
            for i in 0..(N as u32 - 2) {
                mesh.add_triangle(0, i + 1, i + 2);
            }
            painter.add(Shape::mesh(mesh));

            let mask = marching_shapes::Mask::<N>::new(mask);
            let mut mesh = Mesh::default();
            let mut i = 0;

            for [t0, t1, t2] in mask.march() {
                let p0 = get_point(t0, &points_in_screen);
                let p1 = get_point(t1, &points_in_screen);
                let p2 = get_point(t2, &points_in_screen);
                mesh.colored_vertex(p0, Color32::ORANGE);
                mesh.colored_vertex(p1, Color32::ORANGE);
                mesh.colored_vertex(p2, Color32::ORANGE);
                mesh.add_triangle(i, i + 1, i + 2);
                i += 3;
            }

            painter.add(Shape::mesh(mesh));

            for [t0, t1, t2] in mask.march() {
                let p0 = get_point(t0, &points_in_screen);
                let p1 = get_point(t1, &points_in_screen);
                let p2 = get_point(t2, &points_in_screen);
                painter.add(epaint::Shape::line_segment([p0, p1], Stroke::new(3.0, Color32::RED)));
                painter.add(epaint::Shape::line_segment([p1, p2], Stroke::new(3.0, Color32::RED)));
                painter.add(epaint::Shape::line_segment([p2, p0], Stroke::new(3.0, Color32::RED)));
            }
        });
    })
}

fn get_point(t: MarchPoint, vertices: &Vec<Pos2>) -> Pos2 {
    match t {
        MarchPoint::At(idx) => vertices[idx as usize],
        MarchPoint::Between(i, j) => {
            let a = vertices[i as usize];
            let b = Vec2::new(vertices[j as usize].x, vertices[j as usize].y);
            (a + b) / 2.0
        },
    }
}
