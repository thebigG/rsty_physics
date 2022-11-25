.ONESHELL:

buid:
	cd rsty_physics && cargo build

build_image: 
	docker build . -t rsty_physics:rsty_physics

login:
	docker login
push_container: login build_image
	docker tag rsty_physics:rsty_physics thebigg1/rsty_physics
	docker push thebigg1/rsty_physics

