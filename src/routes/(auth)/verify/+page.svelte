<script lang="ts">
	import { goto } from '$app/navigation';
	import { invoke } from '@tauri-apps/api';
	import PasswordInput from '$lib/components/PasswordInput.svelte';
	import { onMount } from 'svelte';
	import { localToStore } from '$lib/utils';
	import ClientStore from '$lib/stores/ClientStore';

	let password: string = '';
	let cPassword: string = 'bg-secondary-800 border-secondary-800 focus:bg-secondary-900';
	let cPasswordError: string = 'invisible';
	let cPasswordMsg: string = 'This field cannot be empty!';

	const logout = () => {
		localStorage.removeItem('client');
		goto('/login');
	};

	const validatePasswordEmpty = () => {
		if (password.length == 0) {
			cPasswordError = 'text-error';
			cPassword =
				'placeholder:text-error focus:border-secondary-800 focus:bg-secondary-900 bg-error border-error';
			cPasswordMsg = 'This field cannot be empty!';
			return false;
		} else {
			cPasswordError = 'invisible';
			cPassword = 'bg-secondary-800 border-secondary-800 focus:bg-secondary-900';
			return true;
		}
	};

	const handleLogin = async () => {
		let canSubmit: boolean = false;

		if (validatePasswordEmpty()) {
			canSubmit = true;
		} else {
			canSubmit = false;
		}

		if (canSubmit) {
			let result: string = await invoke('login', {
				username: $ClientStore.username,
				masterPassword: password
			});
			let data: TauriResponse = JSON.parse(result);

			if (data.status === 400) {
				cPasswordError = 'text-error';
				cPassword =
					'placeholder:text-error focus:border-secondary-800 focus:bg-secondary-900 bg-error border-error';
				cPasswordMsg = 'Password incorrect';
			}

			if (data.status === 200 && data.data) {
				goto('/');
			}
		}
	};

	onMount(async () => {
		await localToStore(ClientStore, 'client', null);
	});
</script>

{#if $ClientStore}
	<div
		class="absolute top-0 left-0 w-full h-full bg-primary-1000 overflow-hidden flex justify-center"
	>
		<form
			class="flex flex-col w-[50%] lg:w-[30%] h-[70%] mt-40"
			on:submit|preventDefault={handleLogin}
		>
			<div class="flex flex-col items-center justify-center w-full mb-10">
				<h1 class="flex text-3xl font-bold mt-10">Enter Master Password</h1>
				<span class="mt-4"
					>Logged in as: <span class="text-hover font-bold text-md">{$ClientStore.username}</span
					></span
				>
			</div>

			<div class="w-full relative">
				<a class="text-hover text-xs hover:text-dark" href="recover">Forgot Master Password?</a>

				<PasswordInput
					on:keyup={validatePasswordEmpty}
					bind:input={password}
					msg={cPasswordMsg}
					placeholder={'Master Password'}
					bind:cInput={cPassword}
					bind:cError={cPasswordError}
				/>
			</div>
			<div class="w-full flex justify-end">
				<button
					on:click={logout}
					type="button"
					class="bg-primary-600 w-[25%] max-w-[100px] mr-4 text-center border-primary-700 active:scale-90 hover:bg-primary-700 border duration-200 rounded-md p-2"
					>Logout</button
				>
				<button
					type="submit"
					class="bg-primary-600 w-[25%] max-w-[100px] text-center border-primary-700 active:scale-90 hover:bg-primary-700 border duration-200 rounded-md p-2"
					>Unlock</button
				>
			</div>
		</form>
	</div>
{/if}
