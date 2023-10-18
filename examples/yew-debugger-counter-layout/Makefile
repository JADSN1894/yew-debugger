DIST_DIR=./dist
NODE_MODLUES=./node_modules
PACKAGE_LOCK_JSON=./package-lock.json

.PHONY: clean

help:
	@printf "\n"
	@printf "Makefile Help\n"
	@printf "======== ====\n"
	@printf "\n"
	@printf "\t$(MAKE)\t\t- This message!\n"
	@printf "\t$(MAKE) help\t- This message!\n"
	@printf "\t$(MAKE) serve_debug\t- Generate release artifact\n"
	@printf "\t$(MAKE) serve_release\t- Generate development artifact\n"
	@printf "\t$(MAKE) clean\t- Remove the : '$(NODE_MODLUES)' and '$(PACKAGE_LOCK_JSON)' folders  \n"
	@printf "\n"
	@printf "   If you don't know what to choose, type:\n"
	@printf "\n"
	@printf "\t$(MAKE) serve_debug\n"
	@printf "\n"

pre_build:
	@npm install

dev: clean pre_build
	@npm run dev

serve_debug_build: clean pre_build
	@npm run debug 

serve_debug: clean pre_build
	@make serve_debug_build && \
    python3 -m http.server --directory $(DIST_DIR)

serve_release_build: clean pre_build
	@npm run build 

serve_release:
	@make serve_release_build && \
    python3 -m http.server --directory $(DIST_DIR)

clean:
	@rm -rfv $(NODE_MODLUES) $(PACKAGE_LOCK_JSON)
	