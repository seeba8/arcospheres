use std::ops::{Add, AddAssign, MulAssign, SubAssign};

use crate::recipes::{FOLDING_RECIPES, INVERSION_RECIPES};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
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
    pub fn balance(&mut self, depth: u8, multiplier: i16) -> bool {
        // let start = Instant::now();
        // let result;
        // if depth > 20 {
        //     println!("Spawning threads");
        //     let mut handles = vec![];
        //     let success = Arc::new(Mutex::new(false));

        //     for (i, recipe) in FOLDING_RECIPES.iter().enumerate() {
        //         let mut cloned = self.clone();
        //         let success = Arc::clone(&success);
        //         handles.push(std::thread::spawn(move || {
        //             let local_start = Instant::now();
        //             let mut path = vec![];
        //             path.push(i as u8);
        //             cloned.add_assign(recipe);
        //             let mut res = success.lock().unwrap();
        //             *res |= cloned._balance(depth - 1, &mut path, i);
        //             println!("Time elapsed in thread {i}: {:?}", local_start.elapsed());
        //         }));
        //     }
        //     for handle in handles {
        //         handle.join().unwrap();
        //     }
        //     result = *success.lock().unwrap();
        // } else {

        if (self.zeta + self.theta + self.gamma + self.omega) % 4 != 0
            || (self.lambda + self.xi + self.epsilon + self.phi) % 4 != 0
        {
            return false;
        }

        let mut num_inversions = 0;
        while self.zeta + self.theta + self.gamma + self.omega
            != self.lambda + self.xi + self.epsilon + self.phi
        {
            match (self.zeta + self.theta + self.gamma + self.omega)
                .cmp(&(self.lambda + self.xi + self.epsilon + self.phi))
            {
                std::cmp::Ordering::Greater => {
                    num_inversions += 1;
                    self.add_assign(&INVERSION_RECIPES[0]);
                }
                std::cmp::Ordering::Less => {
                    num_inversions += 1;
                    self.add_assign(&INVERSION_RECIPES[1]);
                }
                std::cmp::Ordering::Equal => (),
            }
        }

        let mut path = vec![];
        let result = self._balance(depth, &mut path, 0);
        if result {
            println!(
                "Multiplier: {multiplier}, depth: {depth}, inversions: {num_inversions}. Path: {:?}",
                path.iter().map(|step| step + 1).collect::<Vec<u8>>()
            );
        }
        // }
        // println!("Total time elapsed: {:?}", start.elapsed());
        result
    }

    /// If I understand the logic correctly (thanks to [reddit](https://old.reddit.com/r/factorio/comments/qyb52b/a_robotfree_perfectly_balanced_arcosphere_approach/))
    /// the balancing steps can be swapped around. Thus, we can ignore all recipes with a smaller ID (since they will be picked up in that respective branch)
    ///
    /// For example: 1,1,2,3 and 3,2,1,1 result in the same set of spheres
    fn _balance(&mut self, depth: u8, path: &mut Vec<u8>, first_available_recipe: usize) -> bool {
        if self.is_balanced() {
            return true;
        }
        if depth == 0 {
            return false;
        }
        if [0, 1, 4, 5].iter().all(|v| path.contains(v))
            || [2, 3, 6, 7].iter().all(|v| path.contains(v))
        {
            // we reached a loop, so we can return. It's no longer the shortest path
            return false;
        }

        // if self.zeta + self.theta + self.gamma + self.omega
        //     != self.lambda + self.xi + self.epsilon + self.phi
        // {
        //     return false;
        // }

        for (i, recipe) in FOLDING_RECIPES
            .iter()
            .enumerate()
            .skip(first_available_recipe)
        {
            self.add_assign(recipe);
            path.push(i.try_into().unwrap());
            if self._balance(depth - 1, path, i) {
                return true;
            }
            self.sub_assign(recipe);
            path.pop();
        }
        false
    }

    #[must_use = "pedantic clippy says so"]
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

impl Add<&Spheres> for Spheres {
    type Output = Spheres;

    fn add(self, rhs: &Spheres) -> Self::Output {
        Spheres {
            zeta: self.zeta + rhs.zeta,
            theta: self.theta + rhs.theta,
            gamma: self.gamma + rhs.gamma,
            omega: self.omega + rhs.omega,
            lambda: self.lambda + rhs.lambda,
            xi: self.xi + rhs.xi,
            epsilon: self.epsilon + rhs.epsilon,
            phi: self.phi + rhs.phi,
        }
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

impl MulAssign<i16> for Spheres {
    fn mul_assign(&mut self, rhs: i16) {
        self.zeta *= rhs;
        self.theta *= rhs;
        self.gamma *= rhs;
        self.omega *= rhs;
        self.lambda *= rhs;
        self.xi *= rhs;
        self.epsilon *= rhs;
        self.phi *= rhs;
    }
}

#[cfg(test)]
mod tests {
    use crate::{recipes::FOLDING_RECIPES, spheres::Spheres};

    #[test]
    fn it_applies_recipe() {
        let mut spheres = Spheres::default();
        spheres += &FOLDING_RECIPES[0];
        assert_eq!(
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
            spheres
        );

        spheres += &FOLDING_RECIPES[0];
        assert_eq!(
            Spheres {
                zeta: 0,
                theta: 2,
                gamma: 0,
                omega: -2,
                lambda: -2,
                xi: 2,
                epsilon: 0,
                phi: 0,
            },
            spheres
        );

        spheres -= &FOLDING_RECIPES[0];
        assert_eq!(
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
            spheres
        );
        spheres -= &FOLDING_RECIPES[0];
        assert_eq!(Spheres::default(), spheres);
    }

    #[test]
    fn it_balances_simple() {
        let mut spheres = Spheres::default();
        spheres -= &FOLDING_RECIPES[6];
        spheres -= &FOLDING_RECIPES[2];
        spheres.balance(2, 0);
    }

    #[test]
    fn it_stops_if_no_result() {
        let mut spheres = Spheres::default();
        spheres -= &FOLDING_RECIPES[6];
        spheres -= &FOLDING_RECIPES[2];
        spheres.balance(1, 0);
    }
}
