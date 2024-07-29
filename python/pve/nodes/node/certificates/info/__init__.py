from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)


class GetResponseItem(BaseModel):
    filename: Optional[str] = Field(default=None)
    # Certificate SHA 256 fingerprint.
    fingerprint: Optional[str] = Field(default=None)
    # Certificate issuer name.
    issuer: Optional[str] = Field(default=None)
    # Certificate's notAfter timestamp (UNIX epoch).
    notafter: Optional[int] = Field(default=None)
    # Certificate's notBefore timestamp (UNIX epoch).
    notbefore: Optional[int] = Field(default=None)
    # Certificate in PEM format
    pem: Optional[str] = Field(default=None)
    # Certificate's public key size
    public_key_bits: Optional[int] = Field(alias="public-key-bits", default=None)
    # Certificate's public key algorithm
    public_key_type: Optional[str] = Field(alias="public-key-type", default=None)
    # List of Certificate's SubjectAlternativeName entries.
    san: Optional[list[str]] = Field(default=None)
    # Certificate subject name.
    subject: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class InfoClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'info'}"

    def get(self) -> list[GetResponseItem]:
        """
        Get information about node's certificates.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncInfoClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'info'}"

    async def get(self) -> list[GetResponseItem]:
        """
        Get information about node's certificates.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
