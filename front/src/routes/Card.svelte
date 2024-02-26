<script lang="ts">
	export let header: String;
	export let content: String;
	export let button: String;

	import { createEventDispatcher } from 'svelte';
	const dispatch = createEventDispatcher();
</script>

<div class="card">
	<p class="header">{header}</p>
	<p class="content">{@html content}</p>
	<button on:click={() => dispatch('buttonClicked', {})} class="button">{button}</button>
</div>

<style>
	.card {
		width: 300px;
		background-image: radial-gradient(100% 100% at 100% 0, #5adaff 0, #5468ff 100%);
		gap: 20px;
		padding: 20px;
		position: relative;
		inline-size: 250px;
		overflow-wrap: break-word;
	}

	.header {
		font-size: 1.5em;
		font-weight: 600;
		color: rgb(241, 241, 241);
		/* text-shadow: 1px 1px 0 #000; */
		z-index: 2;
	}

	.content {
		font-size: 0.9em;
		color: rgb(241, 241, 241);
		/* text-shadow: 1px 1px 0 #000; */
		z-index: 2;
	}

	.button {
		background-color: #fff;
		background-image: none;
		color: #5468ff;
		border-radius: 6px;
		box-shadow: rgba(45, 35, 66, 0.4) 0 2px 4px, rgba(45, 35, 66, 0.3) 0 7px 13px -3px,
			rgba(58, 65, 111, 0.5) 0 -3px 0 inset;
		box-sizing: border-box;
	}
</style>
