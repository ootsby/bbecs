use criterion::{black_box, criterion_group, criterion_main, Criterion};

use bbecs::data_types::point::Point;
use bbecs::world::{World, WorldMethods};
use eyre::Result;

fn query_for_entities_mixed_components(num_entities: u32, num_queries: u32) -> Result<()> {
    let mut world = World::new();
    let location_name = "location";
    let size_name = "size";
    let first_location = Point::new(0.0, 0.0);
    let second_location = Point::new(10.0, 10.0);
    let third_location = Point::new(15.0, 15.0);
    let size = 15_u32;
    let third_size = 30_u32;
    let mut expected_result_count = 0;

    world.register(location_name)?;
    world.register(size_name)?;

    for i in 0..num_entities {
        match i % 3 {
            0 => {
                expected_result_count += 1;
                let _ = world
                    .spawn_entity()?
                    .with_component(location_name, first_location)?
                    .with_component(size_name, size)?;
            }

            1 => {
                let _ = world
                    .spawn_entity()?
                    .with_component(location_name, second_location)?;
            }

            2 => {
                expected_result_count += 1;
                let _ = world
                    .spawn_entity()?
                    .with_component(location_name, third_location)?
                    .with_component(size_name, third_size)?;
            }

            _ => unreachable!(),
        };
    }

    for _ in 0..num_queries {
        let components = world.query(vec![location_name, size_name])?;
        let locations = components.get(location_name).unwrap();
        let sizes = components.get(size_name).unwrap();

        assert_eq!(locations.len(), sizes.len());
        assert_eq!(locations.len(), expected_result_count);
        // let wrapped_queried_first_location: &Rc<RefCell<Point>> = locations[0].cast()?;
        // let queried_first_location = wrapped_queried_first_location.borrow();
        // assert_eq!(*queried_first_location, first_location);
        // let wrapped_queried_second_location: &Rc<RefCell<Point>> = locations[1].cast()?;
        // let queried_second_location = wrapped_queried_second_location.borrow();
        // assert_eq!(*queried_second_location, third_location);
    }

    Ok(())
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("mixed components - 10,000 entities, 1000 queries", |b| {
        b.iter(|| query_for_entities_mixed_components(black_box(10000), black_box(1000)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
