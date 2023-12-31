use crate::piece_type::PIECE_TYPE_COUNT;
use crate::{
    eval::{eval, DRAW_SCORE, MAX_EVAL},
    game::Game,
    move_gen::check_legal::LegalCheckPreprocessing,
    move_list::MoveList,
    mv::{Decode, Move},
    piece_type::PieceType,
    side::Side,
};

pub const DEFAULT_DEPTH: Depth = 7;
pub const DEFAULT_MAX_DEPTH: Depth = 12;

// todo!("fix stalemate bug");
use self::{
    killer_mv_table::KillerMoveTable,
    tt::{TranspositionTable, TtFlag},
};

mod killer_mv_table;
mod tt;

pub type Depth = u8;
const R: u8 = 2;

const MVV_LVA: [[u8; PIECE_TYPE_COUNT]; PIECE_TYPE_COUNT] = [
    [15, 14, 13, 12, 11, 10], // victim P, attacker none, p, n , b, r, q, k
    [25, 24, 23, 22, 21, 20], // victim N, attacker none, p, n , b, r, q, k
    [35, 34, 33, 32, 31, 30], // victim B, attacker none, p, n , b, r, q, k
    [45, 44, 43, 42, 41, 40], // victim R, attacker none, p, n , b, r, q, k
    [55, 54, 53, 52, 51, 50], // victim Q, attacker none, p, n , b, r, q, k
    [0, 0, 0, 0, 0, 0],       // victim K, attacker none, p, n , b, r, q, k
];

const TT_MOVE_SORT_VAL: u8 = 60;
const KILLER_MOVE_1_SORT_VAL: u8 = 9;
const KILLER_MOVE_2_SORT_VAL: u8 = 8;

type Eval = i32;

#[derive(Debug)]
struct TtDetails(TtFlag, Option<Move>, Eval);

impl TtDetails {
    pub fn new() -> TtDetails {
        TtDetails(TtFlag::Alpha, None, -MAX_EVAL)
    }

    pub fn from(flag: TtFlag, mv: Option<Move>, eval: Eval) -> TtDetails {
        TtDetails(flag, mv, eval)
    }

    pub fn flag(&self) -> TtFlag {
        self.0
    }

    pub fn mv(&self) -> Option<Move> {
        self.1
    }

    pub fn eval(&self) -> Eval {
        self.2
    }
}

pub struct MoveFinder {
    tt: TranspositionTable,
    depth: Depth,
    max_depth: Depth,
}

impl MoveFinder {
    pub fn new(depth: Depth, max_depth: Depth) -> MoveFinder {
        MoveFinder {
            depth,
            max_depth,
            tt: TranspositionTable::new(),
        }
    }

    pub fn change_search_depth(&mut self, depth: Depth) {
        self.depth = depth
    }

    pub fn change_max_depth(&mut self, depth: Depth) {
        self.max_depth = depth
    }

    pub fn score_moves_with_killer_moves(
        &self,
        game: &Game,
        mv_list: &MoveList,
        tt_mv_option: Option<&Move>,
        killer_mv_table: &KillerMoveTable,
        ply: usize,
    ) -> Vec<u8> {
        let mut scores = vec![0; mv_list.list().len()];

        let killer_mv_1_option = killer_mv_table.get_first(ply);
        let killer_mv_1_ref = killer_mv_1_option.as_ref();
        let killer_mv_2_option = killer_mv_table.get_second(ply);
        let killer_mv_2_ref = killer_mv_2_option.as_ref();

        for (i, mv) in mv_list.list().iter().enumerate() {
            if tt_mv_option == Some(mv) {
                scores[i] = TT_MOVE_SORT_VAL;
            } else if killer_mv_1_ref == Some(mv) {
                scores[i] = KILLER_MOVE_1_SORT_VAL;
            } else if killer_mv_2_ref == Some(mv) {
                scores[i] = KILLER_MOVE_2_SORT_VAL;
            } else {
                match mv {
                    Move::King(mv) | Move::Rook(mv) | Move::Pawn(mv) | Move::Piece(mv) => {
                        if mv.is_capture() {
                            let (_, to) = mv.decode_into_squares();
                            debug_assert!(
                                game.position().at(to).is_some(),
                                "move is capture but no piece found on {}\nmove: {}\n{}\n{}",
                                to,
                                mv,
                                game.position(),
                                game.position().bb_side(game.state().side_to_move().opposite())
                            );
                            let attacker = mv.piece_type();
                            let capture = game.position().at(to).unwrap().piece_type();

                            scores[i] = MVV_LVA[capture.to_usize()][attacker.to_usize()];
                        }
                    }
                    Move::EnPassant(_) => {
                        scores[i] = MVV_LVA[PieceType::Pawn.to_usize()][PieceType::Pawn.to_usize()];
                    }
                    Move::Promotion(promote_mv) => {
                        // using mvv lva array to get difference of value between
                        // the promote piece type and the pawn
                        if promote_mv.is_capture() {
                            let (_, to) = promote_mv.decode_into_squares();
                            debug_assert!(
                                game.position().at(to).is_some(),
                                "move is capture but no piece found on {}",
                                to
                            );
                            let capture = game.position().at(to).unwrap().piece_type();
                            scores[i] = MVV_LVA[promote_mv.promote_piece_type().to_usize()]
                                [PieceType::Pawn.to_usize()]
                                + MVV_LVA[capture.to_usize()][PieceType::Pawn.to_usize()]
                        } else {
                            scores[i] = MVV_LVA[promote_mv.promote_piece_type().to_usize()]
                                [PieceType::Pawn.to_usize()]
                        }
                    }
                    Move::DoublePawnPush(_) | Move::Castle(_) => {}
                }
            }
        }

        scores
    }

    pub fn score_moves(
        &self,
        game: &Game,
        mv_list: &MoveList,
        tt_mv_option: Option<&Move>,
    ) -> Vec<u8> {
        let mut scores = vec![0; mv_list.list().len()];

        for (i, mv) in mv_list.list().iter().enumerate() {
            if tt_mv_option == Some(mv) {
                scores[i] = TT_MOVE_SORT_VAL;
            } else {
                match mv {
                    Move::King(mv) | Move::Rook(mv) | Move::Pawn(mv) | Move::Piece(mv) => {
                        if mv.is_capture() {
                            let (_, to) = mv.decode_into_squares();
                            debug_assert!(
                                game.position().at(to).is_some(),
                                "move is capture but no piece found on {}",
                                to
                            );
                            let attacker = mv.piece_type();
                            let capture = game.position().at(to).unwrap().piece_type();

                            scores[i] = MVV_LVA[capture.to_usize()][attacker.to_usize()];
                        }
                    }
                    Move::EnPassant(_) => {
                        scores[i] = MVV_LVA[PieceType::Pawn.to_usize()][PieceType::Pawn.to_usize()];
                    }
                    Move::Promotion(promote_mv) => {
                        // using mvv lva array to get difference of value between
                        // the promote piece type and the pawn
                        if promote_mv.is_capture() {
                            let (_, to) = promote_mv.decode_into_squares();
                            debug_assert!(
                                game.position().at(to).is_some(),
                                "move is capture but no piece found on {}",
                                to
                            );
                            let capture = game.position().at(to).unwrap().piece_type();
                            scores[i] = MVV_LVA[promote_mv.promote_piece_type().to_usize()]
                                [PieceType::Pawn.to_usize()]
                                + MVV_LVA[capture.to_usize()][PieceType::Pawn.to_usize()]
                        } else {
                            scores[i] = MVV_LVA[promote_mv.promote_piece_type().to_usize()]
                                [PieceType::Pawn.to_usize()]
                        }
                    }
                    Move::DoublePawnPush(_) | Move::Castle(_) => {}
                }
            }
        }

        scores
    }

    pub fn pick_move(
        &self,
        mv_list: &mut MoveList,
        scores: &mut Vec<u8>,
        start_idx: usize,
    ) -> Move {
        // finds the move with the highest score and swaps it with the item at start idx

        let mut best_score = scores[start_idx];
        let mut best_score_idx = start_idx;

        for i in start_idx..mv_list.list().len() {
            let score = scores[i];
            if score > best_score {
                best_score = score;
                best_score_idx = i;
            }
        }

        scores.swap(start_idx, best_score_idx);
        mv_list.mut_list().swap(start_idx, best_score_idx);

        *mv_list.list().get(start_idx).unwrap()
    }

    pub fn get(&mut self, game: &mut Game) -> Option<(Move, Eval)> {
        self.tt.update_age(&game);

        let mut best_move = None;

        let mut alpha = -MAX_EVAL;
        let beta = MAX_EVAL;

        let stm = game.state().side_to_move();

        let legal_check_preprocessing = LegalCheckPreprocessing::from(game, stm);
        let mut pseudo_legal_mv_list = if legal_check_preprocessing.num_of_checkers() == 0 {
            game.pseudo_legal_moves(stm)
        } else {
            game.pseudo_legal_escape_moves(stm, &legal_check_preprocessing)
        };

        let mut killer_mv_table = KillerMoveTable::new(self.depth);

        let tt_mv_result = self.tt.probe_move(game.state().zobrist().to_u64(), self.depth);
        let mut scores = self.score_moves(&game, &pseudo_legal_mv_list, tt_mv_result.as_ref());

        for i in 0..pseudo_legal_mv_list.list().len() {
            let mv = self.pick_move(&mut pseudo_legal_mv_list, &mut scores, i);
            if !game.is_legal(mv, &legal_check_preprocessing) {
                continue;
            }

            let prev_state = game.state().encode();
            let capture = game.make_move(mv);

            let eval: Eval = if game.is_draw() {
                DRAW_SCORE.get(game.position().phase())
            } else {
                -self.alpha_beta(
                    game,
                    self.depth - 1,
                    -beta,
                    -alpha,
                    1,
                    &mut killer_mv_table,
                    false,
                )
            };

            println!(
                "eval: {eval}, mv: {mv}, sq score: {}",
                game.position().sq_score(stm) - game.position().sq_score(stm.opposite())
            );

            game.unmake_move(mv, capture, prev_state);

            if eval > alpha {
                alpha = eval;
                best_move = Some(mv);
            } else if best_move.is_none() {
                best_move = Some(mv);
            }
        }

        self.tt.store(
            game.state().zobrist().to_u64(),
            self.depth,
            TtFlag::Exact,
            alpha,
            best_move,
        );
        Some((
            best_move.unwrap(),
            if stm == Side::White { alpha } else { -alpha },
        ))
    }

    fn alpha_beta(
        // https://www.chessprogramming.org/Principal_Variation_Search
        &mut self,
        game: &mut Game,
        depth: Depth,
        mut alpha: Eval,
        beta: Eval,
        levels_searched: u8,
        killer_mv_table: &mut KillerMoveTable,
        do_null_move: bool,
    ) -> Eval {
        if depth == 0 {
            return self.quiescence(game, alpha, beta, levels_searched, killer_mv_table);
        };

        // get tt results
        let tt_val_result = self.tt.probe_val(game.state().zobrist().to_u64(), depth, alpha, beta);

        if let Some(tt_val) = tt_val_result {
            return tt_val;
        }

        // null move pruning
        let stm = game.state().side_to_move();

        let legal_check_preprocessing = LegalCheckPreprocessing::from(game, stm);
        if do_null_move && !legal_check_preprocessing.in_check() && depth > R {
            let en_passant_option = game.state().en_passant();
            game.make_null_move();
            let eval = -self.alpha_beta(
                game,
                depth - 1 - R,
                -beta,
                -beta + 1,
                levels_searched + 1,
                killer_mv_table,
                false,
            );
            game.unmake_null_move(en_passant_option);
            if eval >= beta {
                return eval;
            }
        }

        // getting pseudo legal moves
        let mut pseudo_legal_mv_list = if legal_check_preprocessing.in_check() {
            game.pseudo_legal_escape_moves(stm, &legal_check_preprocessing)
        } else {
            game.pseudo_legal_moves(stm)
        };

        // scores moves
        let tt_mv_result = self.tt.probe_move(game.state().zobrist().to_u64(), depth);
        let mut scores = self.score_moves_with_killer_moves(
            &game,
            &pseudo_legal_mv_list,
            tt_mv_result.as_ref(),
            killer_mv_table,
            depth as usize,
        );

        let mut legal_moves_available = false;

        let mut tt_details = TtDetails::new();
        let mut found_pv = false;

        for i in 0..pseudo_legal_mv_list.list().len() {
            let mv = self.pick_move(&mut pseudo_legal_mv_list, &mut scores, i);
            if !game.is_legal(mv, &legal_check_preprocessing) {
                continue;
            }

            legal_moves_available = true;

            let prev_state = game.state().encode();
            let capture = game.make_move(mv);

            let eval = if game.is_draw() {
                DRAW_SCORE.get(game.position().phase())
            } else if !found_pv {
                -self.alpha_beta(
                    game,
                    depth - 1,
                    -beta,
                    -alpha,
                    levels_searched + 1,
                    killer_mv_table,
                    false,
                )
            } else {
                let mut score = -self.alpha_beta(
                    game,
                    depth - 1,
                    -alpha - 1,
                    -alpha,
                    levels_searched + 1,
                    killer_mv_table,
                    !do_null_move,
                );

                // re-search
                if score > alpha && score < beta {
                    score = -self.alpha_beta(
                        game,
                        depth - 1,
                        -beta,
                        -alpha,
                        levels_searched + 1,
                        killer_mv_table,
                        !do_null_move,
                    )
                }

                score
            };

            game.unmake_move(mv, capture, prev_state);

            if eval >= beta {
                // store lower bound for position
                tt_details = TtDetails::from(TtFlag::Beta, Some(mv), eval);
                if capture.is_none() {
                    killer_mv_table.insert(mv, depth as usize);
                }
                alpha = eval;
                break;
            }

            if eval > alpha {
                tt_details = TtDetails::from(TtFlag::Exact, Some(mv), eval);
                alpha = eval;
                found_pv = true;
            } else if eval > tt_details.eval() {
                // store upper bound
                tt_details = TtDetails::from(TtFlag::Alpha, None, eval);
            }
        }

        if !legal_moves_available && legal_check_preprocessing.in_check() {
            return -(MAX_EVAL - levels_searched as Eval);
        } else if !legal_moves_available && DRAW_SCORE.get(game.position().phase()) > alpha {
            // is a stalemate
            return DRAW_SCORE.get(game.position().phase());
        }

        self.tt.store(
            game.state().zobrist().to_u64(),
            depth,
            tt_details.flag(),
            tt_details.eval(),
            tt_details.mv(),
        );

        debug_assert!(
            alpha > i32::MIN && alpha <= MAX_EVAL,
            "alpha is out of bounds\nalpha:{alpha}\ntt-details: {:?}",
            tt_details
        );

        alpha
    }

    fn quiescence(
        &mut self,
        game: &mut Game,
        mut alpha: Eval,
        beta: Eval,
        levels_searched: u8,
        killer_mv_table: &mut KillerMoveTable,
    ) -> Eval {
        // defining variables we need for the rest of the function
        let stm = game.state().side_to_move();
        let legal_check_preprocessing = LegalCheckPreprocessing::from(game, stm);

        if levels_searched == self.max_depth {
            return eval(game, &legal_check_preprocessing, levels_searched);
        }

        if legal_check_preprocessing.in_check() {
            return self.alpha_beta(
                game,
                1,
                alpha,
                beta,
                levels_searched,
                killer_mv_table,
                false,
            );
        }

        // handle standpat score
        let stand_pat = eval(game, &legal_check_preprocessing, levels_searched);
        if stand_pat >= beta {
            return stand_pat;
        }
        if stand_pat > alpha {
            alpha = stand_pat;
        }

        let mut pseudo_legal_mv_list = game.pseudo_legal_loud_moves(stm);

        let mut scores = self.score_moves(&game, &pseudo_legal_mv_list, None);

        for i in 0..pseudo_legal_mv_list.list().len() {
            let mv = self.pick_move(&mut pseudo_legal_mv_list, &mut scores, i);
            if !game.is_legal(mv, &legal_check_preprocessing) {
                continue;
            }

            let prev_state = game.state().encode();
            let capture = game.make_move(mv);

            let eval: Eval = if game.is_draw() {
                DRAW_SCORE.get(game.position().phase())
            } else {
                -self.quiescence(game, -beta, -alpha, levels_searched + 1, killer_mv_table)
            };

            game.unmake_move(mv, capture, prev_state);

            if eval >= beta {
                return eval;
            }

            if eval > alpha {
                alpha = eval;
            }
        }

        debug_assert!(
            alpha > i32::MIN && alpha <= MAX_EVAL,
            "alpha is out of bounds\nalpha:{alpha}",
        );

        alpha
    }
}

#[cfg(test)]
pub mod test_basic_tactics {
    use crate::fen::STARTING_POSITION_FEN;
    use crate::mv::EncodedMove;
    use crate::piece_type::PieceType;
    use crate::psqt::PSQT;
    use crate::{square::*, uci};

    use super::*;

    #[test]
    fn pos_1() {
        let fen = "r3rk2/pb4p1/4QbBp/1p1q4/2pP4/2P5/PP3PPP/R3R1K1 w - - 0 21";
        let result = Game::from_fen(fen);
        assert!(result.is_ok());
        let mut game = result.unwrap();
        let mut mv_finder = MoveFinder::new(DEFAULT_DEPTH, DEFAULT_MAX_DEPTH);

        let best_move_result = mv_finder.get(&mut game);
        let expected = Move::Piece(EncodedMove::new(E6, E8, PieceType::Queen, true));

        assert!(best_move_result.is_some());
        let (best_move, _) = best_move_result.unwrap();
        assert_eq!(best_move, expected)
    }

    #[test]
    fn pos_2() {
        let fen = "5rk1/ppq3p1/2p3Qp/8/3P4/2P3nP/PP1N2PK/R1B5 b - - 0 28";
        let result = Game::from_fen(fen);
        assert!(result.is_ok());
        let mut game = result.unwrap();
        let mut mv_finder = MoveFinder::new(DEFAULT_DEPTH, DEFAULT_MAX_DEPTH);

        let best_move_result = mv_finder.get(&mut game);
        let expected = Move::Piece(EncodedMove::new(G3, F1, PieceType::Knight, false));

        assert!(best_move_result.is_some());
        let (best_move, _) = best_move_result.unwrap();
        assert_eq!(
            best_move, expected,
            "\nbest move: {}; \nexpected: {}",
            best_move, expected
        )
    }

    #[test]
    fn pos_3() {
        let fen = STARTING_POSITION_FEN;
        let result = Game::from_fen(fen);
        assert!(result.is_ok());
        let mut game = result.unwrap();
        game.make_move(Move::DoublePawnPush(EncodedMove::new(
            E2,
            E4,
            PieceType::Pawn,
            false,
        )));
        let mut mv_finder = MoveFinder::new(DEFAULT_DEPTH, DEFAULT_MAX_DEPTH);

        let best_move_result = mv_finder.get(&mut game);

        assert!(best_move_result.is_some());
        let (best_move, _) = best_move_result.unwrap();
        assert_ne!(best_move.to_string(), "d7d5")
    }

    #[test]
    fn mate_in_4() {
        let fen = "r1bqr2k/ppp3bp/2np2p1/8/2BnPQ2/2N2N2/PPPB1PP1/2KR3R w - - 0 0";
        let result = Game::from_fen(fen);
        assert!(result.is_ok());
        let mut game = result.unwrap();
        let mut mv_finder = MoveFinder::new(DEFAULT_DEPTH, DEFAULT_MAX_DEPTH);

        let best_move_result = mv_finder.get(&mut game);
        let expected = Move::Rook(EncodedMove::new(H1, H7, PieceType::Rook, true));

        println!("{}", game.position());
        assert!(best_move_result.is_some());
        let (best_move, eval) = best_move_result.unwrap();
        assert_eq!(
            best_move, expected,
            "\nbest move: {}; eval: {}\nexpected: {}",
            best_move, eval, expected
        );
        assert_eq!(eval, MAX_EVAL - 9);
    }

    #[test]
    fn pos_4() {
        let mut game =
            Game::from_fen("rn1qkbnr/ppp2ppp/3p4/4p3/2B1P3/5b2/PPPP1PPP/RNBQK2R w KQkq - 0 1")
                .unwrap();

        let mut mv_finder = MoveFinder::new(DEFAULT_DEPTH, DEFAULT_MAX_DEPTH);

        let best_move_result = mv_finder.get(&mut game);

        assert!(best_move_result.is_some());
        let (best_move, eval) = best_move_result.unwrap();
        println!("{}", game.position());
        println!("\nbest move: {}; eval: {}", best_move, eval);
        println!(
            "\n f3 psqt: {}",
            PSQT[Side::White.to_usize()][PieceType::Pawn.to_usize()][F3.to_usize()]
                .get(crate::phase::Phase::Opening)
        );
        assert_eq!(best_move.to_string(), "d1f3");
    }

    #[test]
    fn debug_pos_stop_promotion() {
        let mut game = Game::from_fen(STARTING_POSITION_FEN).unwrap();
        uci::input_position(
            "position startpos moves a2a3 a7a5 b2b3 a5a4 b3a4 a8a4 c2c3 b7b5 d2d4 c7c6 e2e4 d7d5 e4d5 d8d5 h2h3 f7f6 h3h4 e7e6 b1d2 g7g6 h4h5 g6h5 h1h5 e6e5 f2f4 c8e6 d4e5 a4f4 e5f6 f4f5 h5f5 d5f5 g2g4 f5g4 d1g4 e6g4 c3c4 f8d6 f6f7 e8f7 g1e2 b5c4 d2c4 d6e7 a3a4 h7h5 c4e5 f7e6 e5g4 h5g4 a4a5 c6c5 a5a6",
            &mut game,
        );
        let mut mv_finder = MoveFinder::new(DEFAULT_DEPTH, DEFAULT_MAX_DEPTH);

        let best_move_result = mv_finder.get(&mut game);

        assert!(best_move_result.is_some());
        let (best_move, eval) = best_move_result.unwrap();
        println!(
            "white score: {}, black score: {}",
            game.position().piece_score(Side::White),
            game.position().piece_score(Side::Black)
        );
        println!("{}", game.position());
        println!("\nbest move: {}; eval: {}", best_move, eval);
        assert_ne!(best_move.to_string(), "g8h6");
    }

    #[test]
    fn debug_pos_4() {
        let mut game = Game::from_fen(STARTING_POSITION_FEN).unwrap();
        uci::input_position(
            "position startpos moves b1c3 e7e6 d2d4 c7c6 e2e4 a7a6 g1f3 f8b4 a2a3 b4e7 c1f4 b7b5 h2h3 d7d6 e4e5 d6d5 b2b4 a6a5 b4a5 d8a5 f4d2 e7b4 c3b5 b4d2 d1d2 c6b5 d2a5 a8a5 a1b1 c8a6 a3a4 a5a4 f1b5 a6b5 b1b5 a4a1 e1d2 a1h1 b5b8 e8d7 f3g5 h1f1 d2e2 h7h6 e2f1 h6g5 b8b7 d7e8 c2c3 g8h6 b7a7 g5g4 f2f3 g4f3 f1g1 f3g2 g1g2 f7f6 e5f6 g7f6 a7a8 e8d7 a8h8 h6f5 h8f8 d7c6 f8f6 c6d7 g2g1 d7d6 f6f8 f5g3 f8c8 d6e7 c8b8 g3e2 g1f1 e2c3 h3h4 c3e4 h4h5 e4g3 f1f2 g3h5 f2e1 h5g3 e1d2 e6e5 d4e5 d5d4 d2d3 g3f1 d3d4 f1d2 d4e3 d2f1 e3e2 f1h2 b8b4 e7e6 b4e4 e6d7 e5e6 d7c6 e6e7 h2f1 e7e8q c6c5 e2f1 c5d5 e8e5 d5c6 e4b4 c6d7 b4b1 d7c6 b1a1 c6b6 a1b1 b6a6 b1a1 a6b6 a1b1 b6a6",
            &mut game,
        );

        let mut mv_finder = MoveFinder::new(DEFAULT_DEPTH, DEFAULT_MAX_DEPTH);

        let best_move_result = mv_finder.get(&mut game);

        assert!(best_move_result.is_some());
        let (best_move, eval) = best_move_result.unwrap();
        println!(
            "white score: {}, black score: {}",
            game.position().piece_score(Side::White),
            game.position().piece_score(Side::Black)
        );
        println!("{}", game.position());
        println!("\nbest move: {}; eval: {}", best_move, eval);
        assert_ne!(best_move.to_string(), "b1a1");
    }

    #[test]
    fn debug_pos_5() {
        let mut game = Game::from_fen(STARTING_POSITION_FEN).unwrap();
        uci::input_position(
            "position startpos moves d2d4 c7c5 e2e3 c5d4 e3d4 d7d5 f1b5 c8d7 b5d7 b8d7 b1c3 e7e6 g1e2 a7a6 h2h3 g8f6 e1g1 f8d6 b2b3 a8c8 a2a4 e8g8 c1b2 d7b8 f2f4 b8c6 a1c1 h7h6 e2g3 c6d4 c3d5 f6d5 b2d4 d6f4 d1g4 f4e3 g1h1 d8g5 g4g5 h6g5 c1d1 e3d4 d1d4 c8c2 g3f5 c2e2 b3b4 g5g4 h3g4 f8d8 f5e7 g8f8 e7d5 d8d5 d4d5 e6d5 f1d1 e2e5 h1g1 f8e7 b4b5 a6a5 d1d2 e7e6 d2d4 e5e2 g4g5 f7f5 g2g3 b7b6 g1f1 e2e3 f1f2 e3a3 f2g2 g7g6 d4h4 a3a2 g2f1 e6d6 h4d4 d6e5 d4d1 d5d4 d1e1 e5d6 g3g4 f5g4 e1e4 a2a1 f1e2 d6d5 e4g4 a1a4 e2f3 a4a3 f3f4 a3a2 g4g1 a2e2 g1c1 e2f2 f4g4 f2e2 c1c6 e2g2 g4f4 g2g1 c6b6 a5a4 b6g6 g1f1 f4g4 a4a3 g6a6 f1a1 a6a8 a3a2 g4g3 a1g1 g3f2 a2a1q a8a1 g1a1 b5b6 a1b1 g5g6 b1b6 g6g7 b6g6 f2e1 g6g7 e1d1 g7g2 d1c1 d4d3 c1b1 g2c2 b1a1 c2d2 a1b1 d2c2 b1a1 c2d2 a1b1 d2e2 b1a1 d3d2 a1b1",
            &mut game,
        );

        let mut mv_finder = MoveFinder::new(DEFAULT_DEPTH, DEFAULT_MAX_DEPTH);

        let best_move_result = mv_finder.get(&mut game);

        assert!(best_move_result.is_some());
        let (best_move, eval) = best_move_result.unwrap();
        println!(
            "white score: {}, black score: {}",
            game.position().piece_score(Side::White),
            game.position().piece_score(Side::Black)
        );
        println!("{}", game.position());
        println!("\nbest move: {}; eval: {}", best_move, eval);
        assert_eq!(best_move.to_string(), "d2d1q");
    }
}
