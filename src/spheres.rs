use std::{ops::{AddAssign, SubAssign}, time::Instant};

use crate::recipes::{FOLDING_RECIPES, INVERSION_RECIPES};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Spheres {
    pub zeta: i16,
    pub theta: i16,
    pub gamma: i16,
    pub omega: i16,
    pub lambda: i16,
    pub xi: i16,
    pub epsilon: i16,
    pub phi: i16,
}

impl Spheres {
    pub fn balance(&mut self, depth: u8) {
        let start = Instant::now();
        if depth > 10 {
            println!("Spawning threads");
            let mut handles = vec![];
            for (i, recipe) in FOLDING_RECIPES.iter().enumerate() {
                let mut cloned = self.clone();
                handles.push(std::thread::spawn(move || {
                    let local_start = Instant::now();
                    let mut path = vec![];
                    path.push(i as u8);
                    cloned.add_assign(recipe);
                    cloned._balance(depth - 1, &mut path);
                    println!("Time elapsed in thread {i}: {:?}", local_start.elapsed());
                }));
            }
            for handle in handles {
                handle.join().unwrap();
            }
        } else {
            self._balance(depth, &mut vec![]);
        }
        println!("Total time elapsed: {:?}", start.elapsed());
    }
    fn _balance(&mut self, depth: u8, path: &mut Vec<u8>) -> bool {
        if self.is_balanced() {
            println!(
                "{:?}",
                path.iter().map(|step| step + 1).collect::<Vec<u8>>()
            );
            return true;
        }
        if depth == 0 {
            return false;
        }
        match (self.zeta + self.theta + self.gamma + self.omega)
            .cmp(&(self.lambda + self.xi + self.epsilon + self.phi))
        {
            std::cmp::Ordering::Greater => self.add_assign(&INVERSION_RECIPES[0]),
            std::cmp::Ordering::Less => self.add_assign(&INVERSION_RECIPES[1]),
            std::cmp::Ordering::Equal => (),
        }
        assert!(self.zeta + self.theta + self.gamma + self.omega == self.lambda + self.xi + self.epsilon + self.phi);
        for (i, recipe) in FOLDING_RECIPES.iter().enumerate() {
            path.push(i as u8);
            self.add_assign(recipe);
            if self._balance(depth - 1, path) {
                return true;
            }
            self.sub_assign(recipe);
            path.pop();
        }
        return false;
    }

    pub fn is_balanced(&self) -> bool {
        self.zeta == self.theta
            && self.gamma == self.zeta
            && self.omega == self.zeta
            && self.lambda == self.zeta
            && self.xi == self.zeta
            && self.epsilon == self.zeta
            && self.phi == self.zeta
    }
}

impl AddAssign<&Spheres> for Spheres {
    fn add_assign(&mut self, rhs: &Spheres) {
        self.zeta += rhs.zeta;
        self.theta += rhs.theta;
        self.gamma += rhs.gamma;
        self.omega += rhs.omega;
        self.lambda += rhs.lambda;
        self.xi += rhs.xi;
        self.epsilon += rhs.epsilon;
        self.phi += rhs.phi;
    }
}

impl SubAssign<&Spheres> for Spheres {
    fn sub_assign(&mut self, rhs: &Spheres) {
        self.zeta -= rhs.zeta;
        self.theta -= rhs.theta;
        self.gamma -= rhs.gamma;
        self.omega -= rhs.omega;
        self.lambda -= rhs.lambda;
        self.xi -= rhs.xi;
        self.epsilon -= rhs.epsilon;
        self.phi -= rhs.phi;
    }
}
