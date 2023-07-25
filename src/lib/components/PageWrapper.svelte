<script lang="ts">
	import Results from './Results.svelte';
	export let resultQuery: string | undefined = undefined;
</script>

<main class=" bg-primary-1000 h-screen w-full overflow-hidden">
	{#if resultQuery !== undefined && resultQuery.length > 0}
		<Results bind:resultQuery />
	{:else}
		<slot />
	{/if}
</main>
