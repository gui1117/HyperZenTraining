pub fn create_bouncer_generator<'a>(
    pos: ::na::Vector3<f32>,
    generators: &mut ::specs::WriteStorage<'a, ::component::Generator>,
    config: &::specs::Fetch<'a, ::resource::Config>,
    entities: &::specs::Entities,
) {
    let entity = entities.create();
    generators.insert(entity, ::component::Generator {
        pos,
        entity: ::component::GeneratedEntity::Bouncer,
        salvo: config.bouncer_generator_salvo,
        timer: 0.0,
        time_between_salvo: config.bouncer_generator_time_between_salvo,
        black_probability: config.bouncer_generator_black_probability,
    });
}

pub fn create_avoider_generator<'a>(
    pos: ::na::Vector3<f32>,
    generators: &mut ::specs::WriteStorage<'a, ::component::Generator>,
    config: &::specs::Fetch<'a, ::resource::Config>,
    entities: &::specs::Entities,
) {
    let entity = entities.create();
    generators.insert(entity, ::component::Generator {
        pos,
        entity: ::component::GeneratedEntity::Avoider,
        salvo: config.avoider_generator_salvo,
        timer: 0.0,
        time_between_salvo: config.avoider_generator_time_between_salvo,
        black_probability: config.avoider_generator_black_probability,
    });
}
