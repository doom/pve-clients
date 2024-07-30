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
    # Hostnames referring to the ACME servers.
    caa_identities: Optional[list[str]] = Field(alias="caaIdentities", default=None)
    # EAB Required
    external_account_required: Optional[bool] = Field(
        alias="externalAccountRequired", default=None
    )
    # ACME TermsOfService URL.
    terms_of_service: Optional[str] = Field(alias="termsOfService", default=None)
    # URL to more information about the ACME server.
    website: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    # URL of ACME CA directory endpoint.
    directory: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class MetaClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'meta'}"

    def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Retrieve ACME Directory Meta Information
        """
        return self.client.get(self.path, parameters, parse_as=GetResponseItem)


@dataclass
class AsyncMetaClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'meta'}"

    async def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Retrieve ACME Directory Meta Information
        """
        return await self.client.get(self.path, parameters, parse_as=GetResponseItem)
