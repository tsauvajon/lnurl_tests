install:
	cargo build
	cd front && npm install

run:
	cd front && npm run dev -- --open
