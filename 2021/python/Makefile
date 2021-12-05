test:
	poetry run pytest -v .

code-check:
	-poetry run mypy aoc2021
	-find . \( -not -path ./.venv/\* \) -name "*.py" -exec poetry run pflake8 {} \;

format-check:
	-poetry run autopep8 --diff -r .
	-poetry run black --diff -t py39 .

check: code-check format-check

run-one:
	poetry run one

clean:
	rm -rf .pytest_cache -rf .mypy_cache
	-find . \( -not -path ./.venv/\* \) -name "__pycache__" -exec rm -rf {} \;

pristine: clean
	rm -rf .venv

boot:
	POETRY_VIRTUALENVS_IN_PROJECT=true poetry install --no-root
