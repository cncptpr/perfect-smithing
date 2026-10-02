use serde::{Deserialize, Serialize};

use crate::{matrix::Matrix, num::Num};

#[derive(Serialize, Deserialize)]
pub struct APSPResult {
    pub(crate) distance: Matrix<Num>,
    pub(crate) predecessor: Matrix<i64>,
    /// The steps the matrices were built with, used to spot stale caches.
    pub(crate) steps: Vec<i64>,
}

impl APSPResult {
    pub fn path(&self, from: i64, mut to: i64) -> Vec<i64> {
        if self.predecessor.get_expect(from, to) == -1 {
            return vec![];
        }

        let mut path = vec![to];
        while from != to {
            to = self.predecessor.get_expect(from, to);
            path.insert(0, to);
        }
        path
    }

    pub fn dist(&self, from: i64, to: i64) -> Num {
        self.distance.get_expect(from, to)
    }

    /// The highest position the matrices cover.
    pub fn max_pos(&self) -> i64 {
        self.distance.size as i64 - 1
    }
}

pub fn floyd_warshall(node_count: i64, trasition_steps: Vec<i64>) -> APSPResult {
    assert!(node_count > 0);
    let mut distance = Matrix::new(node_count as usize, Num::Inf);
    let mut predecessor = Matrix::new(node_count as usize, -1);

    // Set diagonals
    for i in 0..node_count {
        distance.set(i, i, 0);
        predecessor.set(i, i, i);
    }

    // Fill starting weights
    for i in 0..node_count {
        for step in trasition_steps.iter() {
            distance.set(i, i + step, 1);
            predecessor.set(i, i + step, i);
        }
    }

    // Basic Floyd-Warshall
    for node in 0..node_count {
        for i in 0..node_count {
            for j in 0..node_count {
                let current_distance = distance.get_expect(i, j);
                let distance_over_node =
                    distance.get_expect(i, node) + distance.get_expect(node, j);

                // Cannot be inverted,
                // since any comparison with two Inf is always false,
                // and we don't want to update if both are Inf.
                // Thanks Math!
                if current_distance > distance_over_node {
                    distance.set(i, j, distance_over_node);

                    let prev = predecessor.get_expect(node, j);
                    predecessor.set(i, j, prev);
                }
            }
        }
    }

    APSPResult {
        distance,
        predecessor,
        steps: trasition_steps,
    }
}
