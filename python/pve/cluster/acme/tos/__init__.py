from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)


class GetParameters(BaseModel):
    # URL of ACME CA directory endpoint.
    directory: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class TosClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'tos'}"

    def get(self, parameters: GetParameters) -> Optional[str]:
        """
        Retrieve ACME TermsOfService URL from CA. Deprecated, please use /cluster/acme/meta.
        """
        return self.client.get(self.path, parameters, parse_as=Optional[str])


@dataclass
class AsyncTosClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'tos'}"

    async def get(self, parameters: GetParameters) -> Optional[str]:
        """
        Retrieve ACME TermsOfService URL from CA. Deprecated, please use /cluster/acme/meta.
        """
        return await self.client.get(self.path, parameters, parse_as=Optional[str])
