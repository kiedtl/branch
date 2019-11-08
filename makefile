# 
# branch: a fast, user-friendly version of tree(1)
# https://github.com/lptstr/branch
#

# variables
PLATFORM ?= 
CARGOPTS ?= --target ${TRIPLE} -j$(nproc) --target-dir build
CARGOBIN ?= cargo

CARGOPT_RELEASE ?= --release

PREFIX ?= /usr
DESTDIR ?= /bin

# targets
all: options debug

options:
	@echo "OPTIONS:"
	@echo "\tCC\t=\t${CARGOBIN}"
	@echo "\tCCFLAGS\t=\t${CARGOPTS}"
	@echo ""

debug: build/debug/branch

release: build/release/branch

dev-install: debug
	install -m 755 "build/release/branch" "${PREFIX}${DESTDIR}/branch"

install: release
	install -m 755 "build/release/branch" "${PREFIX}${DESTDIR}/branch"

uninstall:
	rm -f "${PREFIX}${DESTDIR}/branch"

build/debug/branch:
	@echo "CC ${CARGOPT}"
	@${CARGOBIN} ${CARGOPT}

build/release/branch:
	@echo "CC ${CARGOPT} ${CARGOPT_RELEASE}"
	@${CARGOBIN} ${CARGOPT} ${CARGOPT_RELEASE}

.PHONY: all options debug release dev-install install uninstall
