# unittest for the calculate module

from ..parse import parse_list_to_ints


def test_int_lists() -> None:
    src = """
    1
    2
    3
    4
    """.splitlines()
    assert parse_list_to_ints(src) == [1, 2, 3, 4]
