pub const PAWN_TABLE: [i32; 64] = [
    0,  0,  0,  0,  0,  0,  0,  0,
    50,  50,  50,  50,  50,  50,  50,  50,
    20,  20,  25,  40,  40,  25,  20,  20,
    0,  0,  10,  30,  30,  10,  0,  0,
   -10, -20,  10,  30,  30,  10, -20, -10,
   -10,  0,  0,  20,  20,  0,  0,  -10,
    10,  10,  0,  -10, -10, 0,  10,  10,
    1,  0,  0,  0,  0,  0,  0,  0,
];


pub const KNIGHT_TABLE: [i32; 64] = [
    -40, -10, -10, -10, -10, -10, -10, -40,
    -10, 0, 10, 20, 20, 10, 0, -10,
    -10, 0,  10,  10,  10,  10, 0, -10,
    -20, 0,  0,  10,  10,  0, 0, -20,
    -20, 0,  0,  0,  0,  0, 0, -20,
    -20, -10,  10,  0,  0,  10, -10, -20,
    -30, -20, -10, 0, 0, -10, -20, -30,
    -40, -30, -30, -30, -30, -30, -30, -40,
];


pub const BISHOP_TABLE: [i32; 64] = [
    -20, -10, -10, -10, -10, -10, -10, -20,
    0,  0,  0,  0,  0,  0,  0, 0,
    -10,  0,  0,  0,  0,  0,  0, -10,
    -10,  0,  0,  0,  0,  0,  0, -10,
    -10,  0,  0,  0,  0,  0,  0, -10,
    0,  0,  0,  0,  0,  0,  0, 0,
    -10,  10,  0,  0,  0,  0,  10, -10,
    -20, -10, -10, -10, -10, -10, -10, -20,
];

pub const ROOK_TABLE: [i32; 64] = [
     0, 10, 10, 10, 10, 10, 10,  0,
     10, 20, 20, 20, 20, 20, 20,  10,
    -10, 0, 0, 0, 0, 0, 0, -10,0
    -10, 0, 0, 0, 0, 0, 0, -10,
    -10, 0, 0, 0, 0, 0, 0, -10,
    -10, 0, 0, 0, 0, 0, 0, -10,
    -10, 0, 0, 0, 0, 0, 0, -10,
     0, 0, 0, 10, 10, 0, 0,  0,
];

pub const QUEEN_TABLE: [i32; 64] = [
    -20, -10, -10,  0,  0, -10, -10, -20,
    -10,  0,  0,  0,  0,  0,  0, -10,
    -10,  0,  0,  0,  0,  0,  0, -10,
    -10,  0,  0,  5,  5,  0,  0, 0,
    0,  0,  0,  5,  5,  0,  0, -10,
    -10,  5,  0,  0,  0,  5,  0, -10,
    -10,  0,  0,  0,  0,  0,  0, -10,
    -20, -10, 0,  0,  0, -10, -10, -20,
];
pub const KING_TABLE: [i32; 64] = [
    -50, -50, -50, -50, -50, -50, -50, -50,
    -50, -50, -50, -50, -50, -50, -50, -50,
    -50, -50, -50, -80, -80, -50, -50, -50,
    -50, -50, -80, -100, -100, -80, -50, -50,
    -50, -50, -80, -100, -100, -80, -50, -50,
    -30, -50, -50, -80, -80, -50, -50, -30,
    -10, -10, -40, -40, -40, -40, -10, -10,
     20,  30,  20,  -10,  -10,  -10,  30,  30,
];
pub const KING_TABLE_ENDGAME: [i32; 64] = [
    -50, -50, -50, -50, -50, -50, -50, -50,
    -50, -20, -20, -20, -20, -20, -20, -50,
    -50, -20,  20,  20,  20,  20, -20, -50,
    -50, -20,  20,  60,  60,  20, -20, -50,
    -50, -20,  20,  60,  60,  20, -20, -50,
    -50, -20,  20,  20,  20,  20, -20, -50,
    -50, -20, -20, -20, -20, -20, -20, -50,
    -50, -50, -50, -50, -50, -50, -50, -50,
];