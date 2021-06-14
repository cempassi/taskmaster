INSTALL_DIR=/opt/taskmaster

~/.cargo/bin/cargo install --path . --root $INSTALL_DIR

install -v -d $INSTALL_DIR/configs
install -v configs/* $INSTALL_DIR/configs
install -v taskmaster.service /usr/lib/systemd/system/taskmaster.service
