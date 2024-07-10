def _encode_bool_as_int(value: bool) -> int:
    return int(value)


class CommonPydanticConfig:
    """
    Common Pydantic Config class used by all models
    """

    json_encoders = {
        bool: _encode_bool_as_int,
    }
