from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)


class PutParameters(BaseModel):
    # A list of settings you want to delete.
    delete: Optional[str] = Field(default=None)
    # Prevent changes if current configuration file has different SHA1 digest. This can be used to prevent concurrent modifications.
    digest: Optional[str] = Field(default=None)
    # Enable ebtables rules cluster wide.
    ebtables: Optional[bool] = Field(default=None)
    # Enable or disable the firewall cluster wide.
    enable: Optional[int] = Field(default=None)
    # Log ratelimiting settings
    log_ratelimit: Optional[str] = Field(default=None)
    # Input policy.
    policy_in: Optional[str] = Field(default=None)
    # Output policy.
    policy_out: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    # Enable ebtables rules cluster wide.
    ebtables: Optional[bool] = Field(default=None)
    # Enable or disable the firewall cluster wide.
    enable: Optional[int] = Field(default=None)
    # Log ratelimiting settings
    log_ratelimit: Optional[str] = Field(default=None)
    # Input policy.
    policy_in: Optional[str] = Field(default=None)
    # Output policy.
    policy_out: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class OptionsClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'options'}"

    def get(self) -> GetResponseItem:
        """
        Get Firewall options.
        """
        return self.client.get(self.path, parse_as=GetResponseItem)

    def put(self, parameters: PutParameters):
        """
        Set Firewall options.
        """
        return self.client.put(self.path, parameters)


@dataclass
class AsyncOptionsClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'options'}"

    async def get(self) -> GetResponseItem:
        """
        Get Firewall options.
        """
        return await self.client.get(self.path, parse_as=GetResponseItem)

    async def put(self, parameters: PutParameters):
        """
        Set Firewall options.
        """
        return await self.client.put(self.path, parameters)
