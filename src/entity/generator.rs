pub fn create_bouncer_generator<'a>(
    pos: ::na::Vector3<f32>,
    generators: &mut ::specs::WriteStorage<'a, ::component::Generator>,
    entities: &::specs::Entities,
) {
    let entity = entities.create();
    generators.insert(entity, ::component::Generator {
        pos,
        entity: ::component::GeneratedEntity::Bouncer,
        salvo: 1,
        timer: 0.0,
        time_between_salvo: 1.0,
        black_probability: 0.0,
    });
}

pub fn create_avoider_generator<'a>(
    pos: ::na::Vector3<f32>,
    generators: &mut ::specs::WriteStorage<'a, ::component::Generator>,
    entities: &::specs::Entities,
) {
    let entity = entities.create();
    generators.insert(entity, ::component::Generator {
        pos,
        entity: ::component::GeneratedEntity::Avoider,
        salvo: 4,
        timer: 0.0,
        time_between_salvo: 1.0,
        black_probability: 0.0,
    });
}
