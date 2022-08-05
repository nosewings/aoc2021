// NOTE: This is optimized *to hell*.  On my home PC, this solution
// takes ~70 ms to solve Part Two, which is a better than anything I
// saw on reddit.
//
// How did I achieve this?
//
// 1. Smart move choices/pruning.  If a piece can move into a room,
// either from a room or from the hall, then we always take that move
// and no others, since there simply cannot be a better move.
//
// 2. A very tight heuristic function.  My heuristic function
// estimates the cost for the initial state of Part Two at 46864, vs
// an actual cost of 47484.  This is an underestimate of 620, or
// ~0.01%.
//
// 3. Rather than re-running the entire heuristic function at every
// step, we just return the delta along with the new state.
//
// I also eliminated allocations by using arrays and ArrayVecs for
// everything.  I'm not sure that this made much of a difference,
// though.
//
// It is possible that the runtime could be further improved by making
// move selection even smarter, but that would be quite complicated,
// and I'm not sure how to do it.

mod hall;
mod params;
mod room;

use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

use combine::stream::ResetStream;
use combine::{Parser, Positioned};

use hall::Hall;
use params::Params;
use room::Room;
use room::Rooms;
use std::iter;

use crate::triangular;

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct State<const NUM_ROOMS: usize, const ROOM_SIZE: usize, const HALL_SIZE: usize> {
    rooms: Rooms<NUM_ROOMS, ROOM_SIZE>,
    hall: Hall<HALL_SIZE>,
}

impl<const NUM_ROOMS: usize, const ROOM_SIZE: usize, const HALL_SIZE: usize>
    State<NUM_ROOMS, ROOM_SIZE, HALL_SIZE>
{
    fn from_cols(cols: &[Vec<u32>]) -> Self {
        Self {
            rooms: Rooms::from_cols(cols),
            hall: Hall::new(),
        }
    }

    pub fn solve(self, params: &Params) -> u32 {
        let mut open = BinaryHeap::new();
        let mut closed = HashSet::new();
        let x = StateBundle::new(self, 0, params);
        open.push(x);
        loop {
            let x = open.pop().unwrap();
            if x.state.is_solved(params) {
                break x.cost_so_far
            }
            for future in x.futures(params) {
                if !closed.contains(&x.state) {
                    open.push(future);
                }
            }
            closed.insert(x.state);
        }
    }

    fn is_solved(&self, params: &Params) -> bool {
        self.rooms.rooms.iter().all(|r| match r {
            &Room::Open(n) => n == params.height,
            _ => false,
        })
    }

    /// List of states one move in the future from this state,
    /// together with their one-move costs.
    fn futures<'a>(&'a self, params: &'a Params) -> impl Iterator<Item = (Self, u32, u32)> + 'a {
        // If any piece in the hall can immediately move into its
        // destination room, then just do that; there cannot be any
        // better move.
        for (hall_ix, tile) in self.hall.tiles.iter().enumerate() {
            let hall_ix = hall_ix as u32;
            if let &Some(piece) = tile {
                let hall_room_ix = self.hall.room_to_hall_index(piece);
                let (start, end) = [hall_ix, hall_room_ix]
                    .into_iter()
                    .minmax()
                    .into_option()
                    .unwrap();
                let start = start + 1;
                // Make sure that the path from the piece to its
                // room is clear.
                if self.hall.tiles[start as usize..end as usize]
                    .iter()
                    .all(|tile| tile.is_none())
                {
                    if let Some((dist2, room)) = self.rooms.rooms[piece as usize].push(params) {
                        let dist1 = self.hall.room_to_hall_index(piece).abs_diff(hall_ix);
                        let cost = params.get_coeff(piece) * (dist1 + dist2);
                        return itertools::Either::Right(itertools::Either::Left(iter::once((
                            Self {
                                rooms: self.rooms.clone().set(piece, room),
                                hall: self.hall.clone().set(hall_ix, None),
                            },
                            cost,
                            0,
                        ))));
                    }
                }
            }
        }

        // If any piece in a room can immediately move into its
        // destination room, then just do that; there cannot be any
        // better move.
        for (room_ix, room) in self.rooms.rooms.iter().enumerate() {
            let room_ix = room_ix as u32;
            let hall_room_ix = self.hall.room_to_hall_index(room_ix);
            if let Some((piece, dist1, room)) = room.pop(params) {
                if let Some((dist3, room2)) = self.rooms.rooms[piece as usize].push(params) {
                    let hall_room_ix_dst = self.hall.room_to_hall_index(piece);
                    let (start, end) = [hall_room_ix, hall_room_ix_dst]
                        .into_iter()
                        .minmax()
                        .into_option()
                        .unwrap();
                    let start = start + 1;
                    if self.hall.tiles[start as usize..end as usize]
                        .iter()
                        .all(|x| x.is_none())
                    {
                        let dist2 = hall_room_ix_dst.abs_diff(hall_room_ix);
                        return itertools::Either::Left(iter::once((
                            Self {
                                rooms: self.rooms.clone().set(room_ix, room).set(piece, room2),
                                hall: self.hall.clone(),
                            },
                            params.get_coeff(piece) * (dist1 + dist2 + dist3),
                            0,
                        )));
                    }
                }
            }
        }

        // Otherwise, move pieces from rooms into the hall.
        itertools::Either::Right(itertools::Either::Right(
            self.rooms
                .rooms
                .iter()
                .enumerate()
                .flat_map(move |(room_ix, room)| {
                    let room_ix = room_ix as u32;
                    let hall_room_ix = self.hall.room_to_hall_index(room_ix);
                    room.pop(params)
                        .into_iter()
                        .flat_map(move |(piece, dist1, room)| {
                            self.hall
                                .tiles
                                .iter()
                                .enumerate()
                                .take(hall_room_ix as usize)
                                .rev()
                                .filter(|&(i, _)| !self.hall.is_room_index(i as u32))
                                .take_while(|&(i, _)| self.hall.tiles[i].is_none())
                                .chain(
                                    self.hall
                                        .tiles
                                        .iter()
                                        .enumerate()
                                        .skip(hall_room_ix as usize + 1)
                                        .filter(|&(i, _)| !self.hall.is_room_index(i as u32))
                                        .take_while(|&(i, _)| self.hall.tiles[i].is_none()),
                                )
                                .map(move |(hall_ix, _)| {
                                    let hall_ix = hall_ix as u32;
                                    let dist2 = hall_ix.abs_diff(hall_room_ix);
                                    let hall_room_ix_dst = self.hall.room_to_hall_index(piece);
                                    let est_cost_change = params.get_coeff(piece)
                                        * if piece == room_ix {
                                            dist2 - 1
                                        } else if hall_room_ix.cmp(&hall_ix)
                                            != hall_ix.cmp(&hall_room_ix_dst)
                                        {
                                            dist2.min(hall_ix.abs_diff(hall_room_ix_dst))
                                        } else {
                                            0
                                        };
                                    (
                                        Self {
                                            rooms: self.rooms.clone().set(room_ix, room.clone()),
                                            hall: self.hall.clone().set(hall_ix, Some(piece)),
                                        },
                                        params.get_coeff(piece) * (dist1 + dist2),
                                        est_cost_change,
                                    )
                                })
                        })
                }),
        ))
    }

    /// Estimated cost to solve this state.
    fn cost_est(&self, params: &Params) -> u32 {
        // First, consider the costs associated with rooms.
        self.rooms
            .rooms
            .iter()
            .enumerate()
            .map(|(room_ix, room)| {
                let room_ix = room_ix as u32;
                let hall_room_ix = self.hall.room_to_hall_index(room_ix);
                match room {
                    Room::Closed(n, xs) => {
                        xs.iter()
                            .rev()
                            .enumerate()
                            .map(|(dist_to_top, &piece)| {
                                let dist_out = dist_to_top as u32 + 1;
                                params.get_coeff(piece)
                                    * if piece == room_ix {
                                        // If the piece is in its
                                        // destination room, then it
                                        // needs to walk out, move
                                        // over at least once space,
                                        // and then later walk back.
                                        dist_out + 2
                                    } else {
                                        // Otherwise, it needs to walk
                                        // out and then walk to its
                                        // room.
                                        dist_out
                                            + self.hall.room_to_hall_index(piece).abs_diff(hall_room_ix)
                                    }
                            })
                            .sum::<u32>()
                            // Also, all the missing pieces need to
                            // find their place in the room.
                            + params.get_coeff(room_ix) * triangular(params.height - n)
                    }
                    &Room::Open(n) => params.get_coeff(room_ix) * triangular(params.height - n),
                }
            })
            // Second, consider the costs associated with the hall.
            .chain(
                self.hall
                    .tiles
                    .iter()
                    .enumerate()
                    .flat_map(|(hall_ix, tile)| {
                        let hall_ix = hall_ix as u32;
                        tile.map(|piece| {
                            params.get_coeff(piece)
                                * self.hall.room_to_hall_index(piece).abs_diff(hall_ix)
                        })
                    }),
            )
            .sum()
    }

    /// Pretty-print a state.
    #[allow(unstable_name_collisions)]
    pub fn pp(&self, params: &Params) -> String {
        fn tile_to_char(tile: &Option<u32>) -> char {
            tile.map_or('.', |piece| (piece + 'A' as u32).try_into().unwrap())
        }

        iter::repeat('#')
            .take(2 * params.width as usize + 5)
            .chain("\n#".chars())
            .chain(self.hall.tiles.iter().map(tile_to_char))
            .chain("#\n".chars())
            .chain((0..params.height).rev().flat_map(|i| {
                (if i == params.height - 1 {
                    "###".chars()
                } else {
                    "  #".chars()
                })
                .chain(
                    (0..params.width)
                        .map(move |j| tile_to_char(&self.rooms.rooms[j as usize].get(j, i)))
                        .intersperse('#'),
                )
                .chain(if i == params.height - 1 {
                    "###\n".chars()
                } else {
                    "#\n".chars()
                })
            }))
            .chain("  ".chars())
            .chain(iter::repeat('#').take(2 * params.width as usize + 1))
            .collect()
    }
}

/// A state bundled together with its exact cost so far and its
/// estimated total cost.  States are ordered first by their total
/// cost estimates, and then by their costs so far (in reverse order,
/// since Rust's `BinaryHeap` is a max-heap, and we're trying to
/// minimize the cost).
#[derive(PartialEq, Eq, Clone, Hash, Debug)]
struct StateBundle<const NUM_ROOMS: usize, const ROOM_SIZE: usize, const HALL_SIZE: usize> {
    state: State<NUM_ROOMS, ROOM_SIZE, HALL_SIZE>,
    cost_so_far: u32,
    total_cost_est: u32,
}

impl<const NUM_ROOMS: usize, const ROOM_SIZE: usize, const HALL_SIZE: usize> PartialOrd
    for StateBundle<NUM_ROOMS, ROOM_SIZE, HALL_SIZE>
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const NUM_ROOMS: usize, const ROOM_SIZE: usize, const HALL_SIZE: usize> Ord
    for StateBundle<NUM_ROOMS, ROOM_SIZE, HALL_SIZE>
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.total_cost_est.cmp(&other.total_cost_est).reverse()
    }
}

impl<const NUM_ROOMS: usize, const ROOM_SIZE: usize, const HALL_SIZE: usize>
    StateBundle<NUM_ROOMS, ROOM_SIZE, HALL_SIZE>
{
    /// Create a new state bundle from a given state and total cost so
    /// far.
    fn new(
        state: State<NUM_ROOMS, ROOM_SIZE, HALL_SIZE>,
        cost_so_far: u32,
        params: &Params,
    ) -> Self {
        let est = state.cost_est(params);
        Self {
            state,
            cost_so_far,
            total_cost_est: cost_so_far + est,
        }
    }

    /// List of state bundles one move in the future from this state.
    fn futures<'a>(&'a self, params: &'a Params) -> impl Iterator<Item = Self> + 'a {
        self.state
            .futures(params)
            .map(|(state, cost, est_cost_change)| Self {
                state,
                cost_so_far: self.cost_so_far + cost,
                total_cost_est: self.total_cost_est + est_cost_change,
            })
    }
}

#[derive(Debug)]
pub struct Problem<const NUM_ROOMS: usize, const ROOM_SIZE: usize, const HALL_SIZE: usize> {
    pub params: Params,
    pub init: State<NUM_ROOMS, ROOM_SIZE, HALL_SIZE>,
}

impl<const NUM_ROOMS: usize, const ROOM_SIZE: usize, const HALL_SIZE: usize>
    Problem<NUM_ROOMS, ROOM_SIZE, HALL_SIZE>
{
    pub fn from_cols(cols: &[Vec<u32>]) -> Self {
        let params = Params::new(cols.len() as u32, cols[0].len() as u32);
        let init = State::from_cols(cols);
        Self { params, init }
    }
}

pub fn parse_cols<Input>() -> impl Parser<Input, Output = Vec<Vec<u32>>>
where
    Input: ResetStream<Token = char> + Positioned + Debug,
{
    use combine::parser::char::*;
    use combine::*;

    fn skip_line<Input>() -> impl Parser<Input, Output = ()>
    where
        Input: ResetStream<Token = char> + Positioned,
    {
        skip_many(satisfy(|c| c != '\n')).skip(newline())
    }

    fn ascii_uppercase<Input>() -> impl Parser<Input, Output = char>
    where
        Input: ResetStream<Token = char> + Positioned,
    {
        satisfy(|c: char| c.is_ascii_uppercase())
    }

    fn not_ascii_uppercase<Input>() -> impl Parser<Input, Output = char>
    where
        Input: ResetStream<Token = char> + Positioned,
    {
        satisfy(|c: char| !c.is_ascii_uppercase())
    }

    fn not_newline<Input>() -> impl Parser<Input, Output = char>
    where
        Input: ResetStream<Token = char> + Positioned,
    {
        satisfy(|c: char| c != '\n')
    }

    skip_count(2, skip_line())
        .with(sep_end_by(
            attempt(
                skip_many(not_ascii_uppercase())
                    .with(sep_by1(
                        ascii_uppercase(),
                        attempt(char('#').skip(look_ahead(ascii_uppercase()))),
                    ))
                    .skip(skip_many(not_newline())),
            ),
            newline(),
        ))
        .skip(skip_line())
        .skip(eof())
        .map(|rows: Vec<Vec<char>>| {
            // Convert and transpose.
            (0..rows[0].len())
                .map(|i| {
                    rows.iter()
                        .rev()
                        .map(|row| row[i] as u32 - 'A' as u32)
                        .collect()
                })
                .collect_vec()
        })
}
