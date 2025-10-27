/// Equivalence Graph properties
/// 1. x Connected to itself
/// 2. if x is connected to y means y is connected to x
/// 3. if x is connected to y means y is connected to z means x is connected to z
///
/// Given connectivity between nodes
/// connectivity ==> <--->
/// x <---> y, x <---> a, x <---> b, t <---> y, u <---> y, r <---> g, g <---> t
/// Need to find whether two nodes are connected or not.
/// Like, is x connected to g
///
/// This problem is related to Disjoint sets
///
///
pub struct UnionFind {
    data: Vec<usize>,
}

pub struct UFLazy(UnionFind);

impl UFLazy {
    pub fn union(&mut self, n1: usize, n2: usize) {}

    pub fn find(&self, n1: usize, n2: usize) -> bool {
        true
    }
}
