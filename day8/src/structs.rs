#[derive(Debug, Clone)]
pub struct Coordinates {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Clone)]
pub struct Tree {
    pub height: i32,
    pub coords: Coordinates,
}

#[derive(Debug, Clone)]
pub struct Forrest {
    pub trees: Vec<Vec<Tree>>,
    edge: Coordinates,
}

impl Tree {
    pub fn new(height: i32, row: i64, col: i64) -> Tree {
        Tree {
            height,
            coords: Coordinates { x: col, y: row },
        }
    }
}

impl Forrest {
    pub fn new(trees: Vec<Vec<Tree>>) -> Forrest {
        let edge_x = trees.get(0).unwrap().len() as i64;
        let edge_y = trees.len() as i64;
        Forrest {
            trees,
            edge: Coordinates {
                x: edge_x - 1, // zero-indexed
                y: edge_y - 1, // zero-indexed
            },
        }
    }

    pub fn is_edge(&self, tree: Tree) -> bool {
        println!(
            "{:?} {:?} - {:?} {:?}",
            tree.coords.x, tree.coords.y, self.edge.x, self.edge.y
        );
        tree.coords.x <= 0
            || tree.coords.y <= 0
            || tree.coords.x >= self.edge.x
            || tree.coords.y >= self.edge.y
    }
}

#[cfg(test)]
mod tests {

    use super::{Forrest, Tree};

    #[test]
    fn test_is_edge() {
        // single tree
        let tree = Tree::new(1, 0, 0);
        let forrest = Forrest::new(vec![vec![tree.clone()]]);
        assert!(forrest.is_edge(tree.clone()));

        // 2x2 trees
        let tree1a = Tree::new(1, 0, 0);
        let tree1b = Tree::new(1, 0, 1);
        let tree2a = Tree::new(1, 1, 0);
        let tree2b = Tree::new(1, 1, 1);
        let forrest = Forrest::new(vec![
            vec![tree1a.clone(), tree1b.clone()],
            vec![tree2a.clone(), tree2b.clone()],
        ]);
        assert!(forrest.is_edge(tree1a.clone()));
        assert!(forrest.is_edge(tree1b.clone()));
        assert!(forrest.is_edge(tree2a.clone()));
        assert!(forrest.is_edge(tree2b.clone()));

        // 3x3 trees
        let tree1a = Tree::new(1, 0, 0);
        let tree1b = Tree::new(1, 0, 1);
        let tree1c = Tree::new(1, 0, 2);
        let tree2a = Tree::new(1, 1, 0);
        let tree2b = Tree::new(1, 1, 1);
        let tree2c = Tree::new(1, 1, 2);
        let tree3a = Tree::new(1, 2, 0);
        let tree3b = Tree::new(1, 2, 1);
        let tree3c = Tree::new(1, 2, 2);
        let forrest = Forrest::new(vec![
            vec![tree1a.clone(), tree1b.clone(), tree1c.clone()],
            vec![tree2a.clone(), tree2b.clone(), tree2c.clone()],
            vec![tree3a.clone(), tree3b.clone(), tree3c.clone()],
        ]);
        assert!(forrest.is_edge(tree1a.clone()));
        assert!(forrest.is_edge(tree1b.clone()));
        assert!(forrest.is_edge(tree1c.clone()));
        assert!(forrest.is_edge(tree2a.clone()));
        assert!(!forrest.is_edge(tree2b.clone()));
        assert!(forrest.is_edge(tree2c.clone()));
        assert!(forrest.is_edge(tree3a.clone()));
        assert!(forrest.is_edge(tree3b.clone()));
        assert!(forrest.is_edge(tree3c.clone()));
    }
}
