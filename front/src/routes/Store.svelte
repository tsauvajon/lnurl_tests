<script lang="ts">
	let qty = 1;
	const unit_price = 200_000;
	let qr_code: String | null = null;
	let error: String | null = null;
	$: total = unit_price * qty;

	async function buy() {
		try {
			const res = await fetch(`https://google.com/purchase?price=${total}`);
			if (!res.ok) {
				error = await res.text();
			}
			let resp = await res.json();
			qr_code = resp['qr_code'];
			let id = resp['payment_id'];
		} catch (e: any) {
			error = e;
		}
	}

	// TODO: animate QR Code https://svelte.dev/tutorial/bind-this
</script>

<p>
	<!-- <label> -->
	Price: {unit_price}<br />
	Quantity: {qty}
	<input type="range" bind:value={qty} min="0" max="10" /><br />
	Total: {total}<br />
	<!-- </label> -->
	<br />
	<button on:click={buy}> Buy </button>
</p>

{#if error}
	<p>Error: {error}</p>
{:else if qr_code}
	<p>QR Code: {qr_code}</p>
{/if}

<style>
	input,
	p {
		margin: 6px;
	}
</style>
