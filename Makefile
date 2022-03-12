setup:
	@pip install gitlint
	@pip install pre-commit
	@pre-commit install
	@go get -u github.com/git-chglog/git-chglog/cmd/git-chglog

changelog:
	@git-chglog -o CHANGELOG.md
