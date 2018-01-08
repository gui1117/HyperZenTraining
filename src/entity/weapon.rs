use alga::general::SubsetOf;
use std::f32::consts::{FRAC_PI_2, FRAC_PI_3};

pub fn create_light_ray<'a>(
    from: ::na::Vector3<f32>,
    to: ::na::Vector3<f32>,
    radius: f32,
    delet_timers: &mut ::specs::WriteStorage<'a, ::component::DeletTimer>,
    dynamic_draws: &mut ::specs::WriteStorage<'a, ::component::DynamicDraw>,
    dynamic_graphics_assets: &mut ::specs::WriteStorage<'a, ::component::DynamicGraphicsAssets>,
    entities: &::specs::Entities,
) {
    let (primitive, groups) = ::graphics::Primitive::Cylinder.instantiate();
    let color = ::graphics::Color::Yellow;
    let primitive_trans = {
        let i = ::na::Translation::from_vector((from + to) / 2.0)
            * ::na::Rotation3::rotation_between(&::na::Vector3::new(1.0, 0.0, 0.0), &(to - from))
                .unwrap();

        let r = ::na::Rotation3::new(::na::Vector3::new(0.0, -FRAC_PI_2, 0.0));

        i * r * ::graphics::resizer(radius, radius, (to - from).norm() / 2.0)
    };

    let entity = entities.create();
    dynamic_draws.insert(entity, ::component::DynamicDraw);
    dynamic_graphics_assets.insert(
        entity,
        ::component::DynamicGraphicsAssets::new(primitive, groups, color, primitive_trans),
    );
    delet_timers.insert(entity, ::component::DeletTimer::new(0.03));
}

pub fn create_weapon<'a>(
    anchor: ::specs::Entity,
    shooters: &mut ::specs::WriteStorage<'a, ::component::Shooter>,
    weapon_animations: &mut ::specs::WriteStorage<'a, ::component::WeaponAnimation>,
    weapon_anchors: &mut ::specs::WriteStorage<'a, ::component::WeaponAnchor>,
    dynamic_huds: &mut ::specs::WriteStorage<'a, ::component::DynamicHud>,
    dynamic_graphics_assets: &mut ::specs::WriteStorage<'a, ::component::DynamicGraphicsAssets>,
    entities: &::specs::Entities,
) {
    let coef = 3.0;
    let shoot_pos_x = 0.08 * coef;
    let weapon_pos_y = -0.02 * coef;
    let weapon_pos_z = -0.016 * coef;

    let center_radius = 0.0036 * coef;
    let light_ray_radius = 0.002 * coef;

    let six_radius = 0.0056 * coef;
    let six_length = 0.051 * coef;

    let bar_x_pos = 0.071 * coef;
    let bar_x_radius = 0.04 * coef;
    let bar_y_radius = 0.0022 * coef;
    let bar_z_radius = 0.0014 * coef;

    let bullet_radius = ::CONFIG.weapon_bullet_radius * coef;
    let bullet_length = ::CONFIG.weapon_bullet_length * coef;
    let bullet_x = ::CONFIG.weapon_bullet_x * coef;
    let bullet_dx = ::CONFIG.weapon_bullet_dx * coef;
    let bullet_nbr = ::CONFIG.weapon_bullet_nbr;
    let mut bullets = vec![];

    let weapon_trans = ::na::Translation3::new(0.0, weapon_pos_y, weapon_pos_z);

    // Six
    let (primitive, groups) = ::graphics::Primitive::Six.instantiate();
    let primitive_trans = weapon_trans
        * ::na::Rotation3::new(::na::Vector3::new(0.0, FRAC_PI_2, 0.0))
        * ::graphics::resizer(six_radius, six_radius, six_length);

    let entity = entities.create();
    weapon_anchors.insert(entity, ::component::WeaponAnchor { anchor: anchor });
    dynamic_huds.insert(entity, ::component::DynamicHud);
    dynamic_graphics_assets.insert(
        entity,
        ::component::DynamicGraphicsAssets::new(
            primitive,
            groups,
            ::CONFIG.weapon_six_color,
            primitive_trans,
        ),
    );

    // Bullet
    for i in 0..bullet_nbr {
        let (primitive, groups) = ::graphics::Primitive::Six.instantiate();
        let primitive_trans = weapon_trans
            * ::na::Isometry3::new(
                ::na::Vector3::new(bullet_x + bullet_dx * i as f32, 0.0, 0.0),
                ::na::Vector3::new(0.0, FRAC_PI_2, 0.0),
            )
            * ::graphics::resizer(bullet_radius, bullet_radius, bullet_length);

        let entity = entities.create();
        bullets.push(entity);
        weapon_anchors.insert(entity, ::component::WeaponAnchor { anchor: anchor });
        dynamic_huds.insert(entity, ::component::DynamicHud);
        dynamic_graphics_assets.insert(
            entity,
            ::component::DynamicGraphicsAssets::new(
                primitive,
                groups,
                ::CONFIG.weapon_bullet_color,
                primitive_trans,
            ),
        );
    }
    bullets.reverse();

    for angle in (0..3usize).map(|i| i as f32 * 2.0 * FRAC_PI_3) {
        // Bar
        let (primitive, groups) = ::graphics::Primitive::Cube.instantiate();
        let primitive_trans = weapon_trans
            * ::na::Isometry3::new(
                ::na::Vector3::new(
                    bar_x_pos,
                    (center_radius + bar_y_radius) * angle.cos(),
                    (center_radius + bar_y_radius) * angle.sin(),
                ),
                ::na::Vector3::new(angle, 0.0, 0.0),
            )
            * ::graphics::resizer(bar_x_radius, bar_y_radius, bar_z_radius);

        let entity = entities.create();
        weapon_anchors.insert(entity, ::component::WeaponAnchor { anchor: anchor });
        dynamic_huds.insert(entity, ::component::DynamicHud);
        dynamic_graphics_assets.insert(
            entity,
            ::component::DynamicGraphicsAssets::new(
                primitive,
                groups,
                ::CONFIG.weapon_angle_color,
                primitive_trans,
            ),
        );
    }

    weapon_animations.insert(
        anchor,
        ::component::WeaponAnimation {
            weapon_trans: weapon_trans.to_superset(),
            shoot_pos: ::na::Point3::new(shoot_pos_x, 0.0, 0.0),
            light_ray_radius,
            bullets,
        },
    );
    shooters.insert(
        anchor,
        ::component::Shooter::new(::CONFIG.weapon_reload_time, bullet_nbr),
    );
}
