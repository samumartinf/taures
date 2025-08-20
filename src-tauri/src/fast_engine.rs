use crate::engine::Engine;
use crate::{ChessGame, Move};
use std::time::Instant;

/// High-performance engine optimizations
impl Engine {
    /// Much faster best move search by reducing overhead
    pub fn get_best_move_optimized(&mut self, depth: u8) -> Move {
        let start = Instant::now();
        self.num_positions_evaluated = 0;
        self.cache_hits_last_eval = 0;
        
        // Get legal moves once at root
        let moves = self.game.get_legal_moves(self.game.white_turn);
        if moves.is_empty() {
            return Move { source: 0, target: 0, promotion: 0 };
        }
        
        let mut best_move = moves[0];
        let mut best_score = -100000;
        
        for mv in moves {
            // Use existing game infrastructure but optimize search
            let success = self.game.play_move_ob(mv);
            if !success {
                continue;
            }
            
            let score = -self.alpha_beta_optimized(depth - 1, -100000, 100000);
            self.game.undo_move();
            
            if score > best_score {
                best_score = score;
                best_move = mv;
            }
        }
        
        let elapsed = start.elapsed();
        println!("Optimized engine: {} positions in {:?} ({:.0} pos/sec)", 
                self.num_positions_evaluated, elapsed, 
                self.num_positions_evaluated as f64 / elapsed.as_secs_f64());
        
        best_move
    }
    
    /// Optimized alpha-beta with better move ordering and pruning
    pub fn alpha_beta_optimized(&mut self, depth: u8, mut alpha: i32, beta: i32) -> i32 {
        self.num_positions_evaluated += 1;
        
        if depth == 0 {
            // Clone the board to avoid borrowing issues
            let board = self.game.board.clone();
            return self.evaluate(&board);
        }
        
        // Always use bitboard pseudolegal generation for maximum speed
        let moves = self.game.get_all_moves_bitboard(self.game.white_turn);
        let mut best_score = -100000;
        let mut legal_moves_found = false;
        
        for mv in moves {
            // Fast legality check: try the move and see if it leaves king in check
            let success = self.game.play_move_ob(mv);
            if !success {
                continue; // Skip illegal moves
            }
            
            // Quick check: if our king is in check after our move, it's illegal
            let king_square = self.find_king_square(!self.game.white_turn);
            let in_check = self.is_square_attacked_fast(king_square, self.game.white_turn);
            
            if in_check {
                self.game.undo_move();
                continue; // Illegal move - leaves king in check
            }
            
            legal_moves_found = true;
            let score = -self.alpha_beta_optimized(depth - 1, -beta, -alpha);
            self.game.undo_move();
            
            if score > best_score {
                best_score = score;
            }
            if score > alpha {
                alpha = score;
            }
            if alpha >= beta {
                break; // Alpha-beta cutoff
            }
        }
        
        // If no legal moves, it's checkmate or stalemate
        if !legal_moves_found {
            let king_square = self.find_king_square(self.game.white_turn);
            if self.is_square_attacked_fast(king_square, !self.game.white_turn) {
                return -99000 + (5 - depth as i32); // Checkmate (closer is worse)
            } else {
                return 0; // Stalemate
            }
        }
        
        best_score
    }
    
    /// Fast king finding using bitboards
    fn find_king_square(&self, is_white: bool) -> u8 {
        let king_bb = if is_white { 
            self.game.board.bitboard[5] // White king
        } else { 
            self.game.board.bitboard[11] // Black king
        };
        
        // Find the least significant bit (king position)
        for i in 0..64 {
            if (king_bb & (1u64 << i)) != 0 {
                return i as u8;
            }
        }
        60 // Default to e1 if not found (shouldn't happen)
    }
    
    /// Fast attack detection using bitboards
    fn is_square_attacked_fast(&self, square: u8, by_white: bool) -> bool {
        use crate::bitboard_movegen::BitboardMoveGen;
        BitboardMoveGen::is_square_attacked(&self.game.board, square, by_white)
    }
}