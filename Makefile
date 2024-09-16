DIRS := $(wildcard */.)

.PHONY: clean c
clean c:
	for DIR in $(DIRS) ; do \
		( $(MAKE) c --directory=$$DIR ) || exit $$? ; \
	done