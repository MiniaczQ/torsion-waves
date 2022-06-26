struct RungeKutty4 {
    relative_error: f64,
    absolute_error: f64,
    /// Timestep
    h: f64,
}

impl RungeKutty4 {
    fn step(&self, y: f64, f: &'static fn()) -> f64 {
        let c_tb = [0., 1. / 3., 2. / 3., 1.];
        let a_tb = (
            [0.],
            [1. / 3., 0.],
            [-1. / 3., 1., 0.],
            [1., -1., 1., 0.],
        );
        let b_tb = [0.125, 0.475, 0.475, 0.125];

        let k1 = f(y);
        let k2 = ;
        let k3 = ;
        let k4 = ;

        return y + 
    }
}
