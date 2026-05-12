use z3::ast::{Ast, Int};
use z3::{Context, Solver, SatResult};

/// PQC Solver Module for Formal-Engine
/// Targets ML-KEM (Kyber) Lattice Math
/// Optimized Proof: Coefficient Unmasking

pub const KYBER_Q: i64 = 3329;

pub struct PqcLatticeCore<'ctx> {
    pub ctx: &'ctx Context,
}

impl<'ctx> PqcLatticeCore<'ctx> {
    pub fn new(ctx: &'ctx Context) -> Self {
        Self { ctx }
    }

    /// Solve for a single secret coefficient 's0'
    /// Proof of Concept for linear extraction
    pub fn solve_single_coefficient(&self, u0: i32, t0: i32) -> Option<i32> {
        let solver = Solver::new(self.ctx);
        let s0 = Int::new_const(self.ctx, "s0");
        
        // Kyber standard secret bounds: [-2, 2]
        solver.assert(&s0.ge(&Int::from_i64(self.ctx, -2)));
        solver.assert(&s0.le(&Int::from_i64(self.ctx, 2)));

        // Relation: u0 * s0 = t0 mod q
        let q = Int::from_i64(self.ctx, KYBER_Q);
        let lhs = (Int::from_i64(self.ctx, u0 as i64) * &s0).rem(&q);
        let lhs_pos = lhs.ge(&Int::from_i64(self.ctx, 0)).ite(&lhs, &(&lhs + &q));
        
        let rhs = Int::from_i64(self.ctx, t0 as i64);
        solver.assert(&lhs_pos._eq(&rhs));

        if solver.check() == SatResult::Sat {
            let model = solver.get_model().unwrap();
            return Some(model.eval(&s0, true).unwrap().as_i64().unwrap() as i32);
        }
        None
    }

    /// Solve for all 256 secret coefficients 's[256]'
    /// Uses Learning With Errors (LWE) logic: u*s + e = t mod q
    /// Bounds both secret 's' and noise 'e' to [-2, 2].
    pub fn solve_full_polynomial(&self, u_vals: &[i32; 256], t_vals: &[i32; 256]) -> Option<Vec<i32>> {
        let solver = Solver::new(self.ctx);
        let q = Int::from_i64(self.ctx, KYBER_Q);
        let mut s_vars = Vec::new();

        for i in 0..256 {
            let s = Int::new_const(self.ctx, format!("s{}", i));
            let e = Int::new_const(self.ctx, format!("e{}", i));
            
            // Secret s bounds: [-2, 2]
            solver.assert(&s.ge(&Int::from_i64(self.ctx, -2)));
            solver.assert(&s.le(&Int::from_i64(self.ctx, 2)));

            // Noise e bounds: [-2, 2]
            solver.assert(&e.ge(&Int::from_i64(self.ctx, -2)));
            solver.assert(&e.le(&Int::from_i64(self.ctx, 2)));

            // Relation: (u_i * s_i) + e_i = t_i mod q
            let u = Int::from_i64(self.ctx, u_vals[i] as i64);
            let t = Int::from_i64(self.ctx, t_vals[i] as i64);
            
            let lhs = ((&u * &s) + &e).rem(&q);
            let lhs_pos = lhs.ge(&Int::from_i64(self.ctx, 0)).ite(&lhs, &(&lhs + &q));
            solver.assert(&lhs_pos._eq(&t));
            
            s_vars.push(s);
        }

        if solver.check() == SatResult::Sat {
            let model = solver.get_model().unwrap();
            let mut results = Vec::new();
            for s in s_vars {
                results.push(model.eval(&s, true).unwrap().as_i64().unwrap() as i32);
            }
            return Some(results);
        }
        None
    }
}

pub fn main() {
    println!("FORMAL-ENGINE: PQC Identity Pivot Core Active.");
}

#[cfg(test)]
mod tests {
    use super::*;
    use z3::Config;

    #[test]
    fn test_solve_single_coefficient() {
        let cfg = Config::new();
        let ctx = Context::new(&cfg);
        let core = PqcLatticeCore::new(&ctx);
        
        let s_real = -1;
        let u0 = 1234;
        // t0 = u0 * s0 mod q = 1234 * -1 mod 3329 = -1234 mod 3329 = 2095
        let t0 = (u0 * s_real as i32).rem_euclid(KYBER_Q as i32);
        
        let result = core.solve_single_coefficient(u0, t0);
        assert_eq!(result, Some(s_real));
    }

    #[test]
    fn test_solve_full_polynomial() {
        let cfg = Config::new();
        let ctx = Context::new(&cfg);
        let core = PqcLatticeCore::new(&ctx);
        
        let mut u_vals = [0; 256];
        let mut t_vals = [0; 256];
        let mut s_real = [0; 256];
        
        for i in 0..256 {
            u_vals[i] = (i as i32 * 13) % KYBER_Q as i32;
            s_real[i] = (i as i32 % 5) - 2; // bounded [-2, 2]
            let e = (i as i32 % 3) - 1; // bounded [-1, 1]
            
            let val = (u_vals[i] * s_real[i]) + e;
            t_vals[i] = val.rem_euclid(KYBER_Q as i32);
        }
        
        let results = core.solve_full_polynomial(&u_vals, &t_vals).unwrap();
        assert_eq!(results, s_real.to_vec());
    }
}
