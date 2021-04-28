use criterion::{black_box, criterion_group, criterion_main, Criterion};

use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::world::{World, WorldMethods, ENTITY_ID};
use eyre::Result;

const LOCATION_NAME: &str = "location";
const SIZE_NAME: &str = "size";

fn get_populated_world(num_entities: u32) -> Result<World> {
    let mut world = World::new();
    let first_location = Point::new(0.0, 0.0);
    let second_location = Point::new(10.0, 10.0);
    let third_location = Point::new(15.0, 15.0);
    let size = 15_u32;
    let third_size = 30_u32;
    //let mut expected_result_count = 0;

    world.register(LOCATION_NAME)?;
    world.register(SIZE_NAME)?;

    for i in 0..num_entities {
        match i % 3 {
            0 => {
                //expected_result_count += 1;
                let _ = world
                    .spawn_entity()?
                    .with_component(LOCATION_NAME, first_location)?
                    .with_component(SIZE_NAME, size)?;
            }

            1 => {
                let _ = world
                    .spawn_entity()?
                    .with_component(LOCATION_NAME, second_location)?;
            }

            2 => {
                //expected_result_count += 1;
                let _ = world
                    .spawn_entity()?
                    .with_component(LOCATION_NAME, third_location)?
                    .with_component(SIZE_NAME, third_size)?;
            }

            _ => unreachable!(),
        };
    }

    Ok(world)
}

fn query_for_entities_mixed_components(num_entities: u32, num_queries: u32) -> Result<()> {
    let world = get_populated_world(num_entities).unwrap();

    for _ in 0..num_queries {
        let components = world.query(vec![LOCATION_NAME, SIZE_NAME])?;
        let locations = components.get(LOCATION_NAME).unwrap();
        let sizes = components.get(SIZE_NAME).unwrap();

        assert_eq!(locations.len(), sizes.len());
        //assert_eq!(locations.len(), expected_result_count);
        // let wrapped_queried_first_location: &Rc<RefCell<Point>> = locations[0].cast()?;
        // let queried_first_location = wrapped_queried_first_location.borrow();
        // assert_eq!(*queried_first_location, first_location);
        // let wrapped_queried_second_location: &Rc<RefCell<Point>> = locations[1].cast()?;
        // let queried_second_location = wrapped_queried_second_location.borrow();
        // assert_eq!(*queried_second_location, third_location);
    }

    Ok(())
}
fn inserting_deleting(num_entities: u32, num_cycles: u32) -> Result<()> {
    let mut world = get_populated_world(num_entities).unwrap();
    let location = Point::new(0.0, 0.0);
    let size = 15_u32;
    
    for _ in 0..num_cycles {
        let ids: Vec<u32>;
        {
            let query = world.query(vec![SIZE_NAME, ENTITY_ID])?;

            ids = query
                .get(ENTITY_ID)
                .unwrap()
                .iter()
                .map(|&x| *x.cast().unwrap().borrow())
                .collect();
        }
        for &id in ids.iter() {
            world.delete_by_id(std::convert::TryInto::try_into(id).unwrap())?;
        }

        world.update()?;

        for _ in 0..ids.len(){
            let _ = world
                    .spawn_entity()?
                    .with_component(LOCATION_NAME, location)?
                    .with_component(SIZE_NAME, size)?;
            
        }
        

        // let query = world.query(vec!["size", ENTITY_ID])?;
        // let wrapped_size: &DataWrapper<f32> = query.get("size").unwrap()[0].cast()?;
        // let wrapped_id: &DataWrapper<usize> = query.get(ENTITY_ID).unwrap()[0].cast()?;

        // let test_id = *wrapped_id.borrow();
        // let test_size = *wrapped_size.borrow();
        // assert_eq!(0, test_id);
        // assert_eq!(test_size, 30.0_f32);
    }
    Ok(())
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("mixed components - 1,000 entities, 1,000 queries", |b| {
        b.iter(|| query_for_entities_mixed_components(black_box(1000), black_box(1000)))
    });
    c.bench_function("del insert - 1,000 entities, 100 cycles", |b| {
        b.iter(|| inserting_deleting(black_box(1000), black_box(100)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
