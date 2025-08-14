

debug:
	cargo build


release:
	cargo build --release

install:
	sudo  mkdir -p /opt/mini-server
	sudo  cp    -r target/release/mini-server    /opt/mini-server
	sudo  chown -R rustsvc:rustsvc               /opt/mini-server
	sudo  cp       services/mini-server.service  /etc/systemd/system/mini-server.service


