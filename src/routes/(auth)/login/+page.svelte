<script lang="ts">
	import { goto } from '$app/navigation';

	let password: string = '';
	let visibility = false;
	let type: 'text' | 'password' = 'password';

	let cMaster: string = 'bg-secondary-800 border-secondary-800 focus:bg-secondary-900';
	let cErrorMsg: string = 'invisible';
	let cCreateBtn: string = 'bg-primary-700 border-primary-700 cursor-default';

	const validateMaster = () => {
		if (password.length == 0) {
			cErrorMsg = 'text-error';
			cMaster =
				'placeholder:text-error focus:border-secondary-800 focus:bg-secondary-900 bg-error border-error';
			cCreateBtn = 'bg-primary-700 border-primary-700 cursor-default';
			return true;
		} else {
			cErrorMsg = 'invisible';
			cMaster = 'bg-secondary-800 border-secondary-800 focus:bg-secondary-900';
			cCreateBtn = 'bg-primary-600 border-primary-700 active:scale-90';
			return false;
		}
	};

	const handleLogin = () => {
		if (!validateMaster()) {
			goto('/');
		}
	};

	const bindValue = (e: any) => {
		password = e.target.value;
	};

	const hidePassword = () => {
		visibility = false;
		type = 'password';
	};

	const showPassword = () => {
		visibility = true;
		type = 'text';
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
			<h1 class="flex text-3xl font-bold mt-10">Enter Master password</h1>
		</div>

		<div class="w-full relative">
			<a class="text-hover text-xs mb-2 hover:text-dark" href="recover">Forgot Master Password?</a>
			<input
				on:keyup={validateMaster}
				class={`${cMaster} w-full outline-none border rounded-md h-fit p-2`}
				{type}
				placeholder="Master password"
				on:input={bindValue}
			/>
			<span class={`${cErrorMsg} mt-2`}>This field cannot be empty!</span>
			{#if visibility}
				<button on:click={hidePassword}>
					<svg
						class="absolute right-[10px] top-[35px] hover:text-hover"
						fill="currentColor"
						xmlns="http://www.w3.org/2000/svg"
						height="1.2em"
						viewBox="0 0 576 512"
						><!--! Font Awesome Free 6.4.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. --><path
							d="M288 32c-80.8 0-145.5 36.8-192.6 80.6C48.6 156 17.3 208 2.5 243.7c-3.3 7.9-3.3 16.7 0 24.6C17.3 304 48.6 356 95.4 399.4C142.5 443.2 207.2 480 288 480s145.5-36.8 192.6-80.6c46.8-43.5 78.1-95.4 93-131.1c3.3-7.9 3.3-16.7 0-24.6c-14.9-35.7-46.2-87.7-93-131.1C433.5 68.8 368.8 32 288 32zM144 256a144 144 0 1 1 288 0 144 144 0 1 1 -288 0zm144-64c0 35.3-28.7 64-64 64c-7.1 0-13.9-1.2-20.3-3.3c-5.5-1.8-11.9 1.6-11.7 7.4c.3 6.9 1.3 13.8 3.2 20.7c13.7 51.2 66.4 81.6 117.6 67.9s81.6-66.4 67.9-117.6c-11.1-41.5-47.8-69.4-88.6-71.1c-5.8-.2-9.2 6.1-7.4 11.7c2.1 6.4 3.3 13.2 3.3 20.3z"
						/></svg
					>
				</button>
			{:else}
				<button on:click={showPassword}>
					<svg
						class="absolute right-[10px] top-[35px] hover:text-hover"
						fill="currentColor"
						xmlns="http://www.w3.org/2000/svg"
						height="1.2em"
						viewBox="0 0 640 512"
						><!--! Font Awesome Free 6.4.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. --><path
							d="M38.8 5.1C28.4-3.1 13.3-1.2 5.1 9.2S-1.2 34.7 9.2 42.9l592 464c10.4 8.2 25.5 6.3 33.7-4.1s6.3-25.5-4.1-33.7L525.6 386.7c39.6-40.6 66.4-86.1 79.9-118.4c3.3-7.9 3.3-16.7 0-24.6c-14.9-35.7-46.2-87.7-93-131.1C465.5 68.8 400.8 32 320 32c-68.2 0-125 26.3-169.3 60.8L38.8 5.1zM223.1 149.5C248.6 126.2 282.7 112 320 112c79.5 0 144 64.5 144 144c0 24.9-6.3 48.3-17.4 68.7L408 294.5c8.4-19.3 10.6-41.4 4.8-63.3c-11.1-41.5-47.8-69.4-88.6-71.1c-5.8-.2-9.2 6.1-7.4 11.7c2.1 6.4 3.3 13.2 3.3 20.3c0 10.2-2.4 19.8-6.6 28.3l-90.3-70.8zM373 389.9c-16.4 6.5-34.3 10.1-53 10.1c-79.5 0-144-64.5-144-144c0-6.9 .5-13.6 1.4-20.2L83.1 161.5C60.3 191.2 44 220.8 34.5 243.7c-3.3 7.9-3.3 16.7 0 24.6c14.9 35.7 46.2 87.7 93 131.1C174.5 443.2 239.2 480 320 480c47.8 0 89.9-12.9 126.2-32.5L373 389.9z"
						/></svg
					>
				</button>
			{/if}
		</div>
		<div class="w-full flex justify-end">
			<button
				type="submit"
				class={`${cCreateBtn} hover:bg-primary-700 border duration-200 border-primary-600 rounded-md p-2 bg-primary-500`}
				>Login</button
			>
		</div>
	</form>
</div>
