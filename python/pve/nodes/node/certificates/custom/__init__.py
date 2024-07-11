from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class PostResponseItem(BaseModel):
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


class PostParameters(BaseModel):
    # PEM encoded certificate (chain).
    certificates: str
    # Overwrite existing custom or ACME certificate files.
    force: Optional[bool] = Field(default=None)
    # PEM encoded private key.
    key: Optional[str] = Field(default=None)
    # Restart pveproxy.
    restart: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class DeleteParameters(BaseModel):
    # Restart pveproxy.
    restart: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class CustomClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'custom'}"

    def delete(self, parameters: DeleteParameters):
        """
        DELETE custom certificate chain and key.
        """
        return self.client.delete(self.path, parameters)

    def post(self, parameters: PostParameters) -> PostResponseItem:
        """
        Upload or update custom certificate chain and key.
        """
        return self.client.post(self.path, parameters, parse_as=PostResponseItem)


@dataclass
class AsyncCustomClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'custom'}"

    async def delete(self, parameters: DeleteParameters):
        """
        DELETE custom certificate chain and key.
        """
        return await self.client.delete(self.path, parameters)

    async def post(self, parameters: PostParameters) -> PostResponseItem:
        """
        Upload or update custom certificate chain and key.
        """
        return await self.client.post(self.path, parameters, parse_as=PostResponseItem)
