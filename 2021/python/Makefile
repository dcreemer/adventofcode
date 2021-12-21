test:
	poetry run pytest -v .

code-check:
	-poetry run mypy aoc2021
	-find . \( -not -path ./.venv/\* \) -name "*.py" -exec poetry run pflake8 {} \;

format-check:
	-poetry run autopep8 --diff -r .
	-poetry run black --diff -t py39 .

check: code-check format-check

run:
	poetry run one
	poetry run two
	poetry run three
	poetry run four
	poetry run five
	poetry run six
	poetry run seven
	poetry run eight
	poetry run nine
	poetry run ten
	poetry run eleven
	poetry run twelve
	poetry run thirteen
	poetry run fourteen
	poetry run fifteen
	poetry run sixteen
	poetry run seventeen
	poetry run eighteen
	poetry run nineteen
	poetry run twenty
	poetry run twentyone

clean:
	rm -rf .pytest_cache -rf .mypy_cache
	-find . \( -not -path ./.venv/\* \) -name "__pycache__" -exec rm -rf {} \;

pristine: clean
	rm -rf .venv

boot:
	POETRY_VIRTUALENVS_IN_PROJECT=true poetry install --no-root
