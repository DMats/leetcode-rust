/// A tree is an undirected graph in which any two vertices are connected by
/// exactly one path. In other words, any connected graph without simple cycles
/// is a tree.
///
/// Given a tree of n nodes labelled from 0 to n - 1, and an array of n - 1
/// edges where edges[i] = [ai, bi] indicates that there is an undirected edge
/// between the two nodes ai and bi in the tree, you can choose any node of the
/// tree as the root. When you select a node x as the root, the result tree has
/// height h. Among all possible rooted trees, those with minimum height (i.e.
/// min(h))  are called minimum height trees (MHTs).
///
/// Return a list of all MHTs' root labels. You can return the answer in any
/// order.
///
/// The height of a rooted tree is the number of edges on the longest downward
/// path between the root and a leaf.
///
/// Constraints:
///
/// * 1 <= n <= 2 * 104
/// * edges.length == n - 1
/// * 0 <= ai, bi < n
/// * ai != bi
/// * All the pairs (ai, bi) are distinct.
/// * The given input is guaranteed to be a tree and there will be no repeated
///   edges.
pub struct Solution;
impl Solution {
    #[allow(unused_variables)]
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        return Vec::new();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn star() {
        assert_eq!(
            vec![1],
            Solution::find_min_height_trees(4, vec![vec![1, 0], vec![1, 2], vec![1, 3]])
        );
    }

    #[test]
    fn arm() {
        assert_eq!(
            vec![1],
            Solution::find_min_height_trees(
                6,
                vec![vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 4], vec![5, 4]]
            )
        );
    }

    #[test]
    fn point() {
        assert_eq!(vec![0], Solution::find_min_height_trees(1, vec![]));
    }

    #[test]
    fn line() {
        assert_eq!(
            vec![0, 1],
            Solution::find_min_height_trees(2, vec![vec![1, 0]])
        );
    }
}
