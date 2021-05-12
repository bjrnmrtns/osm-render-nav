use lyon::math::point;
use lyon::path::Path;
use lyon::tessellation::*;

#[derive(Copy, Clone, Debug)]
struct MyVertex { position: [f32; 2] }

fn main() {
    // Build a Path.
    let mut builder = Path::builder();
    builder.begin(point(0.0, 0.0));
    builder.line_to(point(1.0, 0.0));
    builder.quadratic_bezier_to(point(2.0, 0.0), point(2.0, 1.0));
    builder.cubic_bezier_to(point(1.0, 1.0), point(0.0, 1.0), point(0.0, 0.0));
    builder.end(true);
    let path = builder.build();
    let mut geometry: VertexBuffers<MyVertex, u16> = VertexBuffers::new();
    let mut tessellator = FillTessellator::new();
    {
        tessellator.tessellate_path(
            &path,
            &FillOptions::default(),
            &mut BuffersBuilder::new(&mut geometry, |vertex: FillVertex| {
                MyVertex {
                    position: vertex.position().to_array(),
                }
            }),
        ).unwrap();
    }
    println!(" -- {} vertices {} indices",
             geometry.vertices.len(),
             geometry.indices.len()
    );
}