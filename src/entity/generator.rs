pub fn create_generator<'a>(
    pos: ::na::Vector3<f32>,
    generated_entity: ::component::GeneratedEntity,
    salvo: usize,
    time_between_salvo: f32,
    black_probability: f32,
    generators: &mut ::specs::WriteStorage<'a, ::component::Generator>,
    entities: &::specs::Entities,
) {
    let entity = entities.create();
    generators.insert(entity, ::component::Generator {
        pos,
        entity: generated_entity,
        salvo,
        timer: 0.0,
        time_between_salvo,
        black_probability,
    });
}
