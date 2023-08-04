<script lang="ts">
	import { fade } from 'svelte/transition';
	import { writeText } from '@tauri-apps/api/clipboard';
	import { invoke } from '@tauri-apps/api';

	export let content: string;
	export let message: string;
	export let isLaunch: boolean = false;
	let cHint: boolean = false;

	const copyToClipboard = async () => {
		if (!isLaunch) {
			await writeText(content);
		} else {
			invoke('launch_website', { url: content });
		}
	};

	const showHoverHint = () => {
		cHint = true;
	};

	const closeHoverHint = () => {
		cHint = false;
	};
</script>

<button
	on:click={copyToClipboard}
	on:mouseenter={showHoverHint}
	on:mouseleave={closeHoverHint}
	class="relative active:scale-90 duration-200 m-2"
>
	<slot />
	{#if cHint}
		<input id={message} type="text" class="hidden" value={content} />
		<div
			transition:fade={{ duration: 200 }}
			class="absolute p-2 top-6 left-1/2 transform -translate-x-1/2 bg-primary-900 border border-primary-700 text-sm z-10"
		>
			{message}
		</div>
	{/if}
</button>
