RCMD = cargo publish
initial_commit:
	git add -A
	git commit -m "Initial work on v$(VERS)"
	git push
git:
	git add -A
	git commit -m "$(MSG)"
	git push

publish:
	cd texcore_derive && $(RCMD)
	cd texcore_traits && $(RCMD)
	cargo release --execute

all:
	make git
	make publish