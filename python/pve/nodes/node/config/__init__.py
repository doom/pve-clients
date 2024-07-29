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
    # Node specific ACME settings.
    acme: Optional[str] = Field(default=None)
    # ACME domain and validation plugin
    acmedomains: dict[int, Optional[str]] = Field(default=None)
    # A list of settings you want to delete.
    delete: Optional[str] = Field(default=None)
    # Description for the Node. Shown in the web-interface node notes panel. This is saved as comment inside the configuration file.
    description: Optional[str] = Field(default=None)
    # Prevent changes if current configuration file has different SHA1 digest. This can be used to prevent concurrent modifications.
    digest: Optional[str] = Field(default=None)
    # Initial delay in seconds, before starting all the Virtual Guests with on-boot enabled.
    startall_onboot_delay: Optional[int] = Field(
        alias="startall-onboot-delay", default=None
    )
    # MAC address for wake on LAN
    wakeonlan: Optional[str] = Field(default=None)

    @model_serializer(mode="wrap")
    def _serialize_repeated(self, serializer):
        data = serializer(self)
        data = serialize_repeated_with_prefix(
            data, group="acmedomains", prefix="acmedomain"
        )
        return data

    @model_validator(mode="before")
    @classmethod
    def _extract_repeated(cls, data: Any) -> Any:
        if not isinstance(data, dict):
            return data
        data = data = extract_repeated_with_prefix(
            data, group="acmedomains", prefix="acmedomain"
        )
        return data

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    # Node specific ACME settings.
    acme: Optional[str] = Field(default=None)
    # ACME domain and validation plugin
    acmedomains: dict[int, Optional[str]] = Field(default=None)
    # Description for the Node. Shown in the web-interface node notes panel. This is saved as comment inside the configuration file.
    description: Optional[str] = Field(default=None)
    # Prevent changes if current configuration file has different SHA1 digest. This can be used to prevent concurrent modifications.
    digest: Optional[str] = Field(default=None)
    # Initial delay in seconds, before starting all the Virtual Guests with on-boot enabled.
    startall_onboot_delay: Optional[int] = Field(
        alias="startall-onboot-delay", default=None
    )
    # MAC address for wake on LAN
    wakeonlan: Optional[str] = Field(default=None)

    @model_serializer(mode="wrap")
    def _serialize_repeated(self, serializer):
        data = serializer(self)
        data = serialize_repeated_with_prefix(
            data, group="acmedomains", prefix="acmedomain"
        )
        return data

    @model_validator(mode="before")
    @classmethod
    def _extract_repeated(cls, data: Any) -> Any:
        if not isinstance(data, dict):
            return data
        data = data = extract_repeated_with_prefix(
            data, group="acmedomains", prefix="acmedomain"
        )
        return data

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    # Return only a specific property from the node configuration.
    property: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class ConfigClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'config'}"

    def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Get node configuration options.
        """
        return self.client.get(self.path, parameters, parse_as=GetResponseItem)

    def put(self, parameters: PutParameters):
        """
        Set node configuration options.
        """
        return self.client.put(self.path, parameters)


@dataclass
class AsyncConfigClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'config'}"

    async def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Get node configuration options.
        """
        return await self.client.get(self.path, parameters, parse_as=GetResponseItem)

    async def put(self, parameters: PutParameters):
        """
        Set node configuration options.
        """
        return await self.client.put(self.path, parameters)
