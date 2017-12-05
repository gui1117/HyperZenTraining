use vulkano::buffer::cpu_pool::CpuBufferPoolSubbuffer;
use vulkano::descriptor::descriptor_set::{PersistentDescriptorSet, PersistentDescriptorSetBuf};
use vulkano::descriptor::PipelineLayoutAbstract;
use vulkano::memory::pool::StdMemoryPool;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::vertex::SingleBufferDefinition;
use vulkano::framebuffer::RenderPass;
use graphics::{shader, Vertex, render_pass};
use nphysics::detection::joint;

use std::sync::Arc;
use std::any::Any;

#[derive(Clone)]
pub enum Life {
    EraserAlive,
    EraserDead,
    DrawAlive,
    DrawDead,
}

impl Life {
    pub fn kill(&mut self) {
        *self = match self.clone() {
            Life::EraserAlive => Life::EraserDead,
            Life::DrawAlive => Life::DrawDead,
            s @ _ => s,
        };
    }
}

impl ::specs::Component for Life {
    type Storage = ::specs::VecStorage<Self>;
}

pub struct Shooter {
    pub reload_time: f32,
    pub timer: f32,
    pub max_bullets: usize,
    pub bullets: usize,
    pub shoot: bool,
}

impl Shooter {
    pub fn new(reload_time: f32, max_bullets: usize) -> Self {
        Shooter {
            reload_time,
            shoot: false,
            max_bullets,
            timer: 0.0,
            bullets: max_bullets,
        }
    }

    pub fn set_shoot(&mut self, shoot: bool) {
        self.shoot = shoot;
    }
}

impl ::specs::Component for Shooter {
    type Storage = ::specs::VecStorage<Self>;
}

#[derive(Default)]
pub struct Bouncer;

impl ::specs::Component for Bouncer {
    type Storage = ::specs::NullStorage<Self>;
}

pub struct Avoider {
    pub goal: Option<::na::Vector3<f32>>,
}

impl ::specs::Component for Avoider {
    type Storage = ::specs::VecStorage<Self>;
}

impl Avoider {
    pub fn new() -> Self {
        Avoider { goal: None }
    }
}

pub struct Aim {
    pub rotation: ::na::UnitQuaternion<f32>,
}

impl Aim {
    pub fn new() -> Self {
        Aim { rotation: ::na::Unit::new_normalize(::na::Quaternion::from_vector(::na::zero())) }
    }
}

impl ::specs::Component for Aim {
    type Storage = ::specs::VecStorage<Self>;
}

#[derive(Default)]
pub struct Player;

impl ::specs::Component for Player {
    type Storage = ::specs::NullStorage<Self>;
}

pub struct Momentum {
    pub ang_damping: f32,
    pub ang_force: Option<::na::Vector3<f32>>,
    pub damping: f32,
    pub force: f32,
    pub direction: ::na::Vector3<f32>,
    pub pnt_to_com: Option<::na::Vector3<f32>>,
}

impl ::specs::Component for Momentum {
    type Storage = ::specs::VecStorage<Self>;
}

const PHYSIC_ALMOST_V_MAX: f32 = 0.9;

impl Momentum {
    pub fn new(
        mass: f32,
        velocity: f32,
        time_to_reach_v_max: f32,
        ang_force: Option<::na::Vector3<f32>>,
        ang_damping: f32,
        pnt_to_com: Option<::na::Vector3<f32>>,
    ) -> Self {
        // TODO: add ang_vel, time_to_reach_ang_v_max arguments and compute ang_damping and ang_force with it
        let damping = -mass * (1. - PHYSIC_ALMOST_V_MAX).ln() / time_to_reach_v_max;
        let force = velocity * damping;
        Momentum {
            ang_damping,
            damping,
            force,
            ang_force,
            direction: ::na::zero(),
            pnt_to_com,
        }
    }
}

pub struct AirMomentum {
    pub gravity_force: f32,
    pub damping: f32,
}

impl ::specs::Component for AirMomentum {
    type Storage = ::specs::VecStorage<Self>;
}

pub struct StaticDraw {
    pub color: ::graphics::Color,
    pub groups: Vec<u16>,
    pub primitive: usize,
    pub set: Arc<PersistentDescriptorSet<Arc<GraphicsPipeline<SingleBufferDefinition<Vertex>, Box<PipelineLayoutAbstract + Sync + Send>, Arc<RenderPass<render_pass::CustomRenderPassDesc>>>>, ((), PersistentDescriptorSetBuf<CpuBufferPoolSubbuffer<shader::draw1_vs::ty::World, Arc<StdMemoryPool>>>)>>,
}

impl ::specs::Component for StaticDraw {
    type Storage = ::specs::VecStorage<Self>;
}

impl StaticDraw {
    pub fn add<'a>(
        entity: ::specs::Entity,
        primitive: usize,
        groups: Vec<u16>,
        color: ::graphics::Color,
        world_trans: ::graphics::shader::draw1_vs::ty::World,
        static_draws: &mut ::specs::WriteStorage<'a, ::component::StaticDraw>,
        graphics: &::specs::Fetch<'a, ::resource::Graphics>,
    ) {
        let world_trans_subbuffer = graphics
            .world_uniform_static_buffer
            .next(world_trans)
            .unwrap();

        let set = Arc::new(
            PersistentDescriptorSet::start(graphics.draw1_pipeline.clone(), 0)
                .add_buffer(world_trans_subbuffer.clone())
                .unwrap()
                .build()
                .unwrap(),
        );

        let static_draw = StaticDraw {
            primitive,
            color,
            groups,
            set,
        };

        static_draws.insert(entity, static_draw);
    }
}

// Maybe add an animation here also to add reload animation
pub struct WeaponAnchor {
    // The anchor must have a rigid body and a weapon animation
    pub anchor: ::specs::Entity,
}

impl ::specs::Component for WeaponAnchor {
    type Storage = ::specs::VecStorage<Self>;
}

pub struct WeaponAnimation {
    pub weapon_trans: ::na::Isometry3<f32>,
    pub shoot_pos: ::na::Point3<f32>,
    pub light_ray_radius: f32,
    pub bullets: Vec<::specs::Entity>,
}

impl ::specs::Component for WeaponAnimation {
    type Storage = ::specs::VecStorage<Self>;
}

pub struct DynamicGraphicsAssets {
    pub primitive: usize,
    pub groups: Vec<u16>,
    pub color: ::graphics::Color,
    pub primitive_trans: ::na::Transform3<f32>,
    pub world_trans: ::graphics::shader::draw1_vs::ty::World,
}

impl ::specs::Component for DynamicGraphicsAssets {
    type Storage = ::specs::VecStorage<Self>;
}

impl DynamicGraphicsAssets {
    pub fn new(
        primitive: usize,
        groups: Vec<u16>,
        color: ::graphics::Color,
        primitive_trans: ::na::Transform3<f32>,
    ) -> Self {
        DynamicGraphicsAssets {
            primitive,
            groups,
            primitive_trans,
            color,
            world_trans: shader::draw1_vs::ty::World { world: primitive_trans.unwrap().into() },
        }
    }
}

#[derive(Default)]
pub struct DynamicDraw;

impl ::specs::Component for DynamicDraw {
    type Storage = ::specs::NullStorage<Self>;
}

#[derive(Default)]
pub struct DynamicEraser;

impl ::specs::Component for DynamicEraser {
    type Storage = ::specs::NullStorage<Self>;
}

#[derive(Default)]
pub struct DynamicHud;

impl ::specs::Component for DynamicHud {
    type Storage = ::specs::NullStorage<Self>;
}

// Rigid body handle and whereas it has been deleted
pub struct PhysicBody {
    handle: usize,
    removed: bool,
}

impl ::specs::Component for PhysicBody {
    type Storage = ::specs::VecStorage<Self>;
}

impl Drop for PhysicBody {
    fn drop(&mut self) {
        if !self.removed {
            // debug_assert!(eprintln!("physic body hasn't been removed from physic world") == ());
        }
    }
}

impl PhysicBody {
    pub fn entity(body: &::nphysics::object::RigidBody<f32>) -> ::specs::Entity {

        let entity = body.user_data().unwrap();
        let entity = unsafe { ::std::mem::transmute::<&Box<_>, &Box<Any>>(entity) };
        entity.downcast_ref::<::specs::Entity>().unwrap().clone()
    }

    pub fn add<'a>(
        entity: ::specs::Entity,
        mut body: ::nphysics::object::RigidBody<f32>,
        bodies: &mut ::specs::WriteStorage<'a, ::component::PhysicBody>,
        physic_world: &mut ::specs::FetchMut<'a, ::resource::PhysicWorld>,
    ) {
        body.set_user_data(Some(Box::new(entity)));
        let bodyhandle = physic_world.add_rigid_body(body);
        bodies.insert(
            entity,
            PhysicBody {
                handle: bodyhandle,
                removed: false,
            },
        );
    }

    #[inline]
    pub fn ball_in_socket(
        &mut self,
        physic_world: &mut ::resource::PhysicWorld,
        position: ::na::Point3<f32>,
    ) {
        physic_world.add_ball_in_socket(joint::BallInSocket::new(
            joint::Anchor::new(None, position),
            joint::Anchor::new(
                Some(self.handle),
                ::na::Point3::new(0.0, 0.0, 0.0),
            ),
        ));
    }

    #[inline]
    pub fn get<'a>(
        &'a self,
        physic_world: &'a ::resource::PhysicWorld,
    ) -> &'a ::nphysics::object::RigidBody<f32> {
        physic_world.rigid_body(self.handle)
    }

    #[inline]
    pub fn get_mut<'a>(
        &'a mut self,
        physic_world: &'a mut ::resource::PhysicWorld,
    ) -> &'a mut ::nphysics::object::RigidBody<f32> {
        physic_world.mut_rigid_body(self.handle)
    }

    pub fn remove(&mut self, physic_world: &mut ::resource::PhysicWorld) {
        if self.removed {
            panic!("physic body already removed from physic world");
        }
        physic_world.remove_rigid_body(self.handle);
        self.removed = true;
    }
}

// Sensor handle and whereas it has been deleted
pub struct PhysicSensor {
    handle: usize,
    removed: bool,
}

impl ::specs::Component for PhysicSensor {
    type Storage = ::specs::VecStorage<Self>;
}

impl Drop for PhysicSensor {
    fn drop(&mut self) {
        if !self.removed {
            // debug_assert!(eprintln!("physic body hasn't been removed from physic world") == ());
        }
    }
}

#[allow(unused)]
impl PhysicSensor {
    pub fn entity(body: &::nphysics::object::Sensor<f32>) -> ::specs::Entity {

        let entity = body.user_data().unwrap();
        let entity = unsafe { ::std::mem::transmute::<&Box<_>, &Box<Any>>(entity) };
        entity.downcast_ref::<::specs::Entity>().unwrap().clone()
    }

    pub fn add<'a>(
        entity: ::specs::Entity,
        mut sensor: ::nphysics::object::Sensor<f32>,
        sensors: &mut ::specs::WriteStorage<'a, ::component::PhysicSensor>,
        physic_world: &mut ::specs::FetchMut<'a, ::resource::PhysicWorld>,
    ) {
        sensor.set_user_data(Some(Box::new(entity)));
        let sensorhandle = physic_world.add_sensor(sensor);
        sensors.insert(
            entity,
            PhysicSensor {
                handle: sensorhandle,
                removed: false,
            },
        );
    }

    #[inline]
    pub fn get<'a>(
        &'a self,
        physic_world: &'a ::resource::PhysicWorld,
    ) -> &'a ::nphysics::object::Sensor<f32> {
        physic_world.sensor(self.handle)
    }

    #[inline]
    pub fn get_mut<'a>(
        &'a mut self,
        physic_world: &'a mut ::resource::PhysicWorld,
    ) -> &'a mut ::nphysics::object::Sensor<f32> {
        physic_world.mut_sensor(self.handle)
    }

    pub fn remove(&mut self, physic_world: &mut ::resource::PhysicWorld) {
        if self.removed {
            panic!("physic body already removed from physic world");
        }
        physic_world.remove_sensor(self.handle);
        self.removed = true;
    }
}

pub type Contact = ::ncollide::query::Contact<::na::Point3<f32>>;

pub struct Contactor {
    pub contacts: Vec<(::specs::Entity, Contact)>,
}

impl ::specs::Component for Contactor {
    type Storage = ::specs::VecStorage<Self>;
}

impl Contactor {
    pub fn new() -> Self {
        Contactor { contacts: vec![] }
    }
}

pub struct Proximitor {
    pub intersections: Vec<::specs::Entity>,
}

impl ::specs::Component for Proximitor {
    type Storage = ::specs::VecStorage<Self>;
}

impl Proximitor {
    pub fn new() -> Self {
        Proximitor { intersections: vec![] }
    }
}

pub struct DeletTimer(pub f32);

impl ::specs::Component for DeletTimer {
    type Storage = ::specs::VecStorage<Self>;
}

impl DeletTimer {
    pub fn new(timer: f32) -> Self {
        DeletTimer(timer)
    }
}

pub struct DeletBool(pub bool);

impl ::specs::Component for DeletBool {
    type Storage = ::specs::VecStorage<Self>;
}

impl DeletBool {
    pub fn new() -> Self {
        DeletBool(true)
    }
}

pub struct Turret {
    pub laser_draw: ::specs::Entity,
    pub laser_physic: ::specs::Entity,
}

impl ::specs::Component for Turret {
    type Storage = ::specs::VecStorage<Self>;
}

#[derive(Default)]
pub struct FollowPlayer {
    pub amortization: f32
}

impl ::specs::Component for FollowPlayer {
    type Storage = ::specs::VecStorage<Self>;
}

impl FollowPlayer {
    pub fn new(amortization: f32) -> Self {
        FollowPlayer { amortization }
    }
}

#[derive(Default)]
pub struct Teleport;

impl ::specs::Component for Teleport {
    type Storage = ::specs::NullStorage<Self>;
}

pub enum GeneratedEntity {
    Avoider,
    Bouncer,
}

pub struct Generator {
    pub pos: ::na::Vector3<f32>,
    pub entity: GeneratedEntity,
    pub salvo: usize,
    pub timer: f32,
    pub time_between_salvo: f32,
    pub eraser_probability: f32,
}

impl ::specs::Component for Generator {
    type Storage = ::specs::VecStorage<Self>;
}

pub struct Anchor {
    pub entity: ::specs::Entity,
    pub local_pos: ::na::Point3<f32>,
    pub pos: ::na::Vector3<f32>,
}

pub struct Hook {
    pub launch: bool,
    pub force: f32,
    pub anchor: Option<Anchor>,
}

impl ::specs::Component for Hook {
    type Storage = ::specs::VecStorage<Self>;
}

impl Hook {
    pub fn new(force: f32) -> Self {
        Hook {
            launch: false,
            force,
            anchor: None,
        }
    }

    pub fn set_launch(&mut self, launch: bool) {
        self.launch = launch;
    }
}
