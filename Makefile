RELEASE_DIR := target/release
DEV_DIR := target/debug

TM := taskmaster
DEP := $(addsuffix .d, $(TM))

DEV_TM := $(addprefix $(DEV_DIR)/, $(TM))
RELEASE_TM := $(addprefix $(RELEASE_DIR)/, $(TM))

SOURCES_FILE := \
	main.rs \
	cli.rs \
	server/relaunch.rs \
	server/default.rs \
	server/signal.rs \
	server/worker.rs \
	server/task.rs \
	server/reader.rs \
	server/error.rs \
	server/mod.rs \
	server/watcher.rs \
	server/state.rs \
	server/listener.rs \
	client/editor.rs \
	client/history.rs \
	client/mod.rs \
	shared/logger.rs \
	shared/mod.rs \

TASKMASTER_DEP := $(addprefix src/, $(SOURCES_FILE))

VENV_DIR := venv

.PHONY: all check clean clean-dev clean-release re re-dev re-release build-release setup-testing-env
all: $(DEV_TM) $(RELEASE_TM)

build-release: $(RELEASE_TM)

setup-testing-env: | $(VENV_DIR)
	python3 -m venv $(VENV_DIR)
	. $(VENV_DIR)/bin/activate; pip3 install -r requirement.txt
	echo "Don't forget to load the virtual env ( source $(VENV_DIR)/bin/activate )"

check: $(RELEASE_TM)
	. $(VENV_DIR)/bin/activate; behave

clean: clean-dev clean-release
re: re-dev re-release

clean-dev:
	$(RM) -rf $(DEV_DIR)

clean-release:
	$(RM) -rf $(RELEASE_DIR)

re-dev:
	$(MAKE) clean-dev
	$(MAKE) $(DEV_TM)

re-release:
	$(MAKE) clean-release
	$(MAKE) $(RELEASE_TM)

$(DEV_TM): $(TASKMASTER_DEP)
	cargo build

$(RELEASE_TM): $(TASKMASTER_DEP)
	cargo build --release
