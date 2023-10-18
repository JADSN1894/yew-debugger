BACKGROUND_DIR=./background
PANEL_DIR=./panel

EXAMPLE_YEW_APP_DIR=./examples/yew-debugger-counter-layout
EXAMPLE_YEW_APP_DIST_DIR=$(EXAMPLE_YEW_APP_DIR)/dist

ARTIFACT_EXAMPLE_DIR=./dist-example
ARTIFACT_CRX_DIR=./crx


.PHONY: dev release clean

help:
	@printf "\n"
	@printf "Makefile Help\n"
	@printf "======== ====\n"
	@printf "\n"
	@printf "\t$(MAKE)\t\t- This message!\n"
	@printf "\t$(MAKE) help\t- This message!\n"
	@printf "\t$(MAKE) release\t- Generate release artifact\n"
	@printf "\t$(MAKE) demo\t- Generate demo for both extension and example\n"
	@printf "\t$(MAKE) debug\t- Generate debug artifact\n"
	@#printf "\t$(MAKE) tests\t- Compile and run tests\n"
	@#printf "\t$(MAKE) audit\t- Check dependencies licenses and disclosured vulnerabilities\n"
	@printf "\t$(MAKE) clean\t- Clean compilation files and artifact folders: '[$(ARTIFACT_CRX_DIR)]' and '[$(ARTIFACT_EXAMPLE_DIR)]'\n"
	@printf "\n"
	@printf "   If you don't know what to choose, type:\n"
	@printf "\n"
	@printf "\t$(MAKE) release\n"
	@printf "\n"



depend_debug:
	cargo check --target=wasm32-unknown-unknown

depend_release:
	cargo check --target=wasm32-unknown-unknown --release

# Build
build_debug: clean depend_debug
	$(MAKE) -C background debug
	$(MAKE) -C panel debug

build_release: clean depend_release
	$(MAKE) -C background release
	$(MAKE) -C panel release


build_example_debug:
	$(MAKE) -C examples/yew-debugger-counter-layout debug
	@cp -rv $(EXAMPLE_YEW_APP_DIST_DIR) $(ARTIFACT_EXAMPLE_DIR)

build_example_release:
	$(MAKE) -C examples/yew-debugger-counter-layout release
	@cp -rv $(EXAMPLE_YEW_APP_DIST_DIR) $(ARTIFACT_EXAMPLE_DIR)

distrib:
	@mkdir -vp $(ARTIFACT_CRX_DIR)
	@cp -v manifest.json $(ARTIFACT_CRX_DIR)/
	@cp -v devtools.html $(ARTIFACT_CRX_DIR)/
	@cp -v devtools.js $(ARTIFACT_CRX_DIR)/
	@cp -v background.js $(ARTIFACT_CRX_DIR)/
	@cp -v content-script.js $(ARTIFACT_CRX_DIR)/
	@cp -rv $(PANEL_DIR)/dist/assets $(ARTIFACT_CRX_DIR)/
	@cp -rv $(PANEL_DIR)/dist/index.html $(ARTIFACT_CRX_DIR)/panel.html
	@cp -rv $(BACKGROUND_DIR)/pkg $(ARTIFACT_CRX_DIR)/background

# Clean
clean_build:
	$(MAKE) -C background clean
	$(MAKE) -C panel clean
	@cargo clean

clean_distrib:
	@rm -rfv $(ARTIFACT_CRX_DIR)
	@rm -rfv $(ARTIFACT_EXAMPLE_DIR)


# Public
debug: build_debug distrib

release: build_release distrib

clean: clean_build clean_distrib

demo_debug: debug build_example_debug

demo_release: release build_example_debug

demo: demo_release