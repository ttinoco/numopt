use crate::model::node::Node;
use crate::model::node_std::NodeStd;
use crate::model::node_std::NodeStdProp;
use crate::model::constant::ConstantScalar;
use crate::model::constraint::{Constraint, ConstraintKind};

pub struct ConstraintStdComp {
    pub ca: Vec<Constraint>,
    pub cj: Vec<Constraint>,
    pub a: Vec<(usize, Node, f64)>,
    pub b: Vec<f64>,               
    pub f: Vec<Node>,                
    pub j: Vec<(usize, Node, Node)>,
    pub h: Vec<(Node, Node, Node)>,
    pub u: Vec<(Node, f64, Constraint)>,
    pub l: Vec<(Node, f64, Constraint)>,
    pub prop: Vec<NodeStdProp>,
}

pub trait ConstraintStd {
    fn components(&self, arow: &mut usize, jrow: &mut usize) -> ConstraintStdComp;
}

impl ConstraintStd for Constraint {

    fn components(&self, arow: &mut usize, jrow: &mut usize) -> ConstraintStdComp {

        let mut ca: Vec<Constraint> = Vec::new();
        let mut cj: Vec<Constraint> = Vec::new();
        let mut a: Vec<(usize, Node, f64)> = Vec::new();
        let mut b: Vec<f64> = Vec::new();
        let mut f: Vec<Node> = Vec::new();    
        let mut j: Vec<(usize, Node, Node)> = Vec::new();
        let mut h: Vec<(Node, Node, Node)> = Vec::new();
        let mut u: Vec<(Node, f64, Constraint)> = Vec::new();
        let mut l: Vec<(Node, f64, Constraint)> = Vec::new();
    
        let exp = self.lhs()-self.rhs();
        let comp = exp.components();
        let mut prop = comp.prop;

        // Bound constraint
        if prop.affine && 
           prop.a.len() == 1 && 
           *prop.a.values().next().unwrap() == 1. && 
           *self.kind() != ConstraintKind::Equal {
            match self.kind() {
                ConstraintKind::LessEqual => {    // x + b <= 0
                    u.push((prop.a.keys().next().unwrap().clone(),
                            -prop.b,
                            self.clone()))
                },
                ConstraintKind::GreaterEqual => { // x + b >= 0
                    l.push((prop.a.keys().next().unwrap().clone(),
                            -prop.b,
                            self.clone()))
                },
                _ => panic!("unexpected constraint type"),
            }
        }

        // Affine constraint
        else if prop.affine {

            // a^Tx + b == 0
            if *self.kind() == ConstraintKind::Equal {
                for (x, val) in prop.a.iter() {
                    a.push((*arow, x.clone(), *val)); 
                }
                b.push(-prop.b);
                ca.push(self.clone());
                *arow += 1;
            }

            // a^Tx + b - s == 0 and s <= 0 or s >= 0
            else {
                let s = self.slack();
                for (x, val) in prop.a.iter() {
                    a.push((*arow, x.clone(), *val)); 
                }
                a.push((*arow, s.clone(), -1.)); 
                b.push(-prop.b);
                ca.push(self.clone());
                match self.kind() {
                    ConstraintKind::LessEqual => {
                        u.push((s.clone(), 0., self.clone()))
                    },
                    ConstraintKind::GreaterEqual => {
                        l.push((s.clone(), 0., self.clone()))
                    },
                    _ => panic!("unexpected constraint type"),
                }
                prop.a.insert(s.clone(), -1.);
                *arow += 1;
            }
        }

        // Nonlinear constraint
        else {

            // H
            for (v1, v2, e) in comp.hphi.iter() {
                h.push((v1.clone(), v2.clone(), e.clone()))
            }   

            // f(x) = 0
            if *self.kind() == ConstraintKind::Equal {
                f.push(comp.phi.clone());
                cj.push(self.clone());
                for (x, e) in comp.gphi.iter() {
                    j.push((*jrow, x.clone(), e.clone()));
                }
                *jrow += 1;
            }

            // f(x) - s == 0 and s <= 0 or s >= 0
            else {
                let s = self.slack();
                f.push(comp.phi-s);
                cj.push(self.clone());
                for (x, e) in comp.gphi.iter() {
                    j.push((*jrow, x.clone(), e.clone()));
                }
                j.push((*jrow, s.clone(), ConstantScalar::new(-1.)));
                match self.kind() {
                    ConstraintKind::LessEqual => {
                        u.push((s.clone(), 0., self.clone()))
                    },
                    ConstraintKind::GreaterEqual => {
                        l.push((s.clone(), 0., self.clone()))
                    },
                    _ => panic!("unexpected constraint type"),
                }
                prop.a.insert(s.clone(), -1.);
                *jrow += 1;
            }
        }

        // Return
        ConstraintStdComp {
            ca: ca,
            cj: cj,
            a: a,
            b: b,
            f: f,
            j: j,
            h: h,
            u: u,
            l: l,
            prop: vec![prop],
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::model::node_cmp::NodeCmp;
    use crate::model::variable::VariableScalar;

    #[test]
    fn components_u_bound() {

        let x = VariableScalar::new_continuous("x");

        let c1 = (&x).leq(3.);
        let mut arow: usize = 1;
        let mut jrow: usize = 2;
        let comp1 = c1.components(&mut arow, &mut jrow);

        assert_eq!(comp1.ca.len(), 0);
        assert_eq!(comp1.cj.len(), 0);
        assert_eq!(comp1.a.len(), 0);
        assert_eq!(comp1.b.len(), 0);
        assert_eq!(comp1.f.len(), 0);
        assert_eq!(comp1.j.len(), 0);
        assert_eq!(comp1.h.len(), 0);
        assert_eq!(comp1.u.len(), 1);
        let (cx, cval, c) = &comp1.u[0];
        assert_eq!(*cx, x);
        assert_eq!(*cval, 3.);
        assert_eq!(*c, c1);
        assert_eq!(comp1.l.len(), 0);
        assert_eq!(arow, 1);
        assert_eq!(jrow, 2);
    }

    #[test]
    fn components_l_bound() {

        let x = VariableScalar::new_continuous("x");

        let c1 = (&x).geq(-4.);
        let mut arow: usize = 1;
        let mut jrow: usize = 2;
        let comp1 = c1.components(&mut arow, &mut jrow);

        assert_eq!(comp1.ca.len(), 0);
        assert_eq!(comp1.cj.len(), 0);
        assert_eq!(comp1.a.len(), 0);
        assert_eq!(comp1.b.len(), 0);
        assert_eq!(comp1.f.len(), 0);
        assert_eq!(comp1.j.len(), 0);
        assert_eq!(comp1.h.len(), 0);
        assert_eq!(comp1.u.len(), 0);
        assert_eq!(comp1.l.len(), 1);
        let (cx, cval, c) = &comp1.l[0];
        assert_eq!(*cx, x);
        assert_eq!(*cval, -4.);
        assert_eq!(*c, c1);
        assert_eq!(arow, 1);
        assert_eq!(jrow, 2);
    }

    #[test]
    fn components_affine_eq() {

        let x = VariableScalar::new_continuous("x");
        let y = VariableScalar::new_continuous("y");

        let c1 = (3.*&x + 4.*&y + 6.).equal(5.);
        let mut arow: usize = 1;
        let mut jrow: usize = 2;
        let comp1 = c1.components(&mut arow, &mut jrow);

        assert_eq!(comp1.ca.len(), 1);
        assert_eq!(comp1.ca[0], c1);
        assert_eq!(comp1.cj.len(), 0);
        assert_eq!(comp1.a.len(), 2);
        assert_eq!(comp1.b.len(), 1);
        let mut counter = 0_usize;
        for (row, col, val) in comp1.a.iter() {
            if *col == x {
                assert_eq!(*row, 1);
                assert_eq!(*val, 3.);
                counter += 1;
            }
            else if *col == y {
                assert_eq!(*row, 1);
                assert_eq!(*val, 4.);
                counter += 1;
            }
            else {
                panic!("error");
            }
        }
        assert_eq!(counter, 2);
        assert_eq!(comp1.b[0], -1.);
        assert_eq!(comp1.f.len(), 0);
        assert_eq!(comp1.j.len(), 0);
        assert_eq!(comp1.h.len(), 0);
        assert_eq!(comp1.u.len(), 0);
        assert_eq!(comp1.l.len(), 0);
        assert_eq!(arow, 2);
        assert_eq!(jrow, 2);
    }

    #[test]
    fn components_affine_leq() {

        let x = VariableScalar::new_continuous("x");
        let y = VariableScalar::new_continuous("y");

        let c1 = (3.*&x + 4.*&y + 6.).leq(5.);
        let mut arow: usize = 1;
        let mut jrow: usize = 2;
        let comp1 = c1.components(&mut arow, &mut jrow);

        assert_eq!(comp1.ca.len(), 1);
        assert_eq!(comp1.ca[0], c1);
        assert_eq!(comp1.cj.len(), 0);
        assert_eq!(comp1.a.len(), 3);
        assert_eq!(comp1.b.len(), 1);
        let mut counter = 0_usize;
        for (row, col, val) in comp1.a.iter() {
            if *col == x {
                assert_eq!(*row, 1);
                assert_eq!(*val, 3.);
                counter += 1;
            }
            else if *col == y {
                assert_eq!(*row, 1);
                assert_eq!(*val, 4.);
                counter += 1;
            }
            else if *col == *c1.slack() {
                assert_eq!(*row, 1);
                assert_eq!(*val, -1.);
                counter += 1;
            }
        }
        assert_eq!(counter, 3);
        assert_eq!(comp1.b[0], -1.);
        assert_eq!(comp1.f.len(), 0);
        assert_eq!(comp1.j.len(), 0);
        assert_eq!(comp1.h.len(), 0);
        assert_eq!(comp1.u.len(), 1);
        let (var, val, c) = &comp1.u[0];
        assert_eq!(*var, *c1.slack());
        assert_eq!(*val, 0.);
        assert_eq!(*c, c1);
        assert_eq!(comp1.l.len(), 0);
        assert_eq!(arow, 2);
        assert_eq!(jrow, 2);
    }

    #[test]
    fn components_affine_geq() {

        let x = VariableScalar::new_continuous("x");
        let y = VariableScalar::new_continuous("y");

        let c1 = (3.*&x + 4.*&y + 6.).geq(5.);
        let mut arow: usize = 1;
        let mut jrow: usize = 2;
        let comp1 = c1.components(&mut arow, &mut jrow);

        assert_eq!(comp1.ca.len(), 1);
        assert_eq!(comp1.ca[0], c1);
        assert_eq!(comp1.cj.len(), 0);
        assert_eq!(comp1.a.len(), 3);
        assert_eq!(comp1.b.len(), 1);
        let mut counter = 0_usize;
        for (row, col, val) in comp1.a.iter() {
            if *col == x {
                assert_eq!(*row, 1);
                assert_eq!(*val, 3.);
                counter += 1;
            }
            else if *col == y {
                assert_eq!(*row, 1);
                assert_eq!(*val, 4.);
                counter += 1;
            }
            else if *col == *c1.slack() {
                assert_eq!(*row, 1);
                assert_eq!(*val, -1.);
                counter += 1;
            }
        }
        assert_eq!(counter, 3);    
        assert_eq!(comp1.b[0], -1.);
        assert_eq!(comp1.f.len(), 0);
        assert_eq!(comp1.j.len(), 0);
        assert_eq!(comp1.h.len(), 0);
        assert_eq!(comp1.u.len(), 0);
        assert_eq!(comp1.l.len(), 1);
        let (var, val, c) = &comp1.l[0];
        assert_eq!(*var, *c1.slack());
        assert_eq!(*val, 0.);
        assert_eq!(*c, c1);
        assert_eq!(arow, 2);
        assert_eq!(jrow, 2);
    }

    #[test]
    fn components_nonlinear_eq() {

        let x = VariableScalar::new_continuous("x");
        let y = VariableScalar::new_continuous("y");

        let c1 = (3.*&x*&x + 4.*&x*&y + 7.*&y*&y + 8.).equal(5.);
        let mut arow: usize = 1;
        let mut jrow: usize = 2;
        let comp1 = c1.components(&mut arow, &mut jrow);

        assert_eq!(comp1.ca.len(), 0);
        assert_eq!(comp1.cj.len(), 1);
        assert_eq!(comp1.a.len(), 0);
        assert_eq!(comp1.b.len(), 0);
        assert_eq!(comp1.f.len(), 1);
        assert_eq!(comp1.j.len(), 2);
        assert_eq!(comp1.h.len(), 3);
        assert_eq!(comp1.u.len(), 0);
        assert_eq!(comp1.l.len(), 0);
        assert_eq!(arow, 1);
        assert_eq!(jrow, 3);
    }

    #[test]
    fn components_nonlinear_leq() {


    }

    #[test]
    fn components_nonlinear_geq() {


    }
}