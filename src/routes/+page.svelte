<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { Chess } from 'svelte-chess';
	import { Button } from '$lib/components/ui/button';

	let fen = $state('rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1'); // Starting position FEN

	let chess: any;
	let moveNumber: number = $state(0);
	let turn: 'w' | 'b' = $state('w');
	let history: string[] = $state([]);
	function flipBoard() {
		chess.toggleOrientation();
	}

	async function getEngineMove() {
		var newFen: string = await invoke('get_engine_move', { depth: 2 });
		chess.load(newFen);
	}

	async function resetBoard() {
		await invoke('restart_game');
		chess.reset();
	}

	async function makeRandomMove() {
		fen = await invoke('make_random_move');
		chess.load(fen);
	}


	async function undoMove() {
		await invoke('undo_move');
		chess.undo();
	}
</script>

<div class="chessboard-container">
	<div class="chessboard-wrapper">
		<Chess bind:this={chess} bind:fen bind:moveNumber bind:turn bind:history />
	</div>
	<div class="mt-4 flex flex-col space-y-2">
		<Button on:click={flipBoard} class="w-full">Flip Board</Button>
		<Button on:click={resetBoard} class="w-full">Reset Board</Button>
		<Button on:click={undoMove} class="w-full">Undo Move</Button>
		<Button on:click={getEngineMove} class="w-full">Get Engine Move</Button>
		<Button on:click={makeRandomMove} class="w-full">Make Random Move</Button>
	</div>
	<div class="mt-4">
		<p>Move: {moveNumber}</p>
		<p>Turn: {turn}</p>
		<p>Last move: {history[history.length - 1] || 'None'}</p>
		<p>Last five moves: {history.slice(-5).join(', ')}</p>
	</div>
</div>

<style>
	.chessboard-container {
		width: 100%;
		height: 100vh;
		display: flex;
		justify-content: center;
		align-items: center;
		padding: 20px;
		box-sizing: border-box;
	}

	.chessboard-wrapper {
		width: 100%;
		height: 100%;
		max-width: 80vh;
		max-height: 80vw;
		aspect-ratio: 1 / 1;
	}

	:global(.chessboard-wrapper > div) {
		width: 100% !important;
		height: 100% !important;
	}
</style>
