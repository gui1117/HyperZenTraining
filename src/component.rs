use std::sync::Arc;

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

pub struct Player {
    pub aim: ::na::Vector3<f32>,
    pub x_aim: f32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            aim: ::na::Vector3::x(),
            x_aim: 0.0,
        }
    }
}

impl ::specs::Component for Player {
    type Storage = ::specs::HashMapStorage<Self>;
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
    pub group: u32,
    pub uniform_buffer: Arc<::vulkano::buffer::cpu_access::CpuAccessibleBuffer<::graphics::shader::vs::ty::World>>,
    pub set: Arc<::vulkano::descriptor::descriptor_set::PersistentDescriptorSet<Arc<::vulkano::pipeline::GraphicsPipeline<::vulkano::pipeline::vertex::SingleBufferDefinition<::graphics::Vertex>, Box<::vulkano::descriptor::PipelineLayoutAbstract + Sync + Send>, Arc<::vulkano::framebuffer::RenderPass<::graphics::render_pass::CustomRenderPassDesc>>>>, ((), ::vulkano::descriptor::descriptor_set::PersistentDescriptorSetBuf<Arc<::vulkano::buffer::CpuAccessibleBuffer<::graphics::shader::vs::ty::World>>>)>>,
}

impl ::specs::Component for StaticDraw {
    type Storage = ::specs::VecStorage<Self>;
}

impl StaticDraw {
    pub fn add(
        world: &mut ::specs::World,
        entity: ::specs::Entity,
        group: u32,
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
    pub group: u32,
    // pub primitive: TODO: allow different primitive
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
        group: u32,
        primitive_trans: ::na::Transform3<f32>,
    ) {
        let graphics = world.read_resource::<::resource::Graphics>();

        let uniform_buffer_pool = Arc::new(::vulkano::buffer::cpu_pool::CpuBufferPool::new(
            graphics.device.clone(),
            ::vulkano::buffer::BufferUsage::uniform_buffer(),
        ));


        let dynamic_draw = DynamicDraw {
            group,
            uniform_buffer_pool,
            primitive_trans,
            world_trans: ::graphics::shader::vs::ty::World { world: [[0f32; 4]; 4] },
        };

        match world.write::<DynamicDraw>().insert(entity, dynamic_draw) {
            ::specs::InsertResult::Inserted => (),
            _ => panic!("cannot insert dynamicdraw to entity"),
        };
    }
}

pub struct PhysicRigidBodyHandle(::nphysics::object::RigidBodyHandle<f32>);
unsafe impl Send for PhysicRigidBodyHandle {}
unsafe impl Sync for PhysicRigidBodyHandle {}

impl ::specs::Component for PhysicRigidBodyHandle {
    type Storage = ::specs::VecStorage<Self>;
}


impl PhysicRigidBodyHandle {
    pub fn new(body: ::nphysics::object::RigidBodyHandle<f32>) -> Self {
        PhysicRigidBodyHandle(body)
    }

    // TODO: maybe the clone method of ref is not thread safe ...
    pub fn get<'a>(
        &'a self,
        _world: &'a ::resource::PhysicWorld,
    ) -> ::std::cell::Ref<'a, ::nphysics::object::RigidBody<f32>> {
        self.0.borrow()
    }

    pub fn get_mut<'a>(
        &'a mut self,
        _world: &'a mut ::resource::PhysicWorld,
    ) -> ::std::cell::RefMut<'a, ::nphysics::object::RigidBody<f32>> {
        self.0.borrow_mut()
    }
}
