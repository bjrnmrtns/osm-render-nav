use lyon::math::point;
use lyon::path::Path;
use lyon::tessellation::*;

use osmpbf::*;

#[derive(Copy, Clone, Debug)]
struct MyVertex { position: [f32; 2] }

fn main() {
    lyon();
    osmpbf();
}

fn lyon() {
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


fn osmpbf() {
    let path = std::path::Path::new("map.osm.pbf");
    let reader = ElementReader::from_path(path).unwrap();

    println!("Counting...");

    match reader.par_map_reduce(
        |element| match element {
            Element::Node(_) | Element::DenseNode(_) => (1, 0, 0),
            Element::Way(_) => (0, 1, 0),
            Element::Relation(_) => (0, 0, 1),
        },
        || (0u64, 0u64, 0u64),
        |a, b| (a.0 + b.0, a.1 + b.1, a.2 + b.2),
    ) {
        Ok((nodes, ways, relations)) => {
            println!("Nodes: {}", nodes);
            println!("Ways: {}", ways);
            println!("Relations: {}", relations);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}