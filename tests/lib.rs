extern crate sfen;
extern crate vec_map;
use sfen::*;
use vec_map::VecMap;
use sfen::Player::*;
use sfen::PieceType::*;

#[test]
fn test_root() {
  let shogi = sfen::root("8l/1l+R2P3/p2pBG1pp/kps1p4/Nn1P2G2/P1P1P2PP/1PS6/1KSG3+r1/LN2+p3L w Sbgn3p 124");
  let cap = sfen::captured_pieces("Sbgn3p").unwrap();
  let board = sfen::board("8l/1l+R2P3/p2pBG1pp/kps1p4/Nn1P2G2/P1P1P2PP/1PS6/1KSG3+r1/LN2+p3L").unwrap();
  assert_eq!((board, Second, cap, Some(124)), shogi.unwrap());
}
#[test]
fn test_board() {
  let b = sfen::board("8l/1l+R2P3/p2pBG1pp/kps1p4/Nn1P2G2/P1P1P2PP/1PS6/1KSG3+r1/LN2+p3L");
  let mut board = VecMap::new();
  board.insert(8, (Lance, Second));

  board.insert(10, (Lance, Second));
  board.insert(11, (PromotedRook, First));
  board.insert(14, (Pawn, First));

  board.insert(18, (Pawn, Second));
  board.insert(21, (Pawn, Second));
  board.insert(22, (Bishop, First));
  board.insert(23, (GoldGeneral, First));
  board.insert(25, (Pawn, Second));
  board.insert(26, (Pawn, Second));

  board.insert(27, (King, Second));
  board.insert(28, (Pawn, Second));
  board.insert(29, (SilverGeneral, Second));
  board.insert(31, (Pawn, Second));

  board.insert(36, (Knight, First));
  board.insert(37, (Knight, Second));
  board.insert(39, (Pawn, First));
  board.insert(42, (GoldGeneral, First));

  board.insert(45, (Pawn, First));
  board.insert(47, (Pawn, First));
  board.insert(49, (Pawn, First));
  board.insert(52, (Pawn, First));
  board.insert(53, (Pawn, First));

  board.insert(55, (Pawn, First));
  board.insert(56, (SilverGeneral, First));

  board.insert(64, (King, First));
  board.insert(65, (SilverGeneral, First));
  board.insert(66, (GoldGeneral, First));
  board.insert(70, (PromotedRook, Second));

  board.insert(72, (Lance, First));
  board.insert(73, (Knight, First));
  board.insert(76, (PromotedPawn, Second));
  board.insert(80, (Lance, First));
  assert_eq!((board, 9, 9), b.unwrap());
}
use std::collections::HashMap;
#[test]
fn test_captured_pieces() {
  let cap = sfen::captured_pieces("Sbgn3p");
  let mut first = HashMap::new();
  first.insert(SilverGeneral, 1);
  let mut second = HashMap::new();
  second.insert(Bishop, 1);
  second.insert(GoldGeneral, 1);
  second.insert(Knight, 1);
  second.insert(Pawn, 3);
  assert_eq!((first, second), cap.unwrap());
}
