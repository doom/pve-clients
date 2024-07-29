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
    # Digest to detect modifications.
    digest: Optional[str] = Field(default=None)
    # Handle that identifies a repository.
    handle: str

    class Config(CommonPydanticConfig):
        pass


class PostParameters(BaseModel):
    # Digest to detect modifications.
    digest: Optional[str] = Field(default=None)
    # Whether the repository should be enabled or not.
    enabled: Optional[bool] = Field(default=None)
    # Index within the file (starting from 0).
    index: int
    # Path to the containing file.
    path: str

    class Config(CommonPydanticConfig):
        pass


class GetResponseStandardReposItem(BaseModel):
    # Handle to identify the repository.
    handle: str
    # Full name of the repository.
    name: str
    # Indicating enabled/disabled status, if the repository is configured.
    status: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseInfosItem(BaseModel):
    # Index of the associated repository within the file.
    index: str
    # Kind of the information (e.g. warning).
    kind: str
    # Information message.
    message: str
    # Path to the associated file.
    path: str
    # Property from which the info originates.
    property: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseFilesItemRepositoriesItemOptionsItem(BaseModel):
    key: str = Field(alias="Key")
    values: list[str] = Field(alias="Values")

    class Config(CommonPydanticConfig):
        pass


class GetResponseFilesItemRepositoriesItem(BaseModel):
    # Associated comment
    comment: Optional[str] = Field(alias="Comment", default=None)
    # List of repository components
    components: Optional[list[str]] = Field(alias="Components", default=None)
    # Whether the repository is enabled or not
    enabled: bool = Field(alias="Enabled")
    # Format of the defining file.
    file_type: str = Field(alias="FileType")
    # Additional options
    options: Optional[list[GetResponseFilesItemRepositoriesItemOptionsItem]] = Field(
        alias="Options", default=None
    )
    # List of package distribuitions
    suites: list[str] = Field(alias="Suites")
    # List of package types.
    types: list[str] = Field(alias="Types")
    # List of repository URIs.
    uris: list[str] = Field(alias="URIs")

    class Config(CommonPydanticConfig):
        pass


class GetResponseFilesItem(BaseModel):
    # Digest of the file as bytes.
    digest: list[int]
    # Format of the file.
    file_type: str = Field(alias="file-type")
    # Path to the problematic file.
    path: str
    # The parsed repositories.
    repositories: list[GetResponseFilesItemRepositoriesItem]

    class Config(CommonPydanticConfig):
        pass


class GetResponseErrorsItem(BaseModel):
    # The error message
    error: str
    # Path to the problematic file.
    path: str

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    """
    Result from parsing the APT repository files in /etc/apt/.
    """

    # Common digest of all files.
    digest: str
    # List of problematic repository files.
    errors: list[GetResponseErrorsItem]
    # List of parsed repository files.
    files: list[GetResponseFilesItem]
    # Additional information/warnings for APT repositories.
    infos: list[GetResponseInfosItem]
    # List of standard repositories and their configuration status
    standard_repos: list[GetResponseStandardReposItem] = Field(alias="standard-repos")

    class Config(CommonPydanticConfig):
        pass


@dataclass
class RepositoriesClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'repositories'}"

    def get(self) -> GetResponseItem:
        """
        Get APT repository information.
        """
        return self.client.get(self.path, parse_as=GetResponseItem)

    def post(self, parameters: PostParameters):
        """
        Change the properties of a repository. Currently only allows enabling/disabling.
        """
        return self.client.post(self.path, parameters)

    def put(self, parameters: PutParameters):
        """
        Add a standard repository to the configuration
        """
        return self.client.put(self.path, parameters)


@dataclass
class AsyncRepositoriesClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'repositories'}"

    async def get(self) -> GetResponseItem:
        """
        Get APT repository information.
        """
        return await self.client.get(self.path, parse_as=GetResponseItem)

    async def post(self, parameters: PostParameters):
        """
        Change the properties of a repository. Currently only allows enabling/disabling.
        """
        return await self.client.post(self.path, parameters)

    async def put(self, parameters: PutParameters):
        """
        Add a standard repository to the configuration
        """
        return await self.client.put(self.path, parameters)
