from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class PutParameters(BaseModel):
    # Force renewal even if expiry is more than 30 days away.
    force: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class PostParameters(BaseModel):
    # Overwrite existing custom certificate.
    force: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class CertificateClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'certificate'}"

    def delete(self) -> str:
        """
        Revoke existing certificate from CA.
        """
        return self.client.delete(self.path, parse_as=str)

    def post(self, parameters: PostParameters) -> str:
        """
        Order a new certificate from ACME-compatible CA.
        """
        return self.client.post(self.path, parameters, parse_as=str)

    def put(self, parameters: PutParameters) -> str:
        """
        Renew existing certificate from CA.
        """
        return self.client.put(self.path, parameters, parse_as=str)


@dataclass
class AsyncCertificateClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'certificate'}"

    async def delete(self) -> str:
        """
        Revoke existing certificate from CA.
        """
        return await self.client.delete(self.path, parse_as=str)

    async def post(self, parameters: PostParameters) -> str:
        """
        Order a new certificate from ACME-compatible CA.
        """
        return await self.client.post(self.path, parameters, parse_as=str)

    async def put(self, parameters: PutParameters) -> str:
        """
        Renew existing certificate from CA.
        """
        return await self.client.put(self.path, parameters, parse_as=str)
