DIST_DIR=./dist

.PHONY: wasm clean

help:
	@printf "\n"
	@printf "Makefile Help\n"
	@printf "======== ====\n"
	@printf "\n"
	@printf "\t$(MAKE)\t\t- This message!\n"
	@printf "\t$(MAKE) help\t- This message!\n"
	@printf "\t$(MAKE) release\t- Generate release artifact\n"
	@printf "\t$(MAKE) dev\t- Generate development artifact\n"
	@printf "\t$(MAKE) tests\t- Compile and run tests\n"
	@printf "\t$(MAKE) audit\t- Check dependencies licenses and disclosured vulnerabilities\n"
	@printf "\t$(MAKE) clean\t- Clean compilation files and artifact folder: '$(DIST_DIR)'\n"
	@printf "\n"
	@printf "   If you don't know what to choose, type:\n"
	@printf "\n"
	@printf "\t$(MAKE) release\n"
	@printf "\n"

pre_build:
	@npm install

dev: clean pre_build
	$(MAKE) -C wasm dev
	@npm run dev

release: clean pre_build
	$(MAKE) -C wasm release
	@npm run build

clean:
	$(MAKE) -C wasm clean
	@rm -rfv $(DIST_DIR)