DIST_DIR=./dist
NODE_MODULES=./node_modules
PACKAGE_LOCK_JSON=./package-lock.json

.PHONY: clean

help:
	@printf "\n"
	@printf "Makefile Help\n"
	@printf "======== ====\n"
	@printf "\n"
	@printf "\t$(MAKE)\t\t- This message!\n"
	@printf "\t$(MAKE) help\t- This message!\n"
	@printf "\t$(MAKE) debug\t- Generate debug artifact\n"
	@printf "\t$(MAKE) release\t- Generate release artifact\n"
	@printf "\t$(MAKE) clean\t- Remove the folders '[$(NODE_MODULES)]' and '[$(PACKAGE_LOCK_JSON)]'\n"
	@printf "\n"
	@printf "   If you don't know what to choose, type:\n"
	@printf "\n"
	@printf "\t$(MAKE) debug\n"
	@printf "\n"

pre_build:
	@npm install

debug: clean pre_build
	@npm run debug 

release: clean pre_build
	@npm run build 

clean:
	@rm -rfv $(NODE_MODULES) $(PACKAGE_LOCK_JSON)
	