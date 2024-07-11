from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig
from . import changelog as _changelog
from . import repositories as _repositories
from . import update as _update
from . import versions as _versions


class GetResponseItem(BaseModel):
    id: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class AptClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'apt'}"

    def update(self) -> _update.UpdateClient:
        return _update.UpdateClient(
            self.client,
            self.path,
        )

    def changelog(self) -> _changelog.ChangelogClient:
        return _changelog.ChangelogClient(
            self.client,
            self.path,
        )

    def repositories(self) -> _repositories.RepositoriesClient:
        return _repositories.RepositoriesClient(
            self.client,
            self.path,
        )

    def versions(self) -> _versions.VersionsClient:
        return _versions.VersionsClient(
            self.client,
            self.path,
        )

    def get(self) -> list[GetResponseItem]:
        """
        Directory index for apt (Advanced Package Tool).
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncAptClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'apt'}"

    def update(self) -> _update.AsyncUpdateClient:
        return _update.AsyncUpdateClient(
            self.client,
            self.path,
        )

    def changelog(self) -> _changelog.AsyncChangelogClient:
        return _changelog.AsyncChangelogClient(
            self.client,
            self.path,
        )

    def repositories(self) -> _repositories.AsyncRepositoriesClient:
        return _repositories.AsyncRepositoriesClient(
            self.client,
            self.path,
        )

    def versions(self) -> _versions.AsyncVersionsClient:
        return _versions.AsyncVersionsClient(
            self.client,
            self.path,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        Directory index for apt (Advanced Package Tool).
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
