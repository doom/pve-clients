from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import in_ as _in_
from . import lv_info as _lv_info
from . import metadata as _metadata
from . import out as _out
from . import scrub as _scrub


class GetResponseItem(BaseModel):
    pass


class DeleteParameters(BaseModel):
    # If set, we remove partition table entries.
    cleanup: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class OsdidClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str, osdid: str):
        self.client = client
        self.path = f"{parent_path}/{osdid}"

    def metadata(self) -> _metadata.MetadataClient:
        return _metadata.MetadataClient(
            self.client,
            self.path,
        )

    def lv_info(self) -> _lv_info.LvInfoClient:
        return _lv_info.LvInfoClient(
            self.client,
            self.path,
        )

    def in_(self) -> _in_.InClient:
        return _in_.InClient(
            self.client,
            self.path,
        )

    def out(self) -> _out.OutClient:
        return _out.OutClient(
            self.client,
            self.path,
        )

    def scrub(self) -> _scrub.ScrubClient:
        return _scrub.ScrubClient(
            self.client,
            self.path,
        )

    def delete(self, parameters: DeleteParameters) -> str:
        """
        Destroy OSD
        """
        return self.client.delete(self.path, parameters, parse_as=str)

    def get(self) -> list[GetResponseItem]:
        """
        OSD index.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncOsdidClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, osdid: str):
        self.client = client
        self.path = f"{parent_path}/{osdid}"

    def metadata(self) -> _metadata.AsyncMetadataClient:
        return _metadata.AsyncMetadataClient(
            self.client,
            self.path,
        )

    def lv_info(self) -> _lv_info.AsyncLvInfoClient:
        return _lv_info.AsyncLvInfoClient(
            self.client,
            self.path,
        )

    def in_(self) -> _in_.AsyncInClient:
        return _in_.AsyncInClient(
            self.client,
            self.path,
        )

    def out(self) -> _out.AsyncOutClient:
        return _out.AsyncOutClient(
            self.client,
            self.path,
        )

    def scrub(self) -> _scrub.AsyncScrubClient:
        return _scrub.AsyncScrubClient(
            self.client,
            self.path,
        )

    async def delete(self, parameters: DeleteParameters) -> str:
        """
        Destroy OSD
        """
        return await self.client.delete(self.path, parameters, parse_as=str)

    async def get(self) -> list[GetResponseItem]:
        """
        OSD index.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
