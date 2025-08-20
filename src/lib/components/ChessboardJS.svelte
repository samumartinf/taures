<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount, onDestroy } from "svelte";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Card, CardContent, CardFooter, CardHeader, CardTitle } from "$lib/components/ui/card";
  import { Separator } from "$lib/components/ui/separator";
  import { Badge } from "$lib/components/ui/badge";
  
  // Import chess piece SVGs
  import blackBishop from '../../assets/chessPieces/bB.svg';
  import blackPawn from '../../assets/chessPieces/bP.svg';
  import blackQueen from '../../assets/chessPieces/bQ.svg';
  import blackRook from '../../assets/chessPieces/bR.svg';
  import blackKing from '../../assets/chessPieces/bK.svg';
  import blackKnight from '../../assets/chessPieces/bN.svg';
  import whiteBishop from '../../assets/chessPieces/wB.svg';
  import whitePawn from '../../assets/chessPieces/wP.svg';
  import whiteQueen from '../../assets/chessPieces/wQ.svg';
  import whiteRook from '../../assets/chessPieces/wR.svg';
  import whiteKnight from '../../assets/chessPieces/wN.svg';
  import whiteKing from '../../assets/chessPieces/wK.svg';

  // State variables
  let fen = 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1'; // Starting position
  let turn = 'w';
  let customFen = '';
  let board: any;
  let engineDepth = 3; // Default engine depth
  let isEngineThinking = false;
  let playMode = true; // false = manual mode, true = play against engine
  let humanColor = 'w'; // 'w' = human plays white, 'b' = human plays black
  let useOpeningVariety = true; // Enable/disable opening book variety
  let lastEngineStats = {
    positions: 0,
    timeMs: 0,
    positionsPerSecond: 0,
    bestMove: '',
    depth: 0
  };
  let debugInfo = {
    boardFen: '',
    engineFen: '',
    piece: '',
    enginePiece: '',
    allowedMoves: [] as string[],
    legalMoves: [] as string[]
  };
  
  // Get piece image based on piece code
  function pieceTheme(piece: string) {
    switch (piece) {
      case "wB": return whiteBishop;
      case "bB": return blackBishop;
      case "wP": return whitePawn;
      case "bP": return blackPawn;
      case "wQ": return whiteQueen;
      case "bQ": return blackQueen;
      case "wR": return whiteRook;
      case "bR": return blackRook;
      case "wN": return whiteKnight;
      case "bN": return blackKnight;
      case "bK": return blackKing;
      case "wK": return whiteKing;
      default: return '';
    }
  }
  
  // Synchronize the board with the engine state
  async function syncBoardWithEngine() {
    console.log("Synchronizing board with engine state");
    try {
      const engineFen = await invoke("get_fen") as string;
      
      // Update the board with the engine's position (false = no animation)
      board.position(engineFen, false);
      
      // Update turn indicator
      await updateTurn();
      
      console.log("Board synchronized with engine state");
    } catch (error) {
      console.error("Error synchronizing board with engine:", error);
    }
  }
  
  // Handle piece drag start - keep it simple like chess.js example
  async function onDragStart(source: string, piece: string) {
    console.log("Drag start from", source, "piece:", piece);
    
        // Update debug info
    debugInfo.boardFen = board.fen();
    debugInfo.piece = piece;
    debugInfo.engineFen = await invoke("get_fen") as string;
    debugInfo.enginePiece = await invoke("get_piece_at_square", { square: source }) as string;
    debugInfo.allowedMoves = await invoke("get_possible_moves", { source }) as string[];
    debugInfo.legalMoves = await invoke("get_legal_moves", { source }) as string[];
    // Only allow the current player to move their pieces
    const pieceColor = piece.charAt(0);
    if (turn !== pieceColor) {
      console.log("Not your turn to move");
      return false;
    }
    
    // In play mode, only allow human to move their color
    if (playMode && turn !== humanColor) {
      console.log("It's the engine's turn");
      return false;
    }
    
    return true;
  }
  
  // Handle piece drop - simple like chess.js example
  async function onDrop(source: string, target: string, piece: string) {
    console.log("Drop from", source, "to", target, "piece:", piece);
    
    // If source and target are the same, it's not a move
    if (source === target) {
      return 'snapback';
    }
    
    // Handle pawn promotion
    let promotion = '';
    if (piece === 'wP' && target.charAt(1) === '8') {
      promotion = 'Q'; // Promote to queen by default
    } else if (piece === 'bP' && target.charAt(1) === '1') {
      promotion = 'q'; // Promote to queen by default
    }
    
    // Try to make the move with the engine
    const move = { source, target, promotion };
    
    try {
      const isLegal = await invoke("is_move_legal", move) as boolean;
      
      // If illegal, snap back
      if (!isLegal) {
        console.log("Illegal move, snapping back");
        return 'snapback';
      }
      
      // Legal move - execute it
      await invoke("play_move", move);
      
      // Update turn after successful move
      await updateTurn();
      
      // In play mode, trigger engine move after human move
      if (playMode && turn !== humanColor) {
        setTimeout(() => makeEngineMove(), 500);
      }
      
      // Move was successful, don't return anything (let the piece stay)
      
    } catch (error) {
      console.error("Error making move:", error);
      return 'snapback';
    }
  }
  
  // Handle snap end - sync visual board with engine state (like chess.js example)
  async function onSnapEnd() {
    // Update the board position after the piece snap
    // This handles castling, en passant, pawn promotion display
    try {
      const currentFen = await invoke("get_fen") as string;
      board.position(currentFen);
      // Note: we don't call updateTurn() here since onDrop already handles it
    } catch (error) {
      console.error("Error in onSnapEnd:", error);
    }
  }
  
  // Update the turn indicator
  async function updateTurn() {
    try {
      const newFen = await invoke("get_fen") as string;
      board.position(newFen);
      fen = newFen;
      
      // Extract turn from FEN
      const fenParts = fen.split(' ');
      if (fenParts.length > 1) {
        turn = fenParts[1];
      }
    } catch (error) {
      console.error("Error updating turn:", error);
    }
  }
  
  // Chess board actions
  async function makeRandomMove() {
    const newFen = await invoke("make_random_move") as string;
    if (newFen !== "None") {
      await syncBoardWithEngine();
    }
  }
  
  async function makeBestMove() {
    try {
      isEngineThinking = true;
      const response = await invoke("get_engine_move", { depth: engineDepth }) as {
        fen: string;
        positions_evaluated: number;
        time_ms: number;
        positions_per_second: number;
        best_move: string;
      };
      
      lastEngineStats = {
        positions: response.positions_evaluated,
        timeMs: response.time_ms,
        positionsPerSecond: response.positions_per_second,
        bestMove: response.best_move,
        depth: engineDepth
      };
      
      await syncBoardWithEngine();
    } catch (error) {
      console.error("Error getting engine move:", error);
    } finally {
      isEngineThinking = false;
    }
  }

  // Automatic engine move for play mode
  async function makeEngineMove() {
    if (!playMode || turn === humanColor || isEngineThinking) return;
    
    console.log("Engine's turn, making automatic move...");
    await makeBestMove();
  }
  
  async function restart() {
    await invoke("restart_game");
    // Clear engine stats when starting new game
    lastEngineStats = {
      positions: 0,
      timeMs: 0,
      positionsPerSecond: 0,
      bestMove: '',
      depth: 0
    };
    await syncBoardWithEngine();
    
    // In play mode, if human plays black, engine should move first
    if (playMode && humanColor === 'b') {
      setTimeout(() => makeEngineMove(), 1000); // Give time for board to update
    }
  }
  
  async function undoMove() {
    await invoke("undo_move");
    await syncBoardWithEngine();
  }
  
  function flipBoard() {
    board.flip();
  }
  
  async function setFen() {
    if (customFen) {
      const success = await invoke("set_fen", { fen: customFen }) as boolean;
      if (success) {
        await syncBoardWithEngine();
      }
    }
  }
  
  // Initialize the chessboard
  onMount(async () => {
    // Wait for the DOM to be ready
    setTimeout(async () => {
      // Make sure jQuery and Chessboard are available
      if (typeof window !== 'undefined' && window.Chessboard) {
        console.log("Chessboard.js found, initializing board...");
        const config = {
          draggable: true,
          position: 'start',
          pieceTheme: pieceTheme,
          onDragStart: onDragStart,
          onDrop: onDrop,
          onSnapEnd: onSnapEnd, // Called when piece finishes moving/snapping back
          moveSpeed: 50, // Make moves faster to reduce visual confusion
          snapbackSpeed: 50, // Make snapback faster
          snapSpeed: 50,
          trashSpeed: 100,
          sparePieces: false,
          showErrors: true,
          showNotation: true
        };
        
        try {
          board = window.Chessboard('board', config);
          console.log("Board initialized successfully");
          
          // Initialize the game
          await restart();
          
          // Load opening variety setting
          useOpeningVariety = await invoke('get_opening_variety') as boolean;
          
          // Make the board responsive
          window.addEventListener('resize', () => {
            board.resize();
          });
          
        } catch (error) {
          console.error("Error initializing chessboard:", error);
        }
      } else {
        console.error('Chessboard.js not found. Make sure it is properly loaded.');
        console.log("Window.Chessboard:", window.Chessboard);
        console.log("jQuery:", window.$);
      }
    }, 500); // Increased timeout to ensure scripts are loaded
  });
  
  // Test function to help debug synchronization issues
  async function testSynchronization() {
    console.log("Testing synchronization between board and engine");
    
    // Get current states
    const boardFen = board ? board.fen() : "Board not initialized";
    const engineFen = await invoke("get_fen") as string;
    
    console.log("Current board FEN:", boardFen);
    console.log("Current engine FEN:", engineFen);
    
    // Check if they match
    if (board && boardFen.split(' ')[0] !== engineFen.split(' ')[0]) {
      console.log("Board and engine are out of sync!");
      console.log("Synchronizing...");
      await syncBoardWithEngine();
      
      // Verify sync worked
      const newBoardFen = board.fen();
      console.log("After sync - Board FEN:", newBoardFen);
      console.log("After sync - Engine FEN:", engineFen);
      
      return "Synchronization completed";
    } else {
      console.log("Board and engine are in sync!");
      return "Already in sync";
    }
  }
  
  onDestroy(() => {
    // Clean up event listeners
    if (typeof window !== 'undefined') {
      window.removeEventListener('resize', () => {
        if (board) board.resize();
      });
    }
  });
</script>

<div class="grid grid-cols-1 md:grid-cols-[1fr_auto] gap-4 p-4">
  <!-- Chessboard -->
  <Card class="shadow-lg">
    <CardHeader>
      <CardTitle class="flex justify-between items-center">
        <span>Chess Game</span>
        <Badge variant={turn === 'w' ? 'default' : 'destructive'}>
          {turn === 'w' ? 'White to move' : 'Black to move'}
        </Badge>
      </CardTitle>
    </CardHeader>
    <CardContent>
      <div class="aspect-square w-full max-w-[600px] mx-auto">
        <div id="board" class="w-full h-full" style="min-height: 400px;"></div>
      </div>
    </CardContent>
    <CardFooter class="flex flex-wrap gap-2 justify-center">
      <Button onclick={() => restart()}>New Game</Button>
      <Button onclick={() => undoMove()}>Undo</Button>
      <Button onclick={() => flipBoard()}>Flip Board</Button>
      {#if !playMode}
        <Button onclick={() => makeRandomMove()}>Random Move</Button>
        <Button 
          onclick={() => makeBestMove()} 
          disabled={isEngineThinking}
        >
          {isEngineThinking ? 'Thinking...' : `Best Move (Depth ${engineDepth})`}
        </Button>
      {:else}
        <Button onclick={() => makeRandomMove()} variant="outline">Random Move</Button>
        {#if isEngineThinking}
          <Badge variant="secondary">Engine thinking...</Badge>
        {/if}
      {/if}
      <Button onclick={() => testSynchronization()} variant="outline">Sync Board</Button>
    </CardFooter>
  </Card>
  
  <!-- Controls and Debug Info -->
  <div class="flex flex-col gap-4">
    <Card>
      <CardHeader>
        <CardTitle>Engine Settings</CardTitle>
      </CardHeader>
      <CardContent class="space-y-4">
        <div class="space-y-3">
          <div class="flex items-center space-x-2">
            <input 
              type="checkbox" 
              id="play-mode" 
              bind:checked={playMode}
              class="rounded"
            />
            <label for="play-mode" class="text-sm font-medium">
              Play against engine
            </label>
          </div>
          
          {#if playMode}
            <div class="space-y-2 pl-6 border-l-2 border-gray-200">
              <label class="text-sm font-medium">You play as:</label>
              <div class="flex gap-2">
                <label class="flex items-center space-x-1">
                  <input 
                    type="radio" 
                    bind:group={humanColor} 
                    value="w"
                    on:change={() => restart()}
                  />
                  <span class="text-sm">White</span>
                </label>
                <label class="flex items-center space-x-1">
                  <input 
                    type="radio" 
                    bind:group={humanColor} 
                    value="b"
                    on:change={() => restart()}
                  />
                  <span class="text-sm">Black</span>
                </label>
              </div>
            </div>
          {/if}
        </div>
        
        <Separator />
        
        <div class="space-y-3">
          <div class="flex items-center space-x-2">
            <input 
              type="checkbox" 
              id="opening-variety" 
              bind:checked={useOpeningVariety}
              on:change={() => invoke('set_opening_variety', { enabled: useOpeningVariety })}
              class="rounded"
            />
            <label for="opening-variety" class="text-sm font-medium">
              Opening variety
            </label>
          </div>
          <p class="text-xs text-gray-500 pl-6">
            Uses different popular openings instead of always playing the "best" first moves
          </p>
        </div>
        
        <Separator />
        
        <div class="space-y-2">
          <label for="depth-input" class="text-sm font-medium">Search Depth</label>
          <div class="flex gap-2 items-center">
            <Input 
              id="depth-input" 
              type="number" 
              min="1" 
              max="6" 
              bind:value={engineDepth} 
              class="w-20"
            />
            <span class="text-xs text-gray-500">
              (1-6, higher = stronger but slower)
            </span>
          </div>
        </div>
        
        <Separator />
        
        {#if lastEngineStats.positions > 0}
          <div class="space-y-2">
            <h3 class="text-sm font-medium">Last Engine Analysis</h3>
            <div class="bg-gray-50 p-3 rounded text-xs space-y-1">
              <p><span class="font-medium">Best Move:</span> {lastEngineStats.bestMove}</p>
              <p><span class="font-medium">Depth:</span> {lastEngineStats.depth}</p>
              <p><span class="font-medium">Time:</span> {lastEngineStats.timeMs}ms</p>
              <p><span class="font-medium">Positions:</span> {lastEngineStats.positions.toLocaleString()}</p>
              <p><span class="font-medium">Speed:</span> {Math.round(lastEngineStats.positionsPerSecond).toLocaleString()} pos/sec</p>
            </div>
          </div>
          
          <Separator />
        {/if}
      </CardContent>
    </Card>

    <Card>
      <CardHeader>
        <CardTitle>Game Controls</CardTitle>
      </CardHeader>
      <CardContent class="space-y-4">
        <div class="space-y-2">
          <label for="fen-input" class="text-sm font-medium">FEN Position</label>
          <div class="flex gap-2">
            <Input id="fen-input" bind:value={customFen} placeholder="Enter FEN string" />
            <Button onclick={() => setFen()} variant="outline">Set</Button>
          </div>
        </div>
        
        <Separator />
        
        <div class="space-y-2">
          <h3 class="text-sm font-medium">Current Position</h3>
          <p class="text-xs break-all bg-gray-100 p-2 rounded">{fen}</p>
        </div>
      </CardContent>
    </Card>
    
    {#if import.meta.env.DEV}
      <Card>
        <CardHeader>
          <CardTitle>Debug Information</CardTitle>
        </CardHeader>
        <CardContent class="space-y-2 text-xs">
          <p><span class="font-medium">Board FEN:</span> {debugInfo.boardFen}</p>
          <p><span class="font-medium">Engine FEN:</span> {debugInfo.engineFen}</p>
          <p><span class="font-medium">Selected Piece:</span> {debugInfo.piece}</p>
          <p><span class="font-medium">Engine Piece:</span> {debugInfo.enginePiece}</p>
          <p><span class="font-medium">Legal Moves:</span> {debugInfo.legalMoves.join(', ')}</p>
        </CardContent>
      </Card>
    {/if}
  </div>
</div> 