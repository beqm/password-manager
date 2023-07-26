<script lang="ts">
	import '../app.postcss';
	import SearchBar from '$lib/components/SearchBar.svelte';
	import NavItems from '$lib/components/NavItems.svelte';
	import PageWrapper from '$lib/components/PageWrapper.svelte';
	let resultQuery: string | undefined = undefined;
</script>

<div class="flex text-dark">
	<nav class=" bg-primary-900 w-[25%] max-w-[300px] min-w-[200px] border-r border-primary-800">
		<div class="flex flex-col w-full items-center h-1/5">
			<div class="h-[70%] w-full flex items-center justify-center">User info</div>
			<SearchBar bind:resultQuery />
		</div>

		<ul class="h-[80%]">
			<li class="border-b border-t border-primary-800">
				<NavItems />
			</li>
		</ul>
	</nav>

	<PageWrapper bind:resultQuery>
		<slot />
	</PageWrapper>
</div>
