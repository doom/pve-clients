from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import certificate as _certificate


class GetResponseItem(BaseModel):
    pass


@dataclass
class AcmeClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'acme'}"

    def certificate(self) -> _certificate.CertificateClient:
        return _certificate.CertificateClient(
            self.client,
            self.path,
        )

    def get(self) -> list[GetResponseItem]:
        """
        ACME index.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncAcmeClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'acme'}"

    def certificate(self) -> _certificate.AsyncCertificateClient:
        return _certificate.AsyncCertificateClient(
            self.client,
            self.path,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        ACME index.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
