use marching_shapes::Mask;

fn main() {
    let mesh = Mask::<6>::new(0b010111);
    println!("{:?}", mesh.march());
}
