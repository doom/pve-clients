from itertools import chain
import re
from typing import Iterable, Iterator


def words(ident: str, sep: str = "-") -> list[str]:
    def strip_braces(w: str) -> str:
        return w.lstrip("{").rstrip("}")

    return [strip_braces(w) for w in ident.split(sep)]


WORD_DELIMITERS = re.compile(r"[-_ .]+")
PASCAL_WORD_SPLITTER = re.compile(r"(?<=[a-z])(?=[A-Z])")


def _split_word(w: str) -> Iterable[str]:
    return WORD_DELIMITERS.split(w)


def _split_pascal(w: str) -> Iterable[str]:
    return PASCAL_WORD_SPLITTER.split(w)


def _into_words(ws: Iterable[str]) -> Iterator[str]:
    ws = chain.from_iterable(_split_word(w) for w in ws)
    return chain.from_iterable(_split_pascal(w) for w in ws)


assert list(_split_pascal("AbcDef")) == ["Abc", "Def"]
assert list(_into_words(["A-b-c"])) == ["A", "b", "c"]
assert list(_into_words(["A", "b-c"])) == ["A", "b", "c"]
assert list(_into_words(["AbcDef-ghi"])) == ["Abc", "Def", "ghi"]


def pascal_ident(*ws: str) -> str:
    return "".join(w.capitalize() for w in _into_words(ws))


def snake_ident(*ws: str) -> str:
    return "_".join(w.lower() for w in _into_words(ws))


def sanitize_path(path: str) -> str:
    def is_allowed(c: str) -> bool:
        if "a" <= c <= "z":
            return True

        if "0" <= c <= "9":
            return True

        if c in {"-", "_", ".", "/"}:
            return True

        return False

    return "".join([c for c in path if is_allowed(c)]).replace("-", "_")
