<script lang="ts">
	import { goto } from '$app/navigation';
	import { invoke } from '@tauri-apps/api';
	import PasswordInput from '$lib/components/PasswordInput.svelte';
	import type { Client, TauriResponse } from '$lib/types/types';
	import { onMount } from 'svelte';

	let username: string = '';
	let cUser: string = 'bg-secondary-800 border-secondary-800 focus:bg-secondary-900';
	let cUserError: string = 'invisible';
	let cUserMsg: string = 'This field cannot be empty!';

	let password: string = '';
	let cPassword: string = 'bg-secondary-800 border-secondary-800 focus:bg-secondary-900';
	let cPasswordError: string = 'invisible';
	let cPasswordMsg: string = 'This field cannot be empty!';

	const validateUserEmpty = () => {
		if (username.length == 0) {
			cUserError = 'text-error';
			cUser =
				'placeholder:text-error focus:border-secondary-800 focus:bg-secondary-900 bg-error border-error';
			return false;
		} else {
			cUserError = 'invisible';
			cUser = 'bg-secondary-800 border-secondary-800 focus:bg-secondary-900';
			return true;
		}
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
		let canRegister: boolean = false;

		if (validateUserEmpty()) {
			canRegister = true;
		} else {
			canRegister = false;
		}

		if (validatePasswordEmpty()) {
			canRegister = true;
		} else {
			canRegister = false;
		}

		if (canRegister) {
			let result: string = await invoke('login', { username, masterPassword: password });
			let data: TauriResponse = JSON.parse(result);

			if (data.status === 400) {
				cUser =
					'placeholder:text-error focus:border-secondary-800 focus:bg-secondary-900 bg-error border-error';
				cPasswordError = 'text-error';
				cPassword =
					'placeholder:text-error focus:border-secondary-800 focus:bg-secondary-900 bg-error border-error';
				cPasswordMsg = 'Username or password incorrect';
			}

			if (data.status === 200 && data.data) {
				let clientData: Client = {
					id: data.data.id,
					username: data.data.username,
					items: data.data.items
				};
				await localStorage.setItem('client', JSON.stringify(clientData));
				goto('/');
			}
		}
	};

	onMount(() => {
		let client = localStorage.getItem('client');
		if (client) {
			goto('/');
		}
	});
</script>

<div
	class="absolute top-0 left-0 w-full h-full bg-primary-1000 overflow-hidden flex justify-center"
>
	<form
		class="flex flex-col w-[50%] lg:w-[30%] h-[70%] mt-40"
		on:submit|preventDefault={handleLogin}
	>
		<div class="flex items-center justify-center w-full mb-10">
			<h1 class="flex text-3xl font-bold mt-10">Login</h1>
		</div>

		<div class="w-full relative">
			<a class="text-hover text-xs hover:text-dark" href="recover">Forgot Master Password?</a>
			<input
				on:keyup={validateUserEmpty}
				class={`${cUser} w-full outline-none border rounded-md mt-2 mb-2  h-fit p-2`}
				type="text"
				placeholder="Username"
				bind:value={username}
			/>
			<span class={cUserError}>{cUserMsg}</span>

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
				type="submit"
				class="bg-primary-600 w-[25%] max-w-[100px] text-center border-primary-700 active:scale-90 hover:bg-primary-700 border duration-200 rounded-md p-2"
				>Login</button
			>
		</div>
		<div class="text-center text-lg mt-4">
			New here? <a href="register" class="text-hover">Register</a>
		</div>
	</form>
</div>
