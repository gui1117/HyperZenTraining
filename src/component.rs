use vulkano::buffer::cpu_access::CpuAccessibleBuffer;
use vulkano::descriptor::descriptor_set::{PersistentDescriptorSet, PersistentDescriptorSetBuf};
use vulkano::descriptor::PipelineLayoutAbstract;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::vertex::SingleBufferDefinition;
use vulkano::framebuffer::RenderPass;
use graphics::{shader, Vertex, render_pass};

use std::sync::Arc;

pub struct Life(pub i32);

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

        if set_ready { self.state = ShooterState::Loaded }
    }

    pub fn set_shoot(&mut self, shoot: bool) {
        self.shoot = shoot;
    }

    pub fn do_shoot(&mut self) -> bool {
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
    pub group: u16,
    pub primitive: usize,
    pub uniform_buffer: Arc<CpuAccessibleBuffer<shader::vs::ty::World>>,
    pub set: Arc<PersistentDescriptorSet<Arc<GraphicsPipeline<SingleBufferDefinition<Vertex>, Box<PipelineLayoutAbstract + Sync + Send>, Arc<RenderPass<render_pass::CustomRenderPassDesc>>>>, ((), PersistentDescriptorSetBuf<Arc<CpuAccessibleBuffer<shader::vs::ty::World>>>)>>,
}

impl ::specs::Component for StaticDraw {
    type Storage = ::specs::VecStorage<Self>;
}

impl StaticDraw {
    pub fn add(
        world: &mut ::specs::World,
        entity: ::specs::Entity,
        primitive: usize,
        group: u16,
        color: u16,
        world_trans: ::graphics::shader::vs::ty::World,
    ) {
        let graphics = world.read_resource::<::resource::Graphics>();

        let uniform_buffer =
            ::vulkano::buffer::cpu_access::CpuAccessibleBuffer::<::graphics::shader::vs::ty::World>::from_data(
                graphics.device.clone(),
                ::vulkano::buffer::BufferUsage::uniform_buffer(),
                world_trans,
                ).expect("failed to create buffer");

        let set = Arc::new(
            ::vulkano::descriptor::descriptor_set::PersistentDescriptorSet::start(
                graphics.pipeline.clone(),
                0,
            ).add_buffer(uniform_buffer.clone())
                .unwrap()
                .build()
                .unwrap(),
        );

        let static_draw = StaticDraw {
            primitive,
            color,
            group,
            uniform_buffer,
            set,
        };

        match world.write::<StaticDraw>().insert(entity, static_draw) {
            ::specs::InsertResult::Inserted => (),
            _ => panic!("cannot insert staticdraw to entity"),
        };
    }
}

pub struct DynamicDraw {
    /// index and group
    pub primitives: Vec<(usize, u16)>,
    pub color: u16,
    pub primitive_trans: ::na::Transform3<f32>,
    pub world_trans: ::graphics::shader::vs::ty::World,
    pub uniform_buffer_pool:
        Arc<::vulkano::buffer::cpu_pool::CpuBufferPool<::graphics::shader::vs::ty::World>>,
}

impl ::specs::Component for DynamicDraw {
    type Storage = ::specs::VecStorage<Self>;
}

impl DynamicDraw {
    pub fn add(
        world: &mut ::specs::World,
        entity: ::specs::Entity,
        primitives: Vec<(usize, u16)>,
        color: u16,
        primitive_trans: ::na::Transform3<f32>,
    ) {
        let graphics = world.read_resource::<::resource::Graphics>();

        let uniform_buffer_pool = Arc::new(::vulkano::buffer::cpu_pool::CpuBufferPool::new(
            graphics.device.clone(),
            ::vulkano::buffer::BufferUsage::uniform_buffer(),
        ));

        let dynamic_draw = DynamicDraw {
            primitives,
            uniform_buffer_pool,
            primitive_trans,
            color,
            world_trans: ::graphics::shader::vs::ty::World { world: [[0f32; 4]; 4] },
        };

        match world.write().insert(entity, dynamic_draw) {
            ::specs::InsertResult::Inserted => (),
            _ => panic!("cannot insert dynamicdraw to entity"),
        };
    }
}

pub struct PhysicBody(usize);

impl ::specs::Component for PhysicBody {
    type Storage = ::specs::VecStorage<Self>;
}

impl PhysicBody {
    pub fn add(world: &mut ::specs::World, entity: ::specs::Entity, body: usize) {
        world.write_resource::<::resource::PhysicWorld>().mut_rigid_body(body).set_user_data(Some(Box::new(entity)));
        world.write().insert(entity, PhysicBody(body));
    }

    #[inline]
    pub fn get<'a>(&'a self, physic_world: &'a ::resource::PhysicWorld) -> &'a ::nphysics::object::RigidBody<f32> {
        physic_world.rigid_body(self.0)
    }

    #[inline]
    pub fn get_mut<'a>(&'a mut self, physic_world: &'a mut ::resource::PhysicWorld) -> &'a mut ::nphysics::object::RigidBody<f32> {
        physic_world.mut_rigid_body(self.0)
    }
}
