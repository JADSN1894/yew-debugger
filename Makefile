DIST_DIR=./crx
BACKGROUND_DIR=./background
PANEL_DIR=./panel

.PHONY: dev release clean

help:
	@printf "\n"
	@printf "Makefile Help\n"
	@printf "======== ====\n"
	@printf "\n"
	@printf "\t$(MAKE)\t\t- This message!\n"
	@printf "\t$(MAKE) help\t- This message!\n"
	@printf "\t$(MAKE) release\t- Generate release artifact\n"
	@printf "\t$(MAKE) dev\t- Generate development artifact\n"
	@#printf "\t$(MAKE) tests\t- Compile and run tests\n"
	@#printf "\t$(MAKE) audit\t- Check dependencies licenses and disclosured vulnerabilities\n"
	@printf "\t$(MAKE) clean\t- Clean compilation files and artifact folder: '$(DIST_DIR)'\n"
	@printf "\n"
	@printf "   If you don't know what to choose, type:\n"
	@printf "\n"
	@printf "\t$(MAKE) release\n"
	@printf "\n"

# Build
build_dev: clean
	$(MAKE) -C background dev
	$(MAKE) -C panel dev

build_release: clean
	$(MAKE) -C background release
	$(MAKE) -C panel release

distrib:
	@mkdir -vp $(DIST_DIR)
	@cp -v manifest.json $(DIST_DIR)/
	@cp -v devtools.html $(DIST_DIR)/
	@cp -v devtools.js $(DIST_DIR)/
	@cp -v background.js $(DIST_DIR)/
	@cp -v content-script.js $(DIST_DIR)/
	@cp -rv $(PANEL_DIR)/dist/assets $(DIST_DIR)/
	@cp -rv $(PANEL_DIR)/dist/index.html $(DIST_DIR)/panel.html
	@cp -rv $(BACKGROUND_DIR)/pkg $(DIST_DIR)/background


# Clean
clean_build:
	$(MAKE) -C background clean
	$(MAKE) -C panel clean

clean_distrib:
	@rm -rfv $(DIST_DIR)


# Public
dev: build_dev distrib

release: build_release distrib

clean: clean_build clean_distrib

# Examples yew-app
debug_examples_yew_app: 
	$(MAKE) -C examples/yew-debugger-counter-layout serve_debug

release_examples_yew_app: 
	$(MAKE) -C examples/yew-debugger-counter-layout serve_release