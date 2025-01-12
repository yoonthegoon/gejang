use crate::types::structs::Bitboard;

const FILE_A: Bitboard = Bitboard(0x0101010101010101);
const FILE_B: Bitboard = Bitboard(0x0202020202020202);
const FILE_C: Bitboard = Bitboard(0x0404040404040404);
const FILE_D: Bitboard = Bitboard(0x0808080808080808);
const FILE_E: Bitboard = Bitboard(0x1010101010101010);
const FILE_F: Bitboard = Bitboard(0x2020202020202020);
const FILE_G: Bitboard = Bitboard(0x4040404040404040);
const FILE_H: Bitboard = Bitboard(0x8080808080808080);

const RANK_1: Bitboard = Bitboard(0x00000000000000FF);
const RANK_2: Bitboard = Bitboard(0x000000000000FF00);
const RANK_3: Bitboard = Bitboard(0x0000000000FF0000);
const RANK_4: Bitboard = Bitboard(0x00000000FF000000);
const RANK_5: Bitboard = Bitboard(0x000000FF00000000);
const RANK_6: Bitboard = Bitboard(0x0000FF0000000000);
const RANK_7: Bitboard = Bitboard(0x00FF000000000000);
const RANK_8: Bitboard = Bitboard(0xFF00000000000000);
