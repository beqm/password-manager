<script lang="ts">
	import { goto } from '$app/navigation';

	let username: string = '';
	let cUser: string = 'bg-secondary-800 border-secondary-800 focus:bg-secondary-900';
	let cUserError: string = 'invisible';
	let cUserMsg: string = 'This field cannot be empty!';

	let recoveryCode: string = '';
	let cRecover: string = 'bg-secondary-800 border-secondary-800 focus:bg-secondary-900';
	let cRecoverError: string = 'invisible';
	let cCreateBtn: string = 'bg-primary-700 border-primary-700 cursor-default';

	const validateRecoverEmpty = () => {
		if (recoveryCode.length == 0) {
			cRecoverError = 'text-error';
			cRecover =
				'placeholder:text-error focus:border-secondary-800 focus:bg-secondary-900 bg-error border-error';
			cCreateBtn = 'bg-primary-700 border-primary-700 cursor-default';
			return false;
		} else {
			cRecoverError = 'invisible';
			cRecover = 'bg-secondary-800 border-secondary-800 focus:bg-secondary-900';
			cCreateBtn = 'bg-primary-600 border-primary-700 active:scale-90';
			return true;
		}
	};

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

	const handleLogin = () => {
		let validRecoveryCode: boolean = false;

		if (validateUserEmpty()) {
			validRecoveryCode = true;
		} else {
			validRecoveryCode = false;
		}

		if (validateRecoverEmpty()) {
			validRecoveryCode = true;
		} else {
			validRecoveryCode = false;
		}

		if (validRecoveryCode) {
			goto('/login');
		}
	};

	const bindValue = (e: any) => {
		recoveryCode = e.target.value;
	};
</script>

<div
	class="absolute top-0 left-0 w-full h-full bg-primary-1000 overflow-hidden flex justify-center"
>
	<form
		class="flex flex-col w-[50%] lg:w-[30%] h-[70%] mt-40"
		on:submit|preventDefault={handleLogin}
	>
		<div class="flex items-center justify-center w-full mb-10">
			<h1 class="flex text-3xl font-bold mt-10">Enter Recovery Code</h1>
		</div>

		<div class="w-full relative">
			<a class="text-hover text-xs mb-2 hover:text-dark" href="login">Remembered Master password?</a
			>
			<input
				on:keyup={validateUserEmpty}
				class={`${cUser} w-full mt-2 outline-none border mb-2 rounded-md h-fit p-2`}
				type="text"
				placeholder="Username"
				bind:value={username}
			/>
			<span class={`${cUserError}`}>This field cannot be empty!</span>
			<input
				on:keyup={validateRecoverEmpty}
				class={`${cRecover} w-full outline-none border mt-2 rounded-md mb-2 h-fit p-2`}
				type="text"
				placeholder="Recovery Code"
				bind:value={recoveryCode}
			/>
			<span class={`${cRecoverError}`}>This field cannot be empty!</span>
		</div>
		<div class="w-full flex justify-end">
			<button
				type="submit"
				class={`${cCreateBtn} hover:bg-primary-700 border duration-200 border-primary-200 rounded-md p-2 bg-primary-500`}
				>Enter</button
			>
		</div>
	</form>
</div>
