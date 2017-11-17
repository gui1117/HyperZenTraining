use vulkano::buffer::cpu_pool::CpuBufferPoolSubbuffer;
use vulkano::descriptor::descriptor_set::{PersistentDescriptorSet, PersistentDescriptorSetBuf};
use vulkano::descriptor::PipelineLayoutAbstract;
use vulkano::memory::pool::StdMemoryPool;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::vertex::SingleBufferDefinition;
use vulkano::framebuffer::RenderPass;
use graphics::{shader, Vertex, render_pass};

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

enum ShooterState {
    Reloading(f32),
    Loaded,
}

pub struct Shooter {
    reload_time: f32,
    state: ShooterState,
    shoot: bool,
}

impl Shooter {
    pub fn new(reload_time: f32) -> Self {
        Shooter {
            reload_time,
            state: ShooterState::Loaded,
            shoot: false,
        }
    }

    pub fn reload(&mut self, dt: f32) {
        let set_ready = if let ShooterState::Reloading(ref mut remaining) = self.state {
            *remaining -= dt;
            *remaining <= 0.0
        } else {
            false
        };

        if set_ready {
            self.state = ShooterState::Loaded
        }
    }

    pub fn set_shoot(&mut self, shoot: bool) {
        self.shoot = shoot;
    }

    pub fn do_shoot(&mut self) -> bool {
        if !self.shoot {
            return false;
        }

        if let ShooterState::Loaded = self.state {
            self.state = ShooterState::Reloading(self.reload_time);
            true
        } else {
            false
        }
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
    pub goal: Option<(usize, usize)>,
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
    pub dir: ::na::Vector3<f32>,
    pub x_dir: f32,
}

impl Aim {
    pub fn new() -> Self {
        Aim {
            dir: ::na::Vector3::x(),
            x_dir: 0.0,
        }
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
            direction: ::na::zero(),
            pnt_to_com,
        }
    }
}

pub struct StaticDraw {
    pub color: u16,
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
        color: u16,
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
}

impl ::specs::Component for WeaponAnimation {
    type Storage = ::specs::VecStorage<Self>;
}

pub struct DynamicGraphicsAssets {
    pub primitive: usize,
    pub groups: Vec<u16>,
    pub color: u16,
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
        color: u16,
        primitive_trans: ::na::Transform3<f32>,
    ) -> Self {
        DynamicGraphicsAssets {
            primitive,
            groups,
            primitive_trans,
            color,
            world_trans: shader::draw1_vs::ty::World {
                world: primitive_trans.unwrap().into(),
            },
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

pub struct PhysicBody(usize);

impl ::specs::Component for PhysicBody {
    type Storage = ::specs::VecStorage<Self>;
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
        bodies.insert(entity, PhysicBody(bodyhandle));
    }

    #[inline]
    pub fn get<'a>(
        &'a self,
        physic_world: &'a ::resource::PhysicWorld,
    ) -> &'a ::nphysics::object::RigidBody<f32> {
        physic_world.rigid_body(self.0)
    }

    #[inline]
    pub fn get_mut<'a>(
        &'a mut self,
        physic_world: &'a mut ::resource::PhysicWorld,
    ) -> &'a mut ::nphysics::object::RigidBody<f32> {
        physic_world.mut_rigid_body(self.0)
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

pub struct Deleter {
    pub timer: f32,
}

impl ::specs::Component for Deleter {
    type Storage = ::specs::VecStorage<Self>;
}

impl Deleter {
    pub fn new(timer: f32) -> Self {
        Deleter { timer }
    }
}
