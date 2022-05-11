.PHONY: build
cid = "bafk2bzacedkaikxtjp5szud3fmefowmwa2wmw52pwkzz6hesnirjmbxumrzs4"
actor_address = 't01042'
invoke = lotus chain invoke

build:
	cargo build
install-actor: build
	lotus chain install-actor target/debug/wbuild/frc20/frc20.compact.wasm

create_actor:
	lotus chain create-actor $(cid)

geneses:
	$(invoke) $(actor_address) 1

balance_of:
	$(invoke) $(actor_address) 2

transfer:
	$(invoke) $(actor_address) 3

approve:
	$(invoke) $(actor_address) 4

allowance:
	$(invoke) $(actor_address) 5