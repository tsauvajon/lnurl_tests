<script lang="ts">
	let qr_code: String | null = null;
	let invoice: String | null = null;
	let error: String | null = null;

	let qty = 1;
	const unit_price = 200_000;
	$: total = unit_price * qty;
	
	let send_amount = 2000;
	$: usd_send = (send_amount / 1930).toFixed(2);

	async function buy() {
		get_invoice(total)
	}

	async function send() {
		get_invoice(send_amount)
	}

	async function get_invoice(sats) {
		error = null;
		try {
			const res = await fetch('http://127.0.0.1:3987/invoice', {
				method: 'POST',
				headers: {
				"Content-Type": "application/json",
				},
				body: JSON.stringify({
					sats
				})
			})
			if (!res.ok) {
				error = await res.text();
			}
			let resp = await res.json();
			qr_code = resp['qr_code'];
			invoice = resp['data'];
		} catch (e: any) {
			error = e;
		}

	}

	// TODO: animate QR Code https://svelte.dev/tutorial/bind-this
</script>

<h2>Self-generate invoices</h2>

<table>
	<tr>
	<th>Self-checkout demo</th>
	<th>Generate invoice demo</th>
	</tr>

	<tr>
	<td>
	<p>
		Price: {unit_price}<br />
		Quantity: {qty}
		<input type="range" bind:value={qty} min="0" max="10" /><br />
		Total: {total}<br />
		<button on:click={buy}>Buy</button>
	</p>
	</td>

	<td>
	<p>
		Generate invoice:
		<input type="number" bind:value={send_amount} min="20" max="40000" /><br />
		<input type="range" bind:value={send_amount} min="20" max="40000" /><br />
		($ {usd_send})
		<br />
		<button on:click={send}>Generate invoice</button>
	</p>
	</td>
	</tr>
</table>

{#if error}
	<p>Error: {error}</p>
{:else if qr_code}
	<p>QR Code: {@html qr_code}</p>
	<p>Raw Invoice: {invoice}</p>
{:else if invoice}
	<p>Invoice: {invoice}</p>
{/if}


<style>
	input,
	p {
		margin: 6px;
	}

	:global(svg rect) {
		animation: contour 6s ease-in-out infinite;
	}

	@keyframes contour {
		0% {fill:#ef5455}
		50% {fill:#f77f00}
		100% {fill:#ef5455}
	}

	:global(svg path) {
		animation: interieur 6s ease-in-out infinite;
	}

	@keyframes interieur {
		0% {fill:#2b3252}
		50% {fill:#5e412f}
		100% {fill:#2b3252}
	}

	:global(svg) {
		animation: rotate 10s ease-in-out infinite;
	}

	@keyframes rotate {
		0% { transform: rotate3d(0); }
		25% { transform: rotate3d(1, 1, 1, 5deg); }
		50% { transform: rotate3d(0); }
		75% { transform: rotate3d(1, 1, 1, -5deg); }
		100% { transform: rotate3d(0); }
	}
</style>
