use Direction::*;
use Node::*;

pub type Tree = Box<Node>;

#[derive(Clone, Debug)]
pub enum Node {
    Leaf(u32),
    Branch(Tree, Tree),
}

impl Node {
    pub fn is_leaf(&self) -> bool {
        matches!(self, Leaf(_))
    }

    pub fn is_branch(&self) -> bool {
        matches!(self, Branch(_, _))
    }
}

#[derive(Clone, Debug)]
pub struct Zipper {
    ctx: Vec<(Direction, Tree)>,
    focus: Tree,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
}

impl Zipper {
    /// Returns the depth of the current focus.
    pub fn depth(&self) -> usize {
        self.ctx.len()
    }

    /// Indicates whether the top of the tree is currently focused.
    pub fn at_top(&self) -> bool {
        self.depth() == 0
    }

    /// Returns a shared reference the current focus.
    pub fn focus(&self) -> &Tree {
        &self.focus
    }

    /// Returns a mutable reference the current focus.
    pub fn focus_mut(&mut self) -> &mut Tree {
        &mut self.focus
    }

    /// Focuses the immediate left child of the current focus.
    ///
    /// Does nothing if the current focus is a leaf.
    pub fn focus_down_left(&mut self) {
        crate::take(&mut self.focus, |focus| match focus {
            box Branch(l, r) => {
                self.ctx.push((Left, r));
                l
            }
            _ => focus,
        });
    }

    /// Focuses the immediate right child of the current focus.
    ///
    /// Does nothing if the current focus is a leaf.
    pub fn focus_down_right(&mut self) {
        crate::take(&mut self.focus, |focus| match focus {
            box Branch(l, r) => {
                self.ctx.push((Right, l));
                r
            }
            _ => focus,
        });
    }

    /// Moves the focus up, if possible.
    ///
    /// Returns mutable references to the children of the new focus,
    /// or `None` if the focus is unchanged (i.e., the focus was
    /// already the top node).
    pub fn focus_up(&mut self) -> Option<(&mut Tree, &mut Tree)> {
        self.ctx.pop().map(|p| {
            crate::take(&mut self.focus, |focus| match p {
                (Left, right) => box Branch(focus, right),
                (Right, left) => box Branch(left, focus),
            });
            match self.focus {
                box Branch(ref mut l, ref mut r) => (l, r),
                _ => unreachable!(),
            }
        })
    }

    /// Focuses the top of the tree.
    pub fn focus_top(&mut self) {
        while !self.at_top() {
            self.focus_up();
        }
    }

    /// Focus the next part of the tree according to a depth-first
    /// traversal.
    pub fn focus_next_depth_first(&mut self) {
        if self.focus.is_branch() {
            self.focus_down_left();
            return;
        }
        loop {
            match self.ctx.last() {
                None => break,
                Some((Left, _)) => {
                    self.focus_up();
                    self.focus_down_right();
                    break;
                }
                Some((Right, _)) => {
                    self.focus_up();
                }
            }
        }
    }

    /// Focus the next leaf according to a depth-first traversal,
    /// returning a mutable reference to its contents.
    pub fn focus_next_leaf_depth_first(&mut self) -> Option<&mut u32> {
        loop {
            self.focus_next_depth_first();
            match self.focus {
                box Leaf(ref mut n) => return Some(n),
                _ if self.at_top() => return None,
                _ => (),
            }
        }
    }

    /// Focuses the leftmost child of the current focus, returning a
    /// mutable reference to its contents.
    pub fn focus_down_left_leaf(&mut self) -> &mut u32 {
        loop {
            match self.focus {
                box Leaf(ref mut n) => break n,
                _ => self.focus_down_left(),
            }
        }
    }

    /// Focuses the rightmost child of the current focus, returning a
    /// mutable reference to its contents.
    pub fn focus_down_right_leaf(&mut self) -> &mut u32 {
        loop {
            match self.focus {
                box Leaf(ref mut n) => break n,
                _ => self.focus_down_right(),
            }
        }
    }

    /// Focuses the next leaf to the left of the current focus,
    /// returning a mutable reference to its contents.
    ///
    /// If there is no such leaf, focuses the top of the tree and
    /// returns `None`.
    pub fn focus_next_leaf_left(&mut self) -> Option<&mut u32> {
        let flag = crate::take_return(&mut self.focus, |mut focus| {
            let flag = loop {
                match self.ctx.pop() {
                    Some((Left, right)) => focus = box Branch(focus, right),
                    Some((Right, left)) => {
                        focus = box Branch(left, focus);
                        break true;
                    }
                    None => break false,
                }
            };
            (flag, focus)
        });
        if flag {
            self.focus_down_left();
            Some(self.focus_down_right_leaf())
        } else {
            None
        }
    }

    /// Focuses the next leaf to the right of the current focus,
    /// returning a mutable reference to its contents.
    ///
    /// If there is no such leaf, focuses the top of the tree and
    /// returns `None`.
    pub fn focus_next_leaf_right(&mut self) -> Option<&mut u32> {
        let flag = crate::take_return(&mut self.focus, |mut focus| {
            let flag = loop {
                match self.ctx.pop() {
                    Some((Right, left)) => focus = box Branch(left, focus),
                    Some((Left, right)) => {
                        focus = box Branch(focus, right);
                        break true;
                    }
                    None => break false,
                }
            };
            (flag, focus)
        });
        if flag {
            self.focus_down_right();
            Some(self.focus_down_left_leaf())
        } else {
            None
        }
    }

    /// Returns a mutable reference to the next leaf to the left of
    /// the current focus, or `None` if there is no such leaf.
    pub fn left_leaf_mut(&mut self) -> Option<&mut u32> {
        let mut it = self.ctx.iter_mut().rev();
        let mut root = loop {
            match it.next() {
                None => return None,
                Some((Right, left)) => break left,
                _ => (),
            }
        };
        loop {
            match root.as_mut() {
                Leaf(n) => break Some(n),
                Branch(_, r) => root = r,
            }
        }
    }

    /// Returns a mutable reference to the next leaf to the right of
    /// the current focus, or `None` if there is no such leaf.
    pub fn right_leaf_mut(&mut self) -> Option<&mut u32> {
        let mut it = self.ctx.iter_mut().rev();
        let mut root = loop {
            match it.next() {
                None => return None,
                Some((Left, right)) => break right,
                _ => (),
            }
        };
        loop {
            match root.as_mut() {
                Leaf(n) => break Some(n),
                Branch(l, _) => root = l,
            }
        }
    }
}

impl From<Tree> for Zipper {
    fn from(tree: Tree) -> Self {
        Zipper {
            ctx: Vec::new(),
            focus: tree,
        }
    }
}

impl From<Zipper> for Tree {
    fn from(zipper: Zipper) -> Self {
        let mut ret = zipper.focus;
        for (dir, tree) in zipper.ctx.into_iter().rev() {
            match dir {
                Left => ret = box Branch(ret, tree),
                Right => ret = box Branch(tree, ret),
            }
        }
        ret
    }
}
