use num_derive::FromPrimitive;

use crate::spheres::Spheres;

#[derive(Debug, FromPrimitive, Copy, Clone)]
pub enum ProductRecipes {
    NaquiumTesseract1 = 0,
    NaquiumTesseract2 = 1,
    WormholeData = 2,
    NaquiumProcessor1 = 3,
    NaquiumProcessor2 = 4,
    SpaceDilation1 = 5,
    SpaceDilation2 = 6,
    SpaceFolding1 = 7,
    SpaceFolding2 = 8,
    SpaceInjection1 = 9,
    SpaceInjection2 = 10,
    SpaceWarping1 = 11,
    SpaceWarping2 = 12,
    Singularity1 = 13,
    Singularity2 = 14,
}

pub static PRODUCTS: [Spheres; 15] = [
    Spheres {
        // NaquiumTesseract1
        zeta: -1,
        theta: 1,
        gamma: 0,
        omega: 0,
        lambda: -1,
        xi: -1,
        epsilon: 1,
        phi: 1,
    },
    Spheres {
        // NaquiumTesseract2
        zeta: -1,
        theta: 0,
        gamma: 1,
        omega: 1,
        lambda: -1,
        xi: -1,
        epsilon: 0,
        phi: 1,
    },
    Spheres {
        // WormholeData
        zeta: -1,
        theta: 1,
        gamma: -1,
        omega: 1,
        lambda: -1,
        xi: 1,
        epsilon: -1,
        phi: 1,
    },
    Spheres {
        // Processor1
        zeta: -1,
        theta: -1,
        gamma: -1,
        omega: -1,
        lambda: 5,
        xi: 1,
        epsilon: -1,
        phi: -1,
    },
    Spheres {
        // Processor2
        zeta: -1,
        theta: -1,
        gamma: -1,
        omega: -1,
        lambda: 1,
        xi: 5,
        epsilon: -1,
        phi: -1,
    },
    Spheres {
        // SpaceDilation1
        zeta: -1,
        theta: 0,
        gamma: 0,
        omega: -1,
        lambda: 2,
        xi: 0,
        epsilon: 0,
        phi: 0,
    },
    Spheres {
        // SpaceDilation2
        zeta: -1,
        theta: 0,
        gamma: 0,
        omega: -1,
        lambda: 0,
        xi: 0,
        epsilon: 0,
        phi: 2,
    },
    Spheres {
        // SpaceFolding1
        zeta: 1,
        theta: 1,
        gamma: 0,
        omega: 0,
        lambda: -1,
        xi: -1,
        epsilon: 0,
        phi: 0,
    },
    Spheres {
        // SpaceFolding2
        zeta: 0,
        theta: 0,
        gamma: 0,
        omega: 0,
        lambda: -1,
        xi: -1,
        epsilon: 1,
        phi: 1,
    },
    Spheres {
        // SpaceInjection1
        zeta: 0,
        theta: -1,
        gamma: -1,
        omega: 0,
        lambda: 0,
        xi: 0,
        epsilon: 2,
        phi: 0,
    },
    Spheres {
        // SpaceInjection2
        zeta: 2,
        theta: -1,
        gamma: -1,
        omega: 0,
        lambda: 0,
        xi: 0,
        epsilon: 0,
        phi: 0,
    },
    Spheres {
        // SpaceWarping1
        zeta: 1,
        theta: 1,
        gamma: 0,
        omega: 0,
        lambda: 0,
        xi: 0,
        epsilon: -1,
        phi: -1,
    },
    Spheres {
        // SpaceWarping2
        zeta: 0,
        theta: 0,
        gamma: 1,
        omega: 1,
        lambda: 0,
        xi: 0,
        epsilon: -1,
        phi: -1,
    },
    Spheres {
        // Singularity1
        zeta: 1,
        theta: -1,
        gamma: 1,
        omega: -1,
        lambda: 0,
        xi: 0,
        epsilon: 0,
        phi: 0,
    },
    Spheres {
        // Singularity2
        zeta: 0,
        theta: 0,
        gamma: 0,
        omega: 0,
        lambda: -1,
        xi: 1,
        epsilon: -1,
        phi: 1,
    },
];
