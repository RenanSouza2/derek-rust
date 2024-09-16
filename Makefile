DIRS := $(wildcard */.)

.PHONY: build b
build b:
	for DIR in $(DIRS) ; do \
		( $(MAKE) b --directory=$$DIR ) || exit $$? ; \
	done

.PHONY: clean c
clean c:
	for DIR in $(DIRS) ; do \
		( $(MAKE) c --directory=$$DIR ) || exit $$? ; \
	done