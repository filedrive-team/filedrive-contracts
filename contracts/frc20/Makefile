.PHONY: build
cid = "bafk2bzacea3qligdipzpcsg6w65zch4upci3ay2exqwpsebb4gx6uiyicjyuc"
actor_address = 't01048'
invoke = lotus chain invoke

build:
	cargo build
install-actor: build
	lotus chain install-actor ../../target/debug/wbuild/frc20/frc20.compact.wasm

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