use std::collections::{HashSet, HashMap};

use crate::matrix::CooMat;

use crate::problem::{Problem,
                     ProblemLp,
                     ProblemMilp,
                     ProblemNlp};

use crate::model::node::Node;
use crate::model::node_base::NodeBase;
use crate::model::node_std::{NodeStd, NodeStdComp};
use crate::model::constant::ConstantScalar;
use crate::model::constraint::Constraint;
use crate::model::constraint_std::{ConstraintStd, ConstraintStdComp};
use crate::model::model::{Model, Objective};

const INF: f64 = 1e8;

pub struct ModelStdComp {
    pub obj: NodeStdComp,
    pub constr: ConstraintStdComp,
}

pub enum ModelStdProb {
    Base(Problem),
    Lp(ProblemLp),
    Milp(ProblemMilp),
    Nlp(ProblemNlp),
}

pub struct ModelStdMaps {
    pub var2index: HashMap<Node, usize>,
    pub aindex2constr: HashMap<usize, Constraint>,
    pub jindex2constr: HashMap<usize, Constraint>,
    pub uindex2constr: HashMap<usize, Constraint>,
    pub lindex2constr: HashMap<usize, Constraint>,
}

pub trait ModelStd {
    fn std_components(&self) -> ModelStdComp;
    fn std_problem(&self) -> (ModelStdProb, ModelStdMaps);
}

impl ModelStd for Model {

    fn std_components(&self) -> ModelStdComp {

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
        ModelStdComp {
            obj: obj,
            constr: constr,
        }
    }

    fn std_problem(&self) -> (ModelStdProb, ModelStdMaps) {

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
        let var2index_eval: HashMap<Node, usize> = var2index.iter()
                                                            .map(|(v,i)| (v.clone(), *i))
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
        let hphi_mat = CooMat::new(
            (num_vars, num_vars),
            hphi_row,
            hphi_col,
            vec![0.; hphi_data.len()]
        );

        println!("phi_data: {}", phi_data);
        println!("gphi_indices: {:?}", gphi_indices);
        println!("gphi_data: {:?}", gphi_data);
        println!("hphi_mat: {:?}", hphi_mat);
        println!("hphi_data: {:?}", hphi_data);

        // Objective grad (c)
        let mut c_data: Vec<f64> = vec![0.; num_vars];
        for (var, val) in comp.obj.prop.a.iter() {
            c_data[*var2index.get(var).unwrap()] = *val;
        }

        println!("c: {:?}", c_data);

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
        let a_mat = CooMat::new(
            (num_a, num_vars),
            a_row,
            a_col,
            a_data
        );

        let b_data = comp.constr.b;

        println!("a_mat: {:?}", a_mat);
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
        let j_mat = CooMat::new(
            (num_j, num_vars),
            j_row,
            j_col,
            vec![0.; j_data.len()]
        );
        let f_data = comp.constr.f;
        let mut h_data: Vec<Vec<Node>> = Vec::with_capacity(num_j);
        let mut h_vec: Vec<CooMat<f64>> = Vec::with_capacity(num_j);
        for hh in comp.constr.h.into_iter() {
            let mut hh_row: Vec<usize> = Vec::with_capacity(hh.len());
            let mut hh_col: Vec<usize> = Vec::with_capacity(hh.len());
            let mut hh_data: Vec<Node> = Vec::with_capacity(hh.len());
            for (v1, v2, exp) in hh.into_iter() {
                hh_row.push(*var2index.get(&v1).unwrap());
                hh_col.push(*var2index.get(&v2).unwrap());
                hh_data.push(exp);
            }
            h_vec.push(CooMat::new(
                (num_vars, num_vars),
                hh_row,
                hh_col,
                vec![0.; hh_data.len()]
            ));
            h_data.push(hh_data);
        }

        println!("f: {:?}", f_data);
        println!("j_mat: {:?}", j_mat);
        println!("j_data: {:?}", j_data);
        println!("h_vec: {:?}", h_vec);
        println!("h_data: {:?}", h_data);

        // Bounds (l <= x <= u)
        let mut uindex2constr: HashMap<usize, Constraint> = HashMap::new();
        let mut lindex2constr: HashMap<usize, Constraint> = HashMap::new();
        let mut u_data = vec![INF; num_vars];
        let mut l_data = vec![-INF; num_vars];
        for (var, val, constr) in comp.constr.u.into_iter() {
            let index = *var2index.get(&var).unwrap();
            if val <= u_data[index] {
                u_data[index] = val;
                uindex2constr.insert(index, constr);
            }
        }
        for (var, val, constr) in comp.constr.l.into_iter() {
            let index = *var2index.get(&var).unwrap();
            if val >= l_data[index] {
                l_data[index] = val;
                lindex2constr.insert(index, constr);
            }
        }

        println!("u_data: {:?}", u_data);
        println!("l_data: {:?}", l_data);

        // Integer restrictions
        let mut num_int: usize = 0;
        let mut p_data = vec![false; num_vars];
        for (var, index) in var2index.iter() {
            match var {
                Node::VariableScalar(x) => {
                    if x.is_integer() {
                        p_data[*index] = true;
                        num_int += 1;
                    }
                }
                _ => (),
            }
        }
        
        println!("p_data: {:?}", p_data);

        // Initial values
        let mut x0_data: Vec<f64> = vec![0.; num_vars];
        for (var, val) in self.init_values().iter() {
            match var2index.get(var) {
                Some(index) => x0_data[*index] = *val,
                None => (), 
            }
        }

        println!("x0_data: {:?}", x0_data);
       
        // Eval
        let eval_fn = Box::new(move | phi: &mut f64, 
                                      gphi: &mut Vec<f64>, 
                                      hphi: &mut CooMat<f64>,
                                      f: &mut Vec<f64>,
                                      j: &mut CooMat<f64>,
                                      h: &mut Vec<CooMat<f64>>,
                                      x: &[f64] | {

            // Var values
            let mut var_values: HashMap<&Node, f64> = HashMap::with_capacity(x.len());
            for (var, index) in var2index_eval.iter() {
                var_values.insert(var, x[*index]);
            }

            // phi
            *phi = phi_data.eval(&var_values);

            // gphi
            for (index, exp) in gphi_indices.iter().zip(gphi_data.iter()) {
                (*gphi)[*index] = exp.eval(&var_values);
            }

            // hphi
            let hphi_dest = hphi.data_mut();
            for (val, exp) in hphi_dest.iter_mut().zip(hphi_data.iter()) {
                *val = exp.eval(&var_values);           
            }

            // f
            for (val, exp) in f.iter_mut().zip(f_data.iter()) {
                *val = exp.eval(&var_values);
            }

            // j
            let j_dest = j.data_mut();
            for (val, exp) in j_dest.iter_mut().zip(j_data.iter()) {
                *val = exp.eval(&var_values);
            }
            
            // h
            for (hh, hh_data) in h.iter_mut().zip(h_data.iter()) {
                let hh_dest = hh.data_mut();
                for (val, exp) in hh_dest.iter_mut().zip(hh_data.iter()) {
                    *val = exp.eval(&var_values);
                }
            }
        });

        // Maps
        let maps = ModelStdMaps {
            var2index: var2index,
            aindex2constr: aindex2constr,
            jindex2constr: jindex2constr,
            uindex2constr: uindex2constr,
            lindex2constr: lindex2constr,
        };

        // Problem
        let problem: ModelStdProb;

        // Lp
        if comp.obj.prop.affine && num_j == 0 && num_int == 0 {
            problem = ModelStdProb::Lp(
                ProblemLp::new(
                    c_data,
                    a_mat,
                    b_data,
                    l_data,
                    u_data,
                    Some(x0_data)
                )
            );
        }

        // Milp
        else if comp.obj.prop.affine && num_j == 0 && num_int > 0 {
            problem = ModelStdProb::Milp(
                ProblemMilp::new(
                    c_data,
                    a_mat,
                    b_data,
                    l_data,
                    u_data,
                    p_data,
                    Some(x0_data)
                )
            );
        }

        // Nlp
        else if num_int == 0 {
            problem = ModelStdProb::Nlp(
                ProblemNlp::new(
                    hphi_mat,
                    a_mat,
                    b_data,
                    j_mat,
                    h_vec,
                    l_data,
                    u_data,
                    Some(x0_data),
                    eval_fn,
                )
            );
        }

        // Base (Milp)
        else {
            problem = ModelStdProb::Base(
                Problem::new(
                    hphi_mat,
                    a_mat,
                    b_data,
                    j_mat,
                    h_vec,
                    l_data,
                    u_data,
                    p_data,
                    Some(x0_data),
                    eval_fn,
                )
            );
        }

        // Return
        (problem, maps)
    }
}

#[cfg(test)]
mod tests {

    use maplit::hashmap;

    use super::*;
    use crate::model::node_cmp::NodeCmp;
    use crate::model::variable::VariableScalar;
    use crate::assert_vec_approx_eq;
    use crate::problem::ProblemLpBase;

    #[test]
    fn std_problem_lp() {

        let x = VariableScalar::new_continuous("x");
        let y = VariableScalar::new_continuous("y");

        let c1 = (2.*&x + &y).equal(2.);
        let c2 = x.leq(5.);
        let c3 = x.geq(0.);
        let c4 = y.leq(5.);
        let c5 = y.geq(0.);

        let mut p = Model::new();
        p.set_objective(Objective::minimize(&(3.*&x + 4.*&y + 1.)));
        p.add_constraint(&c1);
        p.add_constraint(&c2);
        p.add_constraint(&c3);
        p.add_constraint(&c4);
        p.add_constraint(&c5);
        p.set_init_values(&hashmap!{ &x => 2., &y => 3. });

        println!("{}", p);

        let (std_p, std_maps) = p.std_problem();
        let lp = match std_p {
            ModelStdProb::Lp(x) => x,
            _ => panic!("invalid std problem")
        };

        assert_vec_approx_eq!(lp.x0().unwrap(), vec![2., 3.], epsilon=0.);
        assert_vec_approx_eq!(lp.c(), vec![3., 4.,], epsilon=0.);
        assert_eq!(lp.na(), 1);
        assert_eq!(lp.nx(), 2);
        assert_eq!(lp.a().nnz(), 2);
        for (row, col, val) in lp.a().iter() {
            if *row == 0 && *col == 0 {
                assert_eq!(*val, 2.);
            }
            else if *row == 0 && *col == 1 {
                assert_eq!(*val, 1.);
            }
            else {
                panic!("invalid a matrix")
            }
        }
        assert_vec_approx_eq!(lp.b(), vec![2.], epsilon=0.);
        assert_vec_approx_eq!(lp.l(), vec![0., 0.], epsilon=0.);
        assert_vec_approx_eq!(lp.u(), vec![5., 5.], epsilon=0.);

        assert_eq!(std_maps.var2index.len(), 2);
        assert_eq!(*std_maps.var2index.get(&x).unwrap(), 0);
        assert_eq!(*std_maps.var2index.get(&y).unwrap(), 1);
        assert_eq!(std_maps.aindex2constr.len(), 1);
        assert_eq!(*std_maps.aindex2constr.get(&0).unwrap(), c1);
        assert_eq!(std_maps.jindex2constr.len(), 0);
        assert_eq!(std_maps.uindex2constr.len(), 2);
        assert_eq!(*std_maps.uindex2constr.get(&0).unwrap(), c2);
        assert_eq!(*std_maps.uindex2constr.get(&1).unwrap(), c4);
        assert_eq!(std_maps.lindex2constr.len(), 2);
        assert_eq!(*std_maps.lindex2constr.get(&0).unwrap(), c3);
        assert_eq!(*std_maps.lindex2constr.get(&1).unwrap(), c5);
    }
}