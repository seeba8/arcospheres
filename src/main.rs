use std::ops::{ AddAssign, SubAssign};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Spheres {
    zeta: i8,
    theta: i8,
    gamma: i8,
    omega: i8,
    lambda: i8,
    xi: i8,
    epsilon: i8,
    phi: i8,
}

impl Spheres {
    pub fn balance(&mut self, depth: u8) -> bool {
        if self.is_balanced() {
            return true;
        }
        if depth == 0 {
            return false;
        }
        for (i, recipe) in RECIPES.iter().enumerate() {
            self.add_assign(recipe);
            if self.balance(depth - 1) {
                println!("{i}");
                return true;
            }
            self.sub_assign(recipe);
        }
        false
    }

    pub fn is_balanced(&self) -> bool {
        self.zeta == self.theta && self.gamma == self.zeta && self.omega == self.zeta && self.lambda == self.zeta && self.xi == self.zeta && self.epsilon == self.zeta && self.phi == self.zeta
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

static RECIPES: [Spheres; 8] = [
    Spheres {
        zeta: 0,
        theta: 1,
        gamma: 0,
        omega: -1,
        lambda: -1,
        xi: 1,
        epsilon: 0,
        phi: 0,
    },
    Spheres {
        zeta: 1,
        theta: 0,
        gamma: -1,
        omega: 0,
        lambda: 1,
        xi: -1,
        epsilon: 0,
        phi: 0,
    },
    Spheres {
        zeta: -1,
        theta: 1,
        gamma: 0,
        omega: 0,
        lambda: 0,
        xi: -1,
        epsilon: 0,
        phi: 1,
    },
    Spheres {
        zeta: 1,
        theta: -1,
        gamma: 0,
        omega: 0,
        lambda: -1,
        xi: 0,
        epsilon: 1,
        phi: 0,
    },
    Spheres {
        zeta: 0,
        theta: -1,
        gamma: 0,
        omega: 1,
        lambda: 0,
        xi: 0,
        epsilon: -1,
        phi: 1,
    },
    Spheres {
        zeta: -1,
        theta: 0,
        gamma: 1,
        omega: 0,
        lambda: 0,
        xi: 0,
        epsilon: 1,
        phi: -1,
    },
    Spheres {
        zeta: 0,
        theta: 0,
        gamma: -1,
        omega: 1,
        lambda: 0,
        xi: 1,
        epsilon: 0,
        phi: -1,
    },
    Spheres {
        zeta: 0,
        theta: 0,
        gamma: 1,
        omega: -1,
        lambda: 1,
        xi: 0,
        epsilon: -1,
        phi: 0,
    },
];

fn main() {
    let mut spheres = Spheres {
        zeta: -1,
        theta: 1,
        gamma: 0,
        omega: 0,
        lambda: -1,
        xi: -1,
        epsilon:1,
        phi: 1,
    }; // naquium processor 1
    let mut spheres = Spheres {
        zeta: -1,
        theta: 1,
        gamma: -1,
        omega: 1,
        lambda: -1,
        xi: 1,
        epsilon:-1,
        phi: 1,
    }; // wormhole data
    let mut spheres = Spheres {
        zeta: -1,
        theta: 1,
        gamma: 0,
        omega: 0,
        lambda: -1,
        xi: -1,
        epsilon: 1,
        phi: 1,
    }; // Tesseract 1
    let mut guess = String::new();
    println!("Recursion Depth: ");
    std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("Calculating. Read output bottom to top");
    spheres.balance(guess.trim().parse().expect("Expected number of recursion depth (u8)"));
    println!("Finished");
}


#[cfg(test)]
mod tests {
    use crate::{Spheres, RECIPES};

    #[test]
    fn it_applies_recipe() {
        let mut spheres = Spheres::default();
        spheres += &RECIPES[0];
        assert_eq!(Spheres {
            zeta: 0,
            theta: 1,
            gamma: 0,
            omega: -1,
            lambda: -1,
            xi: 1,
            epsilon: 0,
            phi: 0,
        }, spheres);

        spheres += &RECIPES[0];
        assert_eq!(Spheres {
            zeta: 0,
            theta: 2,
            gamma: 0,
            omega: -2,
            lambda: -2,
            xi: 2,
            epsilon: 0,
            phi: 0,
        }, spheres);

        spheres -= &RECIPES[0];
        assert_eq!(Spheres {
            zeta: 0,
            theta: 1,
            gamma: 0,
            omega: -1,
            lambda: -1,
            xi: 1,
            epsilon: 0,
            phi: 0,
        }, spheres);
        spheres -= &RECIPES[0];
        assert_eq!(Spheres::default(), spheres);
    }

    #[test]
    fn it_balances_simple() {
        let mut spheres = Spheres::default();
        spheres -= &RECIPES[6];
        spheres -= &RECIPES[2];
        assert!(spheres.balance(2));
    }

    #[test]
    fn it_stops_if_no_result() {
        let mut spheres = Spheres::default();
        spheres -= &RECIPES[6];
        spheres -= &RECIPES[2];
        assert!(!spheres.balance(1));
    }
}
