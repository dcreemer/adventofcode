# helper functions for AoC
import os.path


def read_raw_input_data(src_file: str) -> str:
    ipath = os.path.join(os.path.dirname(src_file), "input.txt")
    with open(ipath, "r") as f:
        data = f.read()
    return data
