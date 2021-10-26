<script lang="ts">
	import { onMount, setContext } from 'svelte';
	import AssetLoader from './modules/asset/AssetLoader.svelte';
	import Debug from './modules/debug/Debug.svelte';
	import { AssetContext, assetKey, initialAssetState } from './modules/asset'
	import { DebugContext, debugKey, initialDebugState } from './modules/debug'
	
	setContext<AssetContext>(assetKey, initialAssetState);
	setContext<DebugContext>(debugKey, initialDebugState);

	let assetData: Uint8Array | null = null;
	const { data } = initialAssetState;
	data.subscribe(data => {
		if (!data) return;
		assetData = data;
		shrm.main(assetData);
	});

	let shrm: any;
	onMount(async () => {
		let loadedWasmModule = false;
		while (!loadedWasmModule) {
			if ((window as any).shrm) {
				loadedWasmModule = true;
				shrm = (window as any).shrm;
			} else {
				await new Promise(resolve => setTimeout(resolve, 100));
			}
		}

		const loadingIndicator = document.querySelector('#loading-indicator');
		if (loadingIndicator) {
			loadingIndicator.remove();
		}
	})
</script>

<div class="app">
	{#if assetData}
		<Debug />
	{:else}
		<AssetLoader />
	{/if}
</div>
<canvas id="canvas">
</canvas>

<style>
	.app {
		position: relative;
		display: grid;
	}
	#canvas {
		position: absolute;
		top: 0;
		left: 0;
		z-index: -1;
		width: 1280px;
		height: 720px;
	}
</style>
