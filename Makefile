RELEASE_DIR := target/release
DEV_DIR := target/debug

TM := taskmaster
DEP := $(addsuffix .d, $(TM))

DEV_TM := $(addprefix $(DEV_DIR)/, $(TM))
RELEASE_TM := $(addprefix $(RELEASE_DIR)/, $(TM))

DEV_DEP := $(addprefix $(DEV_DIR)/, $(DEP))
RELEASE_DEP := $(addprefix $(RELEASE_DIR)/, $(DEP))

VENV_DIR := venv

include $(DEV_DEP)
include $(RELEASE_DEP)

.PHONY: all check clean clean-dev clean-release re re-dev re-release build-release setup-testing-env
all: $(DEV_TM)

build-release: $(RELEASE_TM)

setup-testing-env: | $(VENV_DIR)
	python3 -m venv $(VENV_DIR)
	. $(VENV_DIR)/bin/activate; pip3 install -r requirement.txt
	echo "Don't forget to load the virtual env ( source $(VENV_DIR)/bin/activate )"

check: $(RELEASE_TM)
	behave

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


$(DEV_DEP) $(DEV_TM):
	cargo build

$(RELEASE_DEP) $(RELEASE_TM):
	cargo build --release
