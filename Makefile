initial_commit:
	git add -A
	git commit -m "Initial work on v$(VERS)"
	git push
git:
	git add -A
	git commit -m "$(MSG)"
	git push

publish:
	make git
	cargo release --execute
