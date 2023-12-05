test:
	poetry run pytest -v .

code-check:
	-poetry run mypy aoc2023
	-poetry run ruff check .
	-poetry run safety check --bare

format-check:
	-poetry run ruff format --diff .

run:
	# poetry run python aoc2023/one/main.py
	poetry run python aoc2023/two/main.py

clean:
	rm -rf htmlcov
	rm -rf .coverage .pytest_cache .mypy_cache .ruff_cache
	-find . \( -not -path ./.venv/\* \) -name "__pycache__" -exec rm -rf {} \;

pristine: clean
	rm -rf .venv

boot:
	POETRY_VIRTUALENVS_IN_PROJECT=true poetry install --no-root
