from typing import Any, Optional


def _encode_bool_as_int(value: bool) -> int:
    return int(value)


class CommonPydanticConfig:
    """
    Common Pydantic Config class used by all models
    """

    json_encoders = {
        bool: _encode_bool_as_int,
    }


def extract_repeated_with_prefix(
    data: dict[str, Any], group: str, prefix: str
) -> dict[str, Any]:
    extracted = {}
    for k, v in list(data.items()):
        if not k.startswith(prefix):
            continue
        try:
            n = int(k.removeprefix(prefix))
        except ValueError:
            continue
        extracted[n] = v
        del data[k]

    if extracted:
        data[group] = extracted

    return data


def serialize_repeated_with_prefix(
    data: dict[str, Any], group: str, prefix: str
) -> dict[str, Any]:
    repeated: dict[int, Any] | None = data.pop(group, None)
    if repeated is None:
        return data

    for k, v in repeated.items():
        data[f"{prefix}{k}"] = v
    return data
