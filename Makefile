CRX_DIR=./crx
BACKGROUND_DIR=./background
PANEL_DIR=./panel
ARTIFACTS_DIR=./artifacts
ARTIFACTS_YEW_APP_DIST_DIR=$(ARTIFACTS_DIR)/dist
EXAMPLE_YEW_APP_DIR=./examples/yew-debugger-counter-layout
EXAMPLE_YEW_APP_DIST_DIR=./$(EXAMPLE_YEW_APP_DIR)/dist

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
	@printf "\t$(MAKE) clean\t- Clean compilation files and artifact folder: '$(CRX_DIR)'\n"
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
	@mkdir -vp $(CRX_DIR)
	@cp -v manifest.json $(CRX_DIR)/
	@cp -v devtools.html $(CRX_DIR)/
	@cp -v devtools.js $(CRX_DIR)/
	@cp -v background.js $(CRX_DIR)/
	@cp -v content-script.js $(CRX_DIR)/
	@cp -rv $(PANEL_DIR)/dist/assets $(CRX_DIR)/
	@cp -rv $(PANEL_DIR)/dist/index.html $(CRX_DIR)/panel.html
	@cp -rv $(BACKGROUND_DIR)/pkg $(CRX_DIR)/background

# Clean
clean_build:
	$(MAKE) -C background clean
	$(MAKE) -C panel clean

clean_distrib:
	@rm -rfv $(CRX_DIR)


# Public
dev: build_dev distrib

release: build_release distrib

clean: clean_build clean_distrib

# Examples yew-app
debug_examples_yew_app: 
	$(MAKE) -C $(EXAMPLE_YEW_APP_DIR) serve_debug

release_examples_yew_app: 
	$(MAKE) -C $(EXAMPLE_YEW_APP_DIR) serve_release

# Artifacts
artifacts_pre_setup: 
	@rm -rfv $(ARTIFACTS_DIR)
	@mkdir -vp $(ARTIFACTS_DIR)

artifacts_post_workflow: 
	@cp -rfv $(CRX_DIR) $(ARTIFACTS_DIR)
	@cp -rfv $(EXAMPLE_YEW_APP_DIST_DIR) $(ARTIFACTS_DIR) && python3 -m http.server --directory $(ARTIFACTS_YEW_APP_DIST_DIR)

artifacts_dev:
	@make artifacts_pre_setup
	@make release
	@$(MAKE) -C $(EXAMPLE_YEW_APP_DIR) serve_debug_build
	@make artifacts_post_workflow

artifacts_build:
	@make artifacts_pre_setup
	@make release
	@$(MAKE) -C $(EXAMPLE_YEW_APP_DIR) serve_release_build
	@make artifacts_post_workflow

