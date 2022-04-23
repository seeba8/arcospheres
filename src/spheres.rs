use std::ops::{AddAssign, SubAssign};

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
        self._balance(depth, &mut vec![])
    }
    fn _balance(&mut self, depth: u8, path: &mut Vec<u8>) {
        if self.is_balanced() {
            println!(
                "{:?}",
                path.iter().map(|step| step + 1).collect::<Vec<u8>>()
            );
            return;
        }
        if depth == 0 {
            return;
        }
        match (self.zeta as i16 + self.theta as i16 + self.gamma as i16 + self.omega as i16)
            .cmp(&(self.lambda as i16 + self.xi as i16 + self.epsilon as i16 + self.phi as i16))
        {
            std::cmp::Ordering::Greater => self.add_assign(&INVERSION_RECIPES[0]),
            std::cmp::Ordering::Less => self.add_assign(&INVERSION_RECIPES[1]),
            std::cmp::Ordering::Equal => (),
        }
        for (i, recipe) in FOLDING_RECIPES.iter().enumerate() {
            path.push(i as u8);
            self.add_assign(recipe);
            self._balance(depth - 1, path);
            self.sub_assign(recipe);
            path.pop();
        }
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
