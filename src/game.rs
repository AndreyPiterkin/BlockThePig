//use std::{collections::{HashMap, VecDeque}, cmp::Ordering};
//
//
//// A Game is a 2D board of hexagons, some of which are movable tiles and some of which are walls
//// (tiles you can't move to), and a pig location. 
//pub struct Game {
//    board: Vec<Vec<Cell>>,
//    pig: Pig<ClassicTopLeftPrefStrategy>
//}
//
//// An OpenCell is a cell that can be moved into by the pig, or have a block placed on it. An
//// OpenCell has 6 neighbors in adjacent hexagonal directions. 
//#[derive(Hash, Eq, PartialEq)]
//pub struct OpenCell {
//    top_left_neighbor: Option<Pos2D>,
//    top_right_neighbor: Option<Pos2D>,
//    right_neighbor: Option<Pos2D>,
//    bot_right_neighbor: Option<Pos2D>,
//    bot_left_neighbor: Option<Pos2D>,
//    left_neighbor: Option<Pos2D>,
//    pos: Pos2D
//}
//
//#[derive(Hash, Eq, PartialEq)]
//// A Cell is one of Open and Closed. 
//pub enum Cell {
//    Open(OpenCell),
//    Closed
//}
//
//impl OpenCell {
//    fn new(r: usize, c: usize) -> Cell {
//        Cell::Open(OpenCell {
//            top_left_neighbor: Option::None,
//            top_right_neighbor: Option::None,
//            right_neighbor: Option::None,
//            bot_right_neighbor: Option::None,
//            bot_left_neighbor: Option::None,
//            left_neighbor: Option::None, 
//            pos: Pos2D::new(r, c)
//        })
//    }
//
//    fn set_top_left(&mut self, other: &OpenCell) -> () {
//        self.top_left_neighbor = Some(other.pos); 
//    }
//
//    fn set_top_right(&mut self, other: &OpenCell) -> () {
//        self.top_right_neighbor = Some(other.pos);
//    }
//
//    fn set_left(&mut self, other: &OpenCell) -> () {
//        self.left_neighbor = Some(other.pos);
//    } 
//
//    fn set_right(&mut self, other: &OpenCell) -> () {
//        self.right_neighbor = Some(other.pos);
//    }
//
//    fn set_bot_left(&mut self, other: &OpenCell) -> () {
//        self.bot_left_neighbor = Some(other.pos);
//    }
//
//    fn set_bot_right(&mut self, other: &OpenCell) -> () {
//        self.bot_right_neighbor = Some(other.pos);
//    }
//
//    fn get_neighbors_as_vec(&self) -> Vec<Option<Pos2D>> {
//        let v : Vec<Option<Pos2D>> = vec![self.top_left_neighbor, self.top_right_neighbor, self.right_neighbor, self.bot_right_neighbor, self.bot_left_neighbor, self.left_neighbor];
//        return v;
//    }
//
//    // given the previous position, computes the next possible destinations for a path which
//    // doesn't loop or go backwards
//    // LOGICAL INVARIANT: Pos2D come_from will never point to a cell that is None in the board.
//    fn next_neighbors(&self, come_from: &Pos2D) -> Vec<Pos2D> {
//        let v = self.get_neighbors_as_vec();
//        let mut res : VecDeque<Option<Pos2D>> = VecDeque::from_iter(v.into_iter());
//        res.rotate_left((res.iter().position(|x| { match x { Some(p) => p == come_from, _ => false}}).unwrap() + 2) % res.len()); 
//        res.truncate(3);
//        return Vec::from_iter(res.into_iter().flatten());
//    }
//}
//
//#[derive(Hash, Debug, Clone, Copy)]
//// A Pos2D is a (x,y) coordinate pair in 2D space, with (0,0) indicating top left.
//pub struct Pos2D {
//    x: usize,
//    y: usize
//}
//
//impl Ord for Pos2D {
//    fn cmp(&self, other: &Self) -> Ordering {
//        (self.x, self.y).cmp(&(other.x, other.y))
//    }
//}
//
//impl PartialOrd for Pos2D {
//    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//        Some(self.cmp(other))
//    }
//}
//
//impl PartialEq for Pos2D {
//    fn eq(&self, other: &Self) -> bool {
//        self.x == other.x && self.y == other.y
//    }
//}
//
//impl Eq for Pos2D {
//
//}
//
//impl Pos2D {
//    pub fn new(x: usize, y: usize) -> Pos2D {
//        Pos2D {
//            x: x,
//            y: y
//        }    
//    }
//
//    pub fn equals(&self, x: usize, y: usize) -> bool {
//        return (x == self.x) && (y == self.y)
//    }
//
//    pub fn unwrap(&self) -> (usize, usize) {
//        return (self.x, self.y)
//    }
//
//}
//
//trait PigMoveStrategy {
//    fn compute_move(&self, cur_pos: &Pos2D, board: &Vec<Vec<Cell>>) -> Vec<Vec<Cell>>;
//}
//
//struct Pig<S: PigMoveStrategy> {
//    pos: Pos2D,
//    strategy: S
//}
//
//struct ClassicTopLeftPrefStrategy;
//
//impl PigMoveStrategy for ClassicTopLeftPrefStrategy {
//    fn compute_move(&self, cur_pos: &Pos2D, board: &Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
//        let mut paths : Vec<Vec<Cell>> = vec![];
//        let mut come_from : HashMap<&Cell, Vec<&Cell>> = HashMap::new();
//        let mut queue : VecDeque<(Pos2D, Pos2D)> = VecDeque::new();
//        let max_x = board.get(0).unwrap().len();
//        let max_y = board.len();
//        queue.push_back((*cur_pos, *cur_pos));
//
//        while !queue.is_empty() {
//            let (new_pos, old_pos) = queue.pop_front().unwrap();
//            let new_cell = &board[new_pos.y][new_pos.x];
//            let old_cell = &board[old_pos.y][old_pos.x];
//            if new_cell == old_cell {
//                // TODO
//            } else {
//                //TODO 
//            }
//        }
//
//        return paths;
//    }
//}
//
//impl Game {
//    pub fn new(row_count: usize, col_count: usize, pig_pos: Pos2D) -> Game {
//        let board : Vec<Vec<Cell>> = (0..row_count).map(|r| (0..col_count).map(|c| OpenCell::new(r, c)).collect()).collect();
//        Game {
//            board: board,
//            pig: Pig { pos: pig_pos, strategy: ClassicTopLeftPrefStrategy } 
//        }
//    }
//
//    pub fn classic_game() -> Game {
//        return Game::new(11, 5, Pos2D::new(5, 2));
//    }
//
//    pub fn get_cell(&self, p: &Pos2D) -> &Cell {
//        return &self.board[p.y][p.x];
//    }
//
//    pub fn get_cell_mut(&mut self, p: Pos2D) -> &mut Cell {
//        return &mut self.board[p.y][p.x];
//    }
//
//    pub fn close_cell(&mut self, r: usize, c: usize) -> Result<(), String> {
//        if r >= self.board.len() || c >= self.board.get(0).unwrap().len() {
//            return Result::Err(String::from("Out of bounds"));
//        }
//
//        if let Cell::Open(oc) = self.board.get(r).unwrap().get(c).unwrap() {
//            if self.pig.pos.equals(r, c) {
//                return Result::Err(String::from("Can't place a block on the pig"));
//            } else {
//                if let Some(tl) = self.get_cell_mut(oc.top_left_neighbor) {
//                    match tl {
//                        Cell::Open(o) => o.set_bot_right(None),
//                        _ => ()
//                    }
//                }
//                self.board[r][c] = Cell::Closed;
//                let pig_pos_xy = Pos2D::new(self.pig.pos.y, self.pig.pos.x);
//                let paths = self.pig.strategy.compute_move(&pig_pos_xy, &self.board);
//                return Result::Ok(());
//            }
//        } else {
//            return Result::Err(String::from("There is already a block placed there"));
//        }
//    }
//
//    pub fn get_board(&self) -> &Vec<Vec<Cell>> {
//        return &self.board
//    }
//
//    pub fn get_pig_pos(&self) -> &Pos2D {
//        return &self.pig.pos
//    }
//}
