from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import account as _account
from . import challenge_schema as _challenge_schema
from . import directories as _directories
from . import plugins as _plugins
from . import tos as _tos


class GetResponseItem(BaseModel):
    pass


@dataclass
class AcmeClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'acme'}"

    def plugins(self) -> _plugins.PluginsClient:
        return _plugins.PluginsClient(
            self.client,
            self.path,
        )

    def account(self) -> _account.AccountClient:
        return _account.AccountClient(
            self.client,
            self.path,
        )

    def tos(self) -> _tos.TosClient:
        return _tos.TosClient(
            self.client,
            self.path,
        )

    def directories(self) -> _directories.DirectoriesClient:
        return _directories.DirectoriesClient(
            self.client,
            self.path,
        )

    def challenge_schema(self) -> _challenge_schema.ChallengeSchemaClient:
        return _challenge_schema.ChallengeSchemaClient(
            self.client,
            self.path,
        )

    def get(self) -> list[GetResponseItem]:
        """
        ACMEAccount index.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncAcmeClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'acme'}"

    def plugins(self) -> _plugins.AsyncPluginsClient:
        return _plugins.AsyncPluginsClient(
            self.client,
            self.path,
        )

    def account(self) -> _account.AsyncAccountClient:
        return _account.AsyncAccountClient(
            self.client,
            self.path,
        )

    def tos(self) -> _tos.AsyncTosClient:
        return _tos.AsyncTosClient(
            self.client,
            self.path,
        )

    def directories(self) -> _directories.AsyncDirectoriesClient:
        return _directories.AsyncDirectoriesClient(
            self.client,
            self.path,
        )

    def challenge_schema(self) -> _challenge_schema.AsyncChallengeSchemaClient:
        return _challenge_schema.AsyncChallengeSchemaClient(
            self.client,
            self.path,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        ACMEAccount index.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
