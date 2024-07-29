from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)


class GetResponseItem(BaseModel):
    # Creation time as reported by `lvs`.
    creation_time: str
    # Name of the logical volume (LV).
    lv_name: str
    # Path to the logical volume (LV).
    lv_path: str
    # Size of the logical volume (LV).
    lv_size: int
    # UUID of the logical volume (LV).
    lv_uuid: str
    # Name of the volume group (VG).
    vg_name: str

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    # OSD device type
    type: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class LvInfoClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'lv-info'}"

    def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Get OSD volume details
        """
        return self.client.get(self.path, parameters, parse_as=GetResponseItem)


@dataclass
class AsyncLvInfoClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'lv-info'}"

    async def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Get OSD volume details
        """
        return await self.client.get(self.path, parameters, parse_as=GetResponseItem)
