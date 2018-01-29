pub struct DepthCoefSystem;

impl<'a> ::specs::System<'a> for DepthCoefSystem {
    type SystemData = ::specs::FetchMut<'a, ::resource::DepthCoef>;

    fn run(
        &mut self,
        mut depth_coef: Self::SystemData,
    ) {
        depth_coef.0 *= ::CONFIG.depth_coef_velocity;

        depth_coef.0 = depth_coef.0.min(1.0).max(::CONFIG.depth_coef_min);
    }
}
