from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig
from . import acme as _acme
from . import custom as _custom
from . import info as _info


class GetResponseItem(BaseModel):
    pass


@dataclass
class CertificatesClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'certificates'}"

    def acme(self) -> _acme.AcmeClient:
        return _acme.AcmeClient(
            self.client,
            self.path,
        )

    def info(self) -> _info.InfoClient:
        return _info.InfoClient(
            self.client,
            self.path,
        )

    def custom(self) -> _custom.CustomClient:
        return _custom.CustomClient(
            self.client,
            self.path,
        )

    def get(self) -> list[GetResponseItem]:
        """
        Node index.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncCertificatesClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'certificates'}"

    def acme(self) -> _acme.AsyncAcmeClient:
        return _acme.AsyncAcmeClient(
            self.client,
            self.path,
        )

    def info(self) -> _info.AsyncInfoClient:
        return _info.AsyncInfoClient(
            self.client,
            self.path,
        )

    def custom(self) -> _custom.AsyncCustomClient:
        return _custom.AsyncCustomClient(
            self.client,
            self.path,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        Node index.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
