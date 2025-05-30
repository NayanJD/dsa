# Test command that takes a problem name and language flag
# Usage: make test PROBLEM=twosum LANG=go
.PHONY: test

test:
ifndef PROBLEM
	$(error PROBLEM is not set. Usage: make test PROBLEM=<problem_name> LANG=<language>)
endif
ifndef LANG
	$(error LANG is not set. Usage: make test PROBLEM=<problem_name> LANG=<language>)
endif
	@echo "Testing problem: $(PROBLEM)"
	@echo "Language: $(LANG)"
	@./test.sh test $(PROBLEM) $(LANG)
	
new:
ifndef PROBLEM
	$(error PROBLEM is not set. Usage: make test PROBLEM=<problem_name> LANG=<language>)
endif
	@./test.sh new $(PROBLEM)
				
