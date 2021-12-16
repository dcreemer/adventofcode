# tests for day Sixteen

from ..sixteen import part_one_calc, part_two_calc


def test_part_one() -> None:
    assert part_one_calc("8A004A801A8002F478") == 16
    assert part_one_calc("620080001611562C8802118E34") == 12
    assert part_one_calc("C0015000016115A2E0802F182340") == 23
    assert part_one_calc("A0016C880162017C3686B18A3D4780") == 31


def test_part_two() -> None:
    assert part_two_calc("C200B40A82") == 3
    assert part_two_calc("04005AC33890") == 54
    assert part_two_calc("880086C3E88112") == 7
    assert part_two_calc("CE00C43D881120") == 9
    assert part_two_calc("D8005AC2A8F0") == 1
    assert part_two_calc("F600BC2D8F") == 0
    assert part_two_calc("9C005AC2F8F0") == 0
    assert part_two_calc("9C0141080250320F1802104A08") == 1
