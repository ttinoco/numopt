use std::collections::{HashSet, HashMap};

use crate::model::node::Node;
use crate::model::node_std::{NodeStd, NodeStdComp};
use crate::model::constant::ConstantScalar;
use crate::model::constraint::Constraint;
use crate::model::constraint_std::{ConstraintStd, ConstraintStdComp};
use crate::model::problem::{Problem, Objective};

pub struct ProblemStdComp {
    pub obj: NodeStdComp,
    pub constr: ConstraintStdComp,
}

pub trait ProblemStd {
    fn std_components(&self) -> ProblemStdComp;
    fn std_problem(&self) -> ();
}

impl ProblemStd for Problem {

    fn std_components(&self) -> ProblemStdComp {

        // Objective std comp
        let obj = match self.objective() {
            Objective::Maximize(f) => f.std_components(),
            Objective::Minimize(f) => f.std_components(),
            Objective::Empty => ConstantScalar::new(0.).std_components(),
        };

        // Constraint std comp
        let mut arow: usize = 0;
        let mut jrow: usize = 0;
        let mut constr = ConstraintStdComp::new();
        for c in self.constraints().iter() {
            constr += c.std_components(&mut arow, &mut jrow);
        }

        // Return
        ProblemStdComp {
            obj: obj,
            constr: constr,
        }
    }

    fn std_problem(&self) -> () {

        // Components
        let comp = self.std_components();

        // Variables
        let mut varset: HashSet<Node> = comp.obj.prop.a.keys()
                                                       .map(|x| x.clone())
                                                       .collect();
        for p in comp.constr.prop.iter() {
            varset.extend(p.a.keys().map(|x| x.clone()));
        }
        let num_vars: usize = varset.len();
        let mut vars: Vec<Node> = varset.into_iter().collect();
        vars.sort_by(|x,y| x.name().partial_cmp(y.name()).unwrap());
        let var2index: HashMap<Node, usize> = vars.into_iter()
                                                  .enumerate()
                                                  .map(|(i,v)| (v,i))
                                                  .collect();
        
        println!("var2index: {:?}", var2index); 

        // Objective (phi)
        let phi_data = comp.obj.phi;
        let mut gphi_indices: Vec<usize> = Vec::with_capacity(comp.obj.gphi.len());
        let mut gphi_data: Vec<Node> = Vec::with_capacity(comp.obj.gphi.len());
        for (v, e) in comp.obj.gphi.into_iter() {
            gphi_indices.push(*var2index.get(&v).unwrap());
            gphi_data.push(e);
        }
        let mut hphi_row: Vec<usize> = Vec::with_capacity(comp.obj.hphi.len());
        let mut hphi_col: Vec<usize> = Vec::with_capacity(comp.obj.hphi.len());
        let mut hphi_data: Vec<Node> = Vec::with_capacity(comp.obj.hphi.len());
        for (v1, v2, e) in comp.obj.hphi.into_iter() {
            let i = *var2index.get(&v1).unwrap();
            let j = *var2index.get(&v2).unwrap();
            if i >= j {
                hphi_row.push(i);
                hphi_col.push(j);
            }
            else {
                hphi_row.push(j);
                hphi_col.push(i);
            }
            hphi_data.push(e);
        }

        println!("phi_data: {}", phi_data);
        println!("gphi_indices: {:?}", gphi_indices);
        println!("gphi_data: {:?}", gphi_data);
        println!("hphi_row: {:?}", hphi_row);
        println!("hphi_col: {:?}", hphi_col);
        println!("hphi_data: {:?}", hphi_data);

        // Linear equality constraints (Ax = b)
        let aindex2constr: HashMap<usize, Constraint> = comp.constr.ca.into_iter()
                                                                      .enumerate()
                                                                      .collect();
        let num_a: usize = comp.constr.b.len();
        let mut a_row: Vec<usize> = Vec::with_capacity(comp.constr.a.len());
        let mut a_col: Vec<usize> = Vec::with_capacity(comp.constr.a.len());
        let mut a_data: Vec<f64> = Vec::with_capacity(comp.constr.a.len());
        for (row, var, val) in comp.constr.a.into_iter() {
            a_row.push(row);
            a_col.push(*var2index.get(&var).unwrap());
            a_data.push(val);
        }
        let b_data = comp.constr.b;

        println!("a_row: {:?}", a_row);
        println!("a_col: {:?}", a_col);
        println!("a_data: {:?}", a_data);
        println!("b_data: {:?}", b_data);

        // Nonlinear equality constraints (f(x) = 0)
        let jindex2constr: HashMap<usize, Constraint> = comp.constr.cj.into_iter()
                                                                      .enumerate()
                                                                      .collect();
        let num_j: usize = comp.constr.f.len();
        let mut j_row: Vec<usize> = Vec::with_capacity(comp.constr.j.len());
        let mut j_col: Vec<usize> = Vec::with_capacity(comp.constr.j.len());
        let mut j_data: Vec<Node> = Vec::with_capacity(comp.constr.j.len());
        for (row, var, exp) in comp.constr.j.into_iter() {
            j_row.push(row);
            j_col.push(*var2index.get(&var).unwrap());
            j_data.push(exp);
        }
        let f_data = comp.constr.f;
        let mut h_row: Vec<Vec<usize>> = Vec::with_capacity(num_j);
        let mut h_col: Vec<Vec<usize>> = Vec::with_capacity(num_j);
        let mut h_data: Vec<Vec<Node>> = Vec::with_capacity(num_j);
        for hh in comp.constr.h.into_iter() {
            let mut hh_row: Vec<usize> = Vec::with_capacity(hh.len());
            let mut hh_col: Vec<usize> = Vec::with_capacity(hh.len());
            let mut hh_data: Vec<Node> = Vec::with_capacity(hh.len());
            for (v1, v2, exp) in hh.into_iter() {
                hh_row.push(*var2index.get(&v1).unwrap());
                hh_col.push(*var2index.get(&v2).unwrap());
                hh_data.push(exp);
            }
            h_row.push(hh_row);
            h_col.push(hh_col);
            h_data.push(hh_data);
        }

        println!("f: {:?}", f_data);
        println!("j_row: {:?}", j_row);
        println!("j_col: {:?}", j_col);
        println!("j_data: {:?}", j_data);
        println!("h_row: {:?}", h_row);
        println!("h_col: {:?}", h_col);
        println!("h_data: {:?}", h_data);

        // Bounds (l <= x <= u)

        // Integer restrictions
        
        // Problem
        // LP
        // MILP
        // NLP
        // MINLP


    }
}

#[cfg(test)]
mod tests {

    use maplit::hashmap;

    use super::*;
    use crate::model::node_cmp::NodeCmp;
    use crate::model::variable::VariableScalar;

    #[test]
    fn std_problem() {

        let x = VariableScalar::new_continuous("x");
        let y = VariableScalar::new_continuous("y");

        let mut p = Problem::new();
        p.set_objective(Objective::minimize(&(3.*&x + 4.*&y + 1.)));
        p.add_constraint(&(2.*&x + &y).equal(2.));
        p.add_constraint(&(&x.leq(5.)));
        p.add_constraint(&(&x.geq(0.)));
        p.add_constraint(&(&y.leq(5.)));
        p.add_constraint(&(&y.geq(0.)));
        p.set_init_values(&hashmap!{ &x => 2., &y => 3. });

        println!("{}", p);

        p.std_problem();
    }
}