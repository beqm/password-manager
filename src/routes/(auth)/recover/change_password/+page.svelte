<script lang="ts">
	import { goto } from '$app/navigation';
	import { invoke } from '@tauri-apps/api';
	import PasswordInput from '$lib/components/PasswordInput.svelte';
	import type { Client, TauriResponse } from '$lib/types/types';
	import { onMount } from 'svelte';

	let password: string = '';
	let cPassword: string = 'bg-secondary-800 border-secondary-800 focus:bg-secondary-900';
	let cPasswordError: string = 'invisible';
	let cPasswordMsg: string = 'This field cannot be empty!';

	let confirmPassword: string = '';
	let cConfirm: string = 'bg-secondary-800 border-secondary-800 focus:bg-secondary-900';
	let cConfirmError: string = 'invisible';
	let cConfirmMsg: string = 'This field cannot be empty!';

	const validatePasswordEmpty = () => {
		if (password.length == 0) {
			cPasswordError = 'text-error';
			cPassword =
				'placeholder:text-error focus:border-secondary-800 focus:bg-secondary-900 bg-error border-error';
			return false;
		} else {
			cPasswordError = 'invisible';
			cPassword = 'bg-secondary-800 border-secondary-800 focus:bg-secondary-900';
			return true;
		}
	};

	const validateConfirmEmpty = () => {
		if (confirmPassword.length == 0) {
			cConfirmError = 'text-error';
			cConfirm =
				'placeholder:text-error focus:border-secondary-800 focus:bg-secondary-900 bg-error border-error';
			cConfirmMsg = 'This field cannot be empty!';
			return false;
		} else {
			cConfirmError = 'invisible';
			cConfirm = 'bg-secondary-800 border-secondary-800 focus:bg-secondary-900';
			return true;
		}
	};

	const handleLogin = async () => {
		let canRegister: boolean = false;

		if (validatePasswordEmpty()) {
			canRegister = true;
		} else {
			canRegister = false;
		}

		if (validateConfirmEmpty()) {
			canRegister = true;
		} else {
			canRegister = false;
		}

		if (canRegister) {
			let username = await localStorage.getItem('temp_value');
			localStorage.removeItem('temp_value');
			let result: string = await invoke('change_password', { username, masterPassword: password });
			// let data: TauriResponse = JSON.parse(result);
			goto('/login');
		}
	};
</script>

<div
	class="absolute top-0 left-0 w-full h-full bg-primary-1000 overflow-hidden flex justify-center"
>
	<form
		class="flex flex-col w-[50%] xl:w-[30%] h-[70%] mt-40"
		on:submit|preventDefault={handleLogin}
	>
		<div class="flex items-center justify-center w-full mb-10">
			<h1 class="flex text-3xl font-bold mt-10">Change Master Password</h1>
		</div>

		<PasswordInput
			on:keyup={validatePasswordEmpty}
			bind:input={password}
			msg={cPasswordMsg}
			placeholder={'Master Password'}
			bind:cInput={cPassword}
			bind:cError={cPasswordError}
		/>

		<PasswordInput
			on:keyup={validateConfirmEmpty}
			bind:input={confirmPassword}
			msg={cConfirmMsg}
			placeholder={'Confirm Master Password'}
			bind:cInput={cConfirm}
			bind:cError={cConfirmError}
		/>

		<div class="w-full flex justify-end">
			<a
				href="/login"
				class="bg-primary-600 w-[25%] max-w-[100px] mr-4 text-center border-primary-700 active:scale-90 hover:bg-primary-700 border duration-200 rounded-md p-2"
				>Back</a
			>
			<button
				type="submit"
				class="bg-primary-600 w-[25%] max-w-[100px] text-center border-primary-700 active:scale-90 hover:bg-primary-700 border duration-200 rounded-md p-2"
				>Change</button
			>
		</div>
	</form>
</div>
