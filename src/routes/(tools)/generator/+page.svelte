<script lang="ts">
	import { generatePassword, localToStore } from '$lib/utils';
	import { onMount } from 'svelte';
	import TableBtn from '$lib/components/TableBtn.svelte';
	import type { Settings } from '$lib/types/types';
	import SettingStore from '$lib/stores/SettingStore';

	let password: string = '';
	let length: number = 16;
	let upper: boolean = false;
	let number: boolean = false;
	let symbol: boolean = false;

	const toggleBtn = (name: string) => {
		if (name == 'upper') {
			upper = !upper;
		} else if (name == 'number') {
			number = !number;
		} else if (name == 'symbol') {
			symbol = !symbol;
		}

		update();
	};

	const update = async () => {
		$SettingStore.length = length;
		$SettingStore.upper = upper;
		$SettingStore.number = number;
		$SettingStore.symbol = symbol;

		localStorage.setItem('settings', JSON.stringify($SettingStore));
		password = await generatePassword($SettingStore);
	};

	onMount(() => {
		localToStore(SettingStore, 'settings');
		length = $SettingStore.length;
		upper = $SettingStore.upper;
		number = $SettingStore.number;
		symbol = $SettingStore.symbol;
		update();
	});
</script>

<div class="flex justify-center h-full">
	<div class="w-[90%]">
		<div class="flex flex-col mt-20 h-[5%] items-center justify-center">
			<h1 class="flex text-3xl font-bold">Password Generator</h1>
			<span class="mt-2 text-hover text-sm"
				>Your configuration here affects password generations in the app</span
			>
		</div>
		<div class="w-full mt-10 drop-shadow-xl h-[70%] rounded-md">
			<div class="flex flex-col text-lg w-full h-full items-center">
				<div
					class="flex flex-col justify-center items-center mt-10 h-[40%] w-[60%] text-2xl bg-secondary-900"
				>
					<div class="p-2 w-full h-full xl:flex xl:justify-center xl:items-center break-words">
						{password}
					</div>
					<div class="flex justify-end text-sm w-full">
						<TableBtn message="Copy" content={password}>
							<svg
								class="hover:text-hover"
								fill="currentColor"
								xmlns="http://www.w3.org/2000/svg"
								height="1.2em"
								viewBox="0 0 512 512"
								><!--! Font Awesome Free 6.4.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. --><path
									d="M272 0H396.1c12.7 0 24.9 5.1 33.9 14.1l67.9 67.9c9 9 14.1 21.2 14.1 33.9V336c0 26.5-21.5 48-48 48H272c-26.5 0-48-21.5-48-48V48c0-26.5 21.5-48 48-48zM48 128H192v64H64V448H256V416h64v48c0 26.5-21.5 48-48 48H48c-26.5 0-48-21.5-48-48V176c0-26.5 21.5-48 48-48z"
								/></svg
							>
						</TableBtn>
					</div>
				</div>

				<div class="flex flex-col mt-10 w-[60%]">
					<div class="flex items-center">
						<span>Password Length</span>
						<input
							min="8"
							max="40"
							class="bg-secondary-800 ml-auto outline-none border border-secondary-800 focus:bg-secondary-900 rounded-md h-fit p-2"
							type="range"
							on:change={update}
							bind:value={length}
						/>
						<div class="ml-4 w-[11px]">{length}</div>
					</div>
					<div class="flex items-center">
						<span>Use capital letters</span>
						<input
							class="bg-secondary-800 ml-auto outline-none border border-secondary-800 focus:bg-secondary-900 rounded-md h-fit p-2"
							type="checkbox"
							on:click={() => toggleBtn('upper')}
							bind:checked={upper}
						/>
					</div>

					<div class="flex items-center">
						<span>Use digits</span>
						<input
							class="bg-secondary-800 ml-auto outline-none border border-secondary-800 focus:bg-secondary-900 rounded-md h-fit p-2"
							type="checkbox"
							on:click={() => toggleBtn('number')}
							bind:checked={number}
						/>
					</div>

					<div class="flex items-center">
						<span>Use symbols</span>
						<input
							class="bg-secondary-800 ml-auto outline-none border border-secondary-800 focus:bg-secondary-900 rounded-md h-fit p-2"
							type="checkbox"
							on:click={() => toggleBtn('symbol')}
							bind:checked={symbol}
						/>
					</div>
				</div>

				<div class="flex mt-10 w-[25%] justify-evenly">
					<button
						on:click={update}
						class="hover:bg-primary-700 w-1/3 min-w-[90px] active:scale-90 border duration-200 border-primary-600 rounded-md p-2 flex justify-center items-center bg-primary-500"
						>Generate</button
					>
				</div>
			</div>
		</div>
	</div>
</div>
