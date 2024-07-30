from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import endpoints as _endpoints
from . import matchers as _matchers
from . import targets as _targets


class GetResponseItem(BaseModel):
    pass


@dataclass
class NotificationsClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'notifications'}"

    def endpoints(self) -> _endpoints.EndpointsClient:
        return _endpoints.EndpointsClient(
            self.client,
            self.path,
        )

    def targets(self) -> _targets.TargetsClient:
        return _targets.TargetsClient(
            self.client,
            self.path,
        )

    def matchers(self) -> _matchers.MatchersClient:
        return _matchers.MatchersClient(
            self.client,
            self.path,
        )

    def get(self) -> list[GetResponseItem]:
        """
        Index for notification-related API endpoints.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncNotificationsClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'notifications'}"

    def endpoints(self) -> _endpoints.AsyncEndpointsClient:
        return _endpoints.AsyncEndpointsClient(
            self.client,
            self.path,
        )

    def targets(self) -> _targets.AsyncTargetsClient:
        return _targets.AsyncTargetsClient(
            self.client,
            self.path,
        )

    def matchers(self) -> _matchers.AsyncMatchersClient:
        return _matchers.AsyncMatchersClient(
            self.client,
            self.path,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        Index for notification-related API endpoints.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
