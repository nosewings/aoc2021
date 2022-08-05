use arrayvec::ArrayVec;

use super::params::Params;

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub enum Room<const ROOM_SIZE: usize> {
    /// The room is not open, because it has some pieces that do not
    /// belong.  `Closed(n, xs)` has `n` pieces that do belong at the
    /// bottom, with the pieces `xs` above them.
    Closed(u32, ArrayVec<u32, ROOM_SIZE>),
    /// The room is open.  It has `n` pieces.
    Open(u32),
}
use Room::*;

impl<const ROOM_SIZE: usize> Room<ROOM_SIZE> {
    /// Given a room id (i.e., what pieces should go in this room) and
    /// a list of inhabitants, construct a room.
    pub fn from_col(a: u32, col: &[u32]) -> Self {
        let pfx = col.iter().copied().take_while(|x| *x == a).count() as u32;
        if pfx == col.len() as u32 {
            Open(col.len() as u32)
        } else {
            Closed(pfx, col[pfx as usize..].try_into().unwrap())
        }
    }

    /// Try to remove this room's uppermost inhabitant.  If
    /// successful, return the removed inhabitant, the distance
    /// required to move it to the top square of the room and into the
    /// hall, and a new `Room` without the removed piece.
    ///
    /// This method returns `None` if the room is open.
    pub fn pop(&self, params: &Params) -> Option<(u32, u32, Self)> {
        match self {
            Room::Closed(n, xs) => {
                let &x = xs.last().unwrap();
                Some((
                    x,
                    params.height - n - xs.len() as u32 + 1,
                    if xs.len() == 1 {
                        Open(*n)
                    } else {
                        Closed(*n, xs[0..xs.len() - 1].try_into().unwrap())
                    },
                ))
            }
            _ => None,
        }
    }

    pub fn push(&self, params: &Params) -> Option<(u32, Self)> {
        match self {
            Closed(_, _) => None,
            Open(n) => Some((params.height - n, Open(n + 1))),
        }
    }

    pub fn get(&self, piece: u32, ix: u32) -> Option<u32> {
        match self {
            Closed(n, pieces) => {
                let n = *n;
                if ix < n {
                    Some(piece)
                } else {
                    pieces.get((ix - n) as usize).copied()
                }
            }
            Open(n) => {
                let n = *n;
                if ix < n {
                    Some(piece)
                } else {
                    None
                }
            }
        }
    }
}

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct Rooms<const NUM_ROOMS: usize, const ROOM_SIZE: usize> {
    pub rooms: ArrayVec<Room<ROOM_SIZE>, NUM_ROOMS>,
}

impl<const NUM_ROOMS: usize, const ROOM_SIZE: usize> Rooms<NUM_ROOMS, ROOM_SIZE> {
    pub fn from_cols(cols: &[Vec<u32>]) -> Self {
        Self {
            rooms: cols
                .iter()
                .enumerate()
                .map(|(i, c)| Room::from_col(i as u32, c))
                .collect(),
        }
    }

    pub fn set(mut self, ix: u32, room: Room<ROOM_SIZE>) -> Self {
        self.rooms[ix as usize] = room;
        self
    }
}
