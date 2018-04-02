MAKEFILE_PATH:=$(word $(words $(MAKEFILE_LIST)),$(MAKEFILE_LIST))
ROOT:=$(shell cd $(dir $(MAKEFILE_PATH))/..;pwd)

rust-linux:
	docker build --no-cache --force-rm -t rust-linux -f $(ROOT)/build/Dockerfile.rust.linux $(ROOT)


rust-directfb:
	docker build --no-cache --force-rm -t rust-directfb -f $(ROOT)/build/Dockerfile.rust.directfb $(ROOT)

rust-toolchain:
	docker build --no-cache --force-rm -t rust-toolchain -f $(ROOT)/build/Dockerfile.rust.toolchain $(ROOT)

rust-compiler:
	docker build --no-cache --force-rm -t rust-compiler -f $(ROOT)/build/Dockerfile.rust.compiler $(ROOT)

rust-build:
	docker build --no-cache -t rust-build -f $(ROOT)/build/Dockerfile.rust.build $(ROOT)

rust-shell:
	docker run -t -i --rm -v $(ROOT)/.cargo/git:/build/.cargo/git -v $(ROOT)/.cargo/registry:/build/.cargo/registry -v $(ROOT):/source rust-build  '/bin/bash'

